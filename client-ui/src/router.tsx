import { createBrowserRouter } from "react-router-dom"
import ErrorPage from "./error/ErrorPage.tsx"
import InboxPage from "./routes/inbox-page/InboxPage.tsx"
import ComposePage from "./routes/compose-page/ComposePage.tsx"
import SettingsPage from "./routes/settings-page/SettingsPage.tsx"

export const router = createBrowserRouter([
  {
    path: "/",
    element: <InboxPage />,
    errorElement: <ErrorPage />,
  },
  {
    path: "/settings",
    element: <SettingsPage />,
    errorElement: <ErrorPage />,
  },
  {
    path: "/compose",
    element: <ComposePage />,
    errorElement: <ErrorPage />,
  },
])
