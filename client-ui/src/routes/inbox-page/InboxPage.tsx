import { useAuth } from "../../contexts/AuthContext.tsx"
import { Fragment, useEffect, useState } from "react"
import { getHmails } from "../../interface.ts"
import { GetHmailsHmail } from "../../interface/get-hmails-hmail.ts"
import { Button, ButtonGroup, Card, Container, Spinner } from "react-bootstrap"
import InboxHeader from "../../components/InboxHeader.tsx"
import { useToast } from "../../contexts/ToastContext.tsx"
import HmailViewer from "./HmailViewer.tsx"
import { useNavigate } from "react-router-dom"
import useViewportWidth from "../../hooks/useViewportWidth.ts"
import HmailUserText from "../../components/hmail-user-text/HmailUserText.tsx"
import { ArrowRepeat } from "react-bootstrap-icons"

function truncateBody(str: string) {
  if (str.length > 300) {
    return str.substring(0, 300) + "..."
  }
  return str
}

function InboxPage() {
  const { user, logout } = useAuth()
  const { showToast } = useToast()
  const navigate = useNavigate()
  const viewWidth = useViewportWidth()

  const [hmails, setHmails] = useState<GetHmailsHmail[]>([])
  const [outboxHmails, setOutboxHmails] = useState<GetHmailsHmail[]>([])
  const [loadingHmails, setLoadingHmails] = useState<boolean>(true)
  const [viewingHmail, setViewingHmail] = useState<GetHmailsHmail | undefined>(
    undefined
  )
  const [viewingOutbox, setViewingOutbox] = useState(false)

  const refreshHmails = async () => {
    setHmails([])
    setOutboxHmails([])
    setLoadingHmails(true)
    const es = await getHmails(undefined, 10, false, logout)
    if (es) {
      setHmails(es)
    } else {
      showToast({
        header: "Failed to Get H-Mails",
        body: "Failed to get h-mails.",
      })
      setLoadingHmails(false)
      return
    }

    const oes = await getHmails(undefined, 10, true, logout)
    if (oes) {
      setOutboxHmails(oes)
    } else {
      showToast({
        header: "Failed to Get H-Mails",
        body: "Failed to get h-mails.",
      })
      setLoadingHmails(false)
      return
    }

    setLoadingHmails(false)
  }

  useEffect(() => {
    refreshHmails().then()
  }, [])

  const narrowWidth = 400
  const narrowView = viewWidth < 1000

  if (viewingHmail && narrowView) {
    return (
      <HmailViewer
        hmail={viewingHmail}
        toplevel
        close={() => {
          setViewingHmail(undefined)
        }}
      />
    )
  }

  return (
    <>
      <div className={"d-flex flex-column vh-100"}>
        <InboxHeader user={user} logout={logout} />
        <div className={"d-flex flex-grow-1"} style={{ minHeight: 0 }}>
          <Container
            className={"overflow-auto"}
            style={viewingHmail && !narrowView ? { width: narrowWidth } : {}}
          >
            <Button
              variant={"outline-success"}
              className={"w-100 mb-3"}
              onClick={() => navigate("/compose")}
            >
              Compose H-Mail
            </Button>
            <ButtonGroup className={"w-100 mb-3"}>
              <Button
                variant={viewingOutbox ? "outline-primary" : "primary"}
                onClick={() => setViewingOutbox(false)}
              >
                Inbox
              </Button>
              <Button
                variant={viewingOutbox ? "secondary" : "outline-secondary"}
                onClick={() => setViewingOutbox(true)}
              >
                Outbox
              </Button>
              <Button
                disabled={loadingHmails}
                variant={"outline-info"}
                onClick={async () => {
                  if (loadingHmails) return
                  await refreshHmails()
                }}
              >
                <ArrowRepeat />
              </Button>
            </ButtonGroup>
            {(viewingOutbox ? outboxHmails : hmails).length === 0 &&
              !loadingHmails && (
                <p className={"text-center text-muted fst-italic"}>
                  No h-mails
                </p>
              )}
            {(viewingOutbox ? outboxHmails : hmails).map((hmail, index) => (
              <Fragment key={index}>
                <Card
                  className={"mb-3"}
                  onClick={() => {
                    setViewingHmail(hmail)
                  }}
                >
                  <Card.Body>
                    <Card.Title>
                      <span className={"text-muted"}>
                        {new Date(hmail.received_at).toLocaleString()} |
                      </span>{" "}
                      {hmail.subject}
                    </Card.Title>
                    <div>
                      <p className="mb-0">
                        {viewingOutbox ? (
                          <>
                            To:{" "}
                            {hmail.recipients.map((recipient, i) => (
                              <Fragment key={i}>
                                <HmailUserText user={recipient} />;{" "}
                              </Fragment>
                            ))}
                          </>
                        ) : (
                          <>
                            From: <HmailUserText user={hmail.sender} />;
                          </>
                        )}
                      </p>
                      <hr />
                      <p>{truncateBody(hmail.body)}</p>
                    </div>
                  </Card.Body>
                </Card>
              </Fragment>
            ))}
            {loadingHmails ? (
              <div
                className={
                  "w-100 text-center d-flex justify-content-center align-content-center my-4"
                }
              >
                <Spinner />
              </div>
            ) : (
              <div className="w-100 text-center my-4">
                <Button
                  className={"w-100"}
                  variant={"outline-primary"}
                  onClick={async () => {
                    const outbox = viewingOutbox
                    const e_hmails = outbox ? outboxHmails : hmails
                    setLoadingHmails(true)
                    let new_hmails
                    if (e_hmails.length > 0) {
                      new_hmails = await getHmails(
                        e_hmails[e_hmails.length - 1].incrementing_id,
                        3,
                        outbox,
                        logout
                      )
                    } else {
                      new_hmails = await getHmails(undefined, 3, outbox, logout)
                    }
                    if (new_hmails) {
                      if (new_hmails.length === 0) {
                        showToast({
                          header: "No More H-Mails",
                          body: "No more h-mails to load.",
                        })
                      } else {
                        if (outbox) setHmails([...e_hmails, ...new_hmails])
                        else setOutboxHmails([...e_hmails, ...new_hmails])
                      }
                    }
                    setLoadingHmails(false)
                  }}
                >
                  Load More
                </Button>
              </div>
            )}
          </Container>
          {viewingHmail && !narrowView && (
            <div
              className={"border-start overflow-auto"}
              style={{ width: viewWidth - narrowWidth - 1 }}
            >
              <HmailViewer
                hmail={viewingHmail}
                toplevel
                close={() => {
                  setViewingHmail(undefined)
                }}
              />
            </div>
          )}
        </div>
      </div>
    </>
  )
}

export default InboxPage
