import { useAuth } from "../contexts/AuthContext.tsx"
import { Fragment, useEffect, useState } from "react"
import { getHmails } from "../interface.ts"
import { useNavigate } from "react-router-dom"
import { GetHmailsHmail } from "../interface/get-hmails-hmail.ts"
import { Button, Card, Container, Navbar } from "react-bootstrap"
import { BoxArrowRight, Gear } from "react-bootstrap-icons"
import { useLockout } from "../contexts/LockoutProvider.tsx"

function InboxPage() {
  const { user, logout } = useAuth()
  const { enterLockout, exitLockout } = useLockout()

  const [hmails, setHmails] = useState<GetHmailsHmail[] | undefined>(undefined)

  useEffect(() => {
    getHmails(undefined, 10, logout).then((es) => {
      setHmails(es)
    })
  }, [])

  const navigate = useNavigate()

  return (
    <>
      <Navbar bg="light" className="px-3">
        <Container fluid>
          <Navbar.Brand className="fw-bold fs-4 me-5">
            {user.domain}
          </Navbar.Brand>

          <div className="d-flex align-items-center gap-3 ms-2">
            <span className="fw-semibold">{user.name}</span>
            <Button
              variant="outline-danger"
              size="sm"
              className="d-flex align-items-center"
              onClick={logout}
            >
              <BoxArrowRight className="me-1" size={18} />
              Logout
            </Button>
            <Button
              variant="outline-secondary"
              size="sm"
              className="d-flex align-items-center"
              onClick={() => navigate("/settings", { viewTransition: true })}
            >
              <Gear className="me-1" size={18} />
              Settings
            </Button>
          </div>
        </Container>
      </Navbar>
      <hr className={"mt-0"} />

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
          <Button
            variant={"outline-primary"}
            onClick={async () => {
              enterLockout()
              const new_hmails = await getHmails(
                hmails[hmails.length - 1].incrementing_id,
                3,
                logout
              )
              if (new_hmails) {
                setHmails([...hmails, ...new_hmails])
              }
              exitLockout()
            }}
          >
            Load More
          </Button>
        </>
      )}
    </>
  )
}

export default InboxPage
