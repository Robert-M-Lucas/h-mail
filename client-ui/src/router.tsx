import { createBrowserRouter } from "react-router-dom"
import ErrorPage from "./error/ErrorPage.tsx"
import App from "./routes/App.tsx"

export const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    errorElement: <ErrorPage />,
  },
])
