import { createBrowserRouter } from "react-router-dom"
import ErrorPage from "./error/ErrorPage.tsx"
import App from "./routes/App.tsx"
import Whitelist from "./routes/whitelist/Whitelist.tsx"
import SendHmail from "./routes/send_hmail/SendHmail.tsx"

export const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    errorElement: <ErrorPage />,
  },
  {
    path: "/whitelist",
    element: <Whitelist />,
    errorElement: <ErrorPage />,
  },
  {
    path: "/send_hmail",
    element: <SendHmail />,
    errorElement: <ErrorPage />,
  },
])
