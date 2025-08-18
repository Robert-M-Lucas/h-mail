import { useAuth } from "../AuthContext.tsx"
import { Fragment, useEffect, useState } from "react"
import { getHmails, getServer } from "../interface.ts"
import { useNavigate } from "react-router-dom"
import { GetHmailsHmail } from "../interface/get-hmails-hmail.ts"
import { Card } from "react-bootstrap"

function App() {
  const { user, logout } = useAuth()

  const [server, setServer] = useState<string>("-")
  const [emails, setEmails] = useState<GetHmailsHmail[] | undefined>(undefined)

  useEffect(() => {
    getServer().then((s) => setServer(s ?? "-"))

    getHmails(logout).then((es) => {
      setEmails(es)
    })
  }, [])

  const navigate = useNavigate()

  return (
    <>
      <h1>
        Welcome to {server}, {user.name}.
      </h1>
      <button className="btn btn-outline-danger" onClick={() => logout()}>
        Logout
      </button>
      <button
        className="btn btn-outline-dark"
        onClick={() => navigate("/whitelist")}
      >
        Whitelist
      </button>
      <button className="btn btn-outline-primary">Send Email</button>
      {emails && (
        <>
          <p>Emails:</p>
          {emails.map((email, index) => (
            <Fragment key={index}>
              <hr />
              <div className="p-3">
                <Card>
                  <Card.Body>
                    <Card.Title>{email.subject}</Card.Title>
                    <div>
                      <p className="mb-0">
                        To:{" "}
                        {email.to.map((to, i) => (
                          <Fragment key={i}>
                            {i !== 0 && <span key={i + 1}>; </span>}
                            <span key={i * 2 + 1}>
                              {to.display_name && to.display_name}
                              {"<"}
                              {to.address}
                              {">"}
                            </span>
                          </Fragment>
                        ))}
                      </p>
                      <hr />
                      <p>{email.body}</p>
                      <p className="mb-0">
                        Reply to:{" "}
                        {email.reply_to ? (
                          <span>
                            {email.reply_to.display_name &&
                              email.reply_to.display_name}
                            {"<"}
                            {email.reply_to.address}
                            {">"}
                          </span>
                        ) : (
                          "No Reply"
                        )}
                      </p>
                    </div>
                  </Card.Body>
                </Card>
              </div>
            </Fragment>
          ))}
        </>
      )}
    </>
  )
}

export default App
