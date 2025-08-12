import { useAuth } from "./AuthContext.tsx"
import "bootstrap/dist/css/bootstrap.min.css"

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
