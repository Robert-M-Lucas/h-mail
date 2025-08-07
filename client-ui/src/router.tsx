import { createBrowserRouter } from "react-router-dom"
import ErrorPage from "./error/ErrorPage.tsx"
import App from "./App.tsx"

export const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    errorElement: <ErrorPage />,
  },
])
