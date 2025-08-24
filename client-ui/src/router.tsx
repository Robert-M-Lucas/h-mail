import { createBrowserRouter } from "react-router-dom"
import ErrorPage from "./error/ErrorPage.tsx"
import InboxPage from "./routes/inbox-page/InboxPage.tsx"
import WhitelistPage from "./routes/whitelist/WhitelistPage.tsx"
import SendHmailPage from "./routes/send-hmail/SendHmailPage.tsx"
import SettingsPage from "./routes/settings/SettingsPage.tsx"

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
    path: "/whitelist",
    element: <WhitelistPage />,
    errorElement: <ErrorPage />,
  },
  {
    path: "/send-hmail",
    element: <SendHmailPage />,
    errorElement: <ErrorPage />,
  },
])
