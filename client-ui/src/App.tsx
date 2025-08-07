import "./App.css"
import { useAuth } from "./AuthContext.tsx"

function App() {
  const { user, logout } = useAuth()

  return (
    <>
      <h1>Welcome to -, {user.name}</h1>
      <button onClick={() => logout()}>Logout</button>
    </>
  )
}

export default App
