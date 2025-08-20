import { useAuth } from "../contexts/AuthContext.tsx"
import { Fragment, useEffect, useState } from "react"
import { getHmails, getServer } from "../interface.ts"
import { useNavigate } from "react-router-dom"
import { GetHmailsHmail } from "../interface/get-hmails-hmail.ts"
import { Card } from "react-bootstrap"

function App() {
  const { user, logout } = useAuth()

  const [server, setServer] = useState<string>("-")
  const [hmails, setHmails] = useState<GetHmailsHmail[] | undefined>(undefined)

  useEffect(() => {
    getServer().then((s) => setServer(s ?? "-"))

    getHmails(logout).then((es) => {
      setHmails(es)
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
        onClick={() => navigate("/whitelist", { viewTransition: true })}
      >
        Whitelist
      </button>
      <button
        className="btn btn-outline-primary"
        onClick={() => navigate("/send_hmail", { viewTransition: true })}
      >
        Send H-mail
      </button>
      {hmails && (
        <>
          <p>Emails:</p>
          {hmails.map((hmail, index) => (
            <Fragment key={index}>
              <hr />
              <div className="p-3">
                <Card>
                  <Card.Body>
                    <Card.Title>{hmail.subject}</Card.Title>
                    <div>
                      <p className="mb-0">
                        To:{" "}
                        {hmail.recipients.map((recipient, i) => (
                          <Fragment key={i}>
                            {i !== 0 && <span key={i + 1}>; </span>}
                            <span key={i * 2 + 1}>
                              {recipient.display_name && recipient.display_name}
                              {"<"}
                              {recipient.address}
                              {">"}
                            </span>
                          </Fragment>
                        ))}
                      </p>
                      <hr />
                      <p>{hmail.body}</p>
                      <p className="mb-0">
                        Reply to:{" "}
                        {hmail.reply_to ? (
                          <span>
                            {hmail.reply_to.display_name &&
                              hmail.reply_to.display_name}
                            {"<"}
                            {hmail.reply_to.address}
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
