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
  getServer,
  reauthenticate,
  setServer,
} from "../interface.ts"
import { invoke } from "@tauri-apps/api/core"
import FullscreenCenter from "../components/FullscreenCenter.tsx"
import { Button, ButtonGroup, Card, Form, InputGroup } from "react-bootstrap"

type AuthInfo = {
  name: string
  domain: string
}

interface AuthContextType {
  user: AuthInfo
  logout: () => void
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
  const [error, setError] = useState<string>("")

  useEffect(() => {
    getServer().then(async (server) => {
      if (server) {
        setServerVal(server)

        checkAuth().then((user) => {
          if (user) setUser({ name: user, domain: server })
        })
      }
    })
  }, [])

  if (user) {
    const logout = async () => {
      await invoke("logout")
      setUser(null)
    }

    return (
      <AuthContext.Provider value={{ user: user!, logout }}>
        {children}
      </AuthContext.Provider>
    )
  } else {
    const login = async () => {
      await setServer(serverVal)
      const result = await reauthenticate(username, password)
      if (result.ok) {
        setUser({ name: result.value, domain: serverVal })
      } else {
        setError(result.error)
      }
    }

    const createAccountF = async () => {
      await setServer(serverVal)
      setError("Creating account...")
      const result = await createAccount(username, password)
      if (result.ok) {
        setUser({ name: result.value, domain: serverVal })
      } else {
        setError(result.error)
      }
    }

    const checkAliveF = async () => {
      await setServer(serverVal)
      if (await checkAlive()) {
        setError("Server Alive")
      } else {
        setError("Server Not Alive")
      }
    }

    return (
      <FullscreenCenter insetShadow>
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
                  onClick={() => createAccountF().then(() => {})}
                >
                  Create Account
                </Button>
              </ButtonGroup>
            </Form>
            <p className={"text-danger"}>{error}</p>
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
