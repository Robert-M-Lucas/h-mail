import React from "react"
import ReactDOM from "react-dom/client"
import { AuthProvider } from "./contexts/AuthContext.tsx"
import { RouterProvider } from "react-router-dom"
import { router } from "./router.tsx"
import PowProgress from "./components/PowProgress.tsx"
import "bootstrap/dist/css/bootstrap.min.css"
import { ToastProvider } from "./contexts/ToastContext.tsx"
import { EstimateProvider } from "./contexts/EstimateContext.tsx"
import { LockoutProvider } from "./contexts/LockoutContext.tsx"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <PowProgress>
      <ToastProvider>
        <EstimateProvider>
          <LockoutProvider>
            <AuthProvider>
              <RouterProvider router={router} />
            </AuthProvider>
          </LockoutProvider>
        </EstimateProvider>
      </ToastProvider>
    </PowProgress>
  </React.StrictMode>
)
