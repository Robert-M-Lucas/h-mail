import React, {
  createContext,
  useContext,
  useState,
  ReactNode,
  useEffect,
} from "react"
import {
  checkAlive,
  checkAuth,
  createAccount,
  createAccountRequirement,
  getServer,
  reauthenticate,
  setServer,
} from "../interface.ts"
import { invoke } from "@tauri-apps/api/core"
import FullscreenCenter from "../components/FullscreenCenter.tsx"
import {
  Button,
  ButtonGroup,
  Card,
  Form,
  InputGroup,
  Modal,
} from "react-bootstrap"
import { useToast } from "./ToastContext.tsx"
import { useEstimate } from "./EstimateContext.tsx"
import { useLockout } from "./LockoutContext.tsx"

export type AuthInfo = {
  name: string
  domain: string
}

interface AuthContextType {
  user: AuthInfo
  logout: () => void
}

function msToTime(duration: number): string {
  const seconds = Math.floor((duration / 1000) % 60)
  const minutes = Math.floor((duration / (1000 * 60)) % 60)
  const hours = Math.floor(duration / (1000 * 60 * 60))

  const hh = String(hours).padStart(2, "0")
  const mm = String(minutes).padStart(2, "0")
  const ss = String(seconds).padStart(2, "0")

  return `${hh}:${mm}:${ss}`
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

interface AuthProviderProps {
  children: ReactNode
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<AuthInfo | null>(null)
  const [serverVal, setServerVal] = useState<string>("")
  const [username, setUsername] = useState<string>("")
  const [password, setPassword] = useState<string>("")
  const [showCreateAccountConfirmation, setShowCreateAccountConfirmation] =
    useState(false)
  const [createAccountEstimate, setCreateAccountEstimate] = useState<
    number | undefined
  >(undefined)
  const { showToast } = useToast()
  const estimate = useEstimate()
  const { enterLockout, exitLockout } = useLockout()

  useEffect(() => {
    enterLockout()
    getServer().then(async (server) => {
      exitLockout()
      if (server) {
        enterLockout()
        setServerVal(server)

        checkAuth().then((user) => {
          if (user) setUser({ name: user, domain: server })
          exitLockout()
        })
      }
    })
  }, [])

  if (user) {
    const logout = async () => {
      enterLockout()
      await invoke("logout")
      setUser(null)
      exitLockout()
    }

    return (
      <AuthContext.Provider value={{ user: user!, logout }}>
        {children}
      </AuthContext.Provider>
    )
  } else {
    const login = async () => {
      const e = await setServer(serverVal)
      if (e) {
        showToast({
          header: "Login Failure",
          body: `Bad address: ${e}`,
        })
        return
      }

      enterLockout()
      const result = await reauthenticate(username, password)
      if (result.ok) {
        setUser({ name: result.value, domain: serverVal })
      } else {
        showToast({
          header: "Login Failure",
          body: result.error,
        })
      }
      exitLockout()
    }

    const createAccountF = async () => {
      const e = await setServer(serverVal)
      if (e) {
        showToast({
          header: "Create Account Failure",
          body: `Bad address: ${e}`,
        })
        return
      }

      closeCreateAccountModal()
      enterLockout()
      const result = await createAccount(username, password)
      if (result.ok) {
        setUser({ name: result.value, domain: serverVal })
      } else {
        showToast({
          header: "Create Account Failure",
          body: result.error,
        })
      }
      exitLockout()
    }

    const showCreateAccountModal = async () => {
      const e = await setServer(serverVal)
      if (e) {
        showToast({
          header: "Create Account Failure",
          body: `Bad address: ${e}`,
        })
        return
      }

      enterLockout()
      setCreateAccountEstimate(undefined)
      const result = await createAccountRequirement()
      if (result.ok) {
        const requirement = result.value
        setCreateAccountEstimate((requirement / estimate) * 1000)
        setShowCreateAccountConfirmation(true)
      } else {
        setShowCreateAccountConfirmation(false)
        showToast({
          header: "Failed to Contact Server",
          body: result.error,
        })
      }
      exitLockout()
    }

    const closeCreateAccountModal = () => {
      setShowCreateAccountConfirmation(false)
      setCreateAccountEstimate(undefined)
    }

    const checkAliveF = async () => {
      const e = await setServer(serverVal)
      if (e) {
        showToast({
          header: "Server Alive Check",
          body: `Bad address: ${e}`,
        })
        return
      }

      enterLockout()
      try {
        const aliveResult = await checkAlive()
        showToast({
          header: "Server Alive Check",
          body: `Server status: ${aliveResult}`,
        })
      } catch (e) {
        showToast({
          header: "Server Alive Check",
          body: `Server status: ${e}`,
        })
      }

      exitLockout()
    }

    return (
      <FullscreenCenter insetShadow>
        <Modal
          centered
          show={showCreateAccountConfirmation}
          onHide={closeCreateAccountModal}
        >
          <Modal.Header closeButton>
            <Modal.Title>Create Account</Modal.Title>
          </Modal.Header>

          {createAccountEstimate ? (
            <Modal.Body>
              Creating an account is estimated to take{" "}
              {msToTime(createAccountEstimate)}.
            </Modal.Body>
          ) : (
            <Modal.Body className="text-muted">
              Estimating time to create an account...
            </Modal.Body>
          )}

          <Modal.Footer>
            {createAccountEstimate && (
              <Button
                variant={"outline-success"}
                className={"w-100"}
                onClick={createAccountF}
              >
                Create
              </Button>
            )}
          </Modal.Footer>
        </Modal>

        <Card>
          <Card.Body>
            <Card.Title className={"display-6"}>Log In</Card.Title>
            <Form>
              <Form.Group className="mb-3">
                <Form.Label>Server</Form.Label>
                <InputGroup>
                  <Form.Control
                    placeholder={"Server"}
                    onChange={(e) => setServerVal(e.currentTarget.value)}
                    value={serverVal}
                  ></Form.Control>
                  <Button
                    variant={"outline-warning"}
                    onClick={() => checkAliveF()}
                  >
                    Check Alive
                  </Button>
                </InputGroup>
              </Form.Group>

              <Form.Group className="mb-3">
                <Form.Label>Username</Form.Label>
                <Form.Control
                  placeholder={"Username"}
                  onChange={(e) => setUsername(e.currentTarget.value)}
                  value={username}
                ></Form.Control>
              </Form.Group>

              <Form.Group className="mb-3">
                <Form.Label>Password</Form.Label>
                <Form.Control
                  placeholder="Password"
                  type={"password"}
                  onChange={(e) => setPassword(e.currentTarget.value)}
                  value={password}
                ></Form.Control>
              </Form.Group>

              <ButtonGroup className={"w-100"}>
                <Button
                  className={"w-50"}
                  variant="outline-primary"
                  onClick={() => login().then(() => {})}
                >
                  Login
                </Button>
                <Button
                  className={"w-50"}
                  variant="outline-success"
                  onClick={showCreateAccountModal}
                >
                  Create Account
                </Button>
              </ButtonGroup>
            </Form>
          </Card.Body>
        </Card>
      </FullscreenCenter>
    )
  }
}

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext)
  if (!context) {
    throw new Error("useAuth must be used within a AuthProvider")
  }
  return context
}
