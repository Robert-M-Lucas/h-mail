import React from "react"
import ReactDOM from "react-dom/client"
import { AuthProvider } from "./AuthContext.tsx"
import { RouterProvider } from "react-router-dom"
import { router } from "./router.tsx"
import PowProgress from "./components/PowProgress.tsx"
import "bootstrap/dist/css/bootstrap.min.css"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <PowProgress>
      <AuthProvider>
        <RouterProvider router={router} />
      </AuthProvider>
    </PowProgress>
  </React.StrictMode>
)
