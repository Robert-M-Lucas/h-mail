import { useAuth } from "../AuthContext.tsx"
import { useEffect, useState } from "react"
import { getEmails, getServer } from "../interface.ts"
import { useNavigate } from "react-router-dom"

function App() {
  const { user, logout } = useAuth()

  const [server, setServer] = useState<string>("-")

  useEffect(() => {
    getServer().then((s) => setServer(s ?? "-"))

    getEmails(logout).then()
  }, [])

  const navigate = useNavigate()

  return (
    <>
      <h1>
        Welcome to {server}, {user.name}.
      </h1>
      <button className="btn btn-outline-danger" onClick={() => logout()}>
        Logout
      </button>
      <button
        className="btn btn-outline-dark"
        onClick={() => navigate("/whitelist")}
      >
        Whitelist
      </button>
      <button className="btn btn-outline-primary">Send Email</button>
    </>
  )
}

export default App
