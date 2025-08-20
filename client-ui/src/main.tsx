import React from "react"
import ReactDOM from "react-dom/client"
import { AuthProvider } from "./contexts/AuthContext.tsx"
import { RouterProvider } from "react-router-dom"
import { router } from "./router.tsx"
import PowProgress from "./components/PowProgress.tsx"
import "bootstrap/dist/css/bootstrap.min.css"
import { ToastProvider } from "./contexts/ToastContext.tsx"
import { EstimateProvider } from "./contexts/EstimateProvider.tsx"
import { LockoutProvider } from "./contexts/LockoutProvider.tsx"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <PowProgress>
      <EstimateProvider>
        <ToastProvider>
          <LockoutProvider>
            <AuthProvider>
              <RouterProvider router={router} />
            </AuthProvider>
          </LockoutProvider>
        </ToastProvider>
      </EstimateProvider>
    </PowProgress>
  </React.StrictMode>
)
