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
} from "./interface.ts"
import { invoke } from "@tauri-apps/api/core"

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
      <>
        <h1>Log In</h1>
        <p>Server:</p>
        <input
          onChange={(e) => setServerVal(e.currentTarget.value)}
          value={serverVal}
        ></input>
        <button
          className="btn btn-outline-warning"
          onClick={() => checkAliveF()}
        >
          Check Alive
        </button>
        <p>Username:</p>
        <input onChange={(e) => setUsername(e.currentTarget.value)}></input>
        <p>Password:</p>
        <input onChange={(e) => setPassword(e.currentTarget.value)}></input>
        <button
          className="btn btn-outline-primary"
          onClick={() => login().then(() => {})}
        >
          Login
        </button>
        <button
          className="btn btn-outline-success"
          onClick={() => createAccountF().then(() => {})}
        >
          Create Account
        </button>
        <p>{error}</p>
      </>
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
