import { useAuth } from "./AuthContext.tsx"
import { useEffect, useState } from "react"
import { getServer } from "./interface.ts"

function App() {
  const { user, logout } = useAuth()

  const [server, setServer] = useState<string>("-")

  useEffect(() => {
    getServer().then((s) => setServer(s ?? "-"))
  }, [])

  return (
    <>
      <h1>
        Welcome to {server}, {user.name}.
      </h1>
      <button className="btn btn-outline-danger" onClick={() => logout()}>
        Logout
      </button>
      <button className="btn btn-outline-dark">Whitelist</button>
      <button className="btn btn-outline-dark">Send Email</button>
    </>
  )
}

export default App
