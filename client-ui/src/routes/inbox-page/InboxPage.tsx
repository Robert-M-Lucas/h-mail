import { useAuth } from "../../contexts/AuthContext.tsx"
import { Fragment, useEffect, useState } from "react"
import { getHmails } from "../../interface.ts"
import { GetHmailsHmail } from "../../interface/get-hmails-hmail.ts"
import { Button, Card, Container, Spinner } from "react-bootstrap"
import InboxHeader from "../../components/InboxHeader.tsx"
import { useToast } from "../../contexts/ToastContext.tsx"
import HmailViewer from "./HmailViewer.tsx"

function InboxPage() {
  const { user, logout } = useAuth()
  const { showToast } = useToast()

  const [hmails, setHmails] = useState<GetHmailsHmail[]>([])
  const [loadingHmails, setLoadingHmails] = useState<boolean>(true)
  const [viewingHmail, setViewingHmail] = useState<GetHmailsHmail | undefined>(
    undefined
  )

  useEffect(() => {
    getHmails(undefined, 10, logout).then((es) => {
      setLoadingHmails(false)
      if (es) {
        setHmails(es)
      } else {
        showToast({
          header: "Failed to Get Emails",
          body: "Failed to get emails.",
        })
      }
    })
  }, [])

  if (viewingHmail) {
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
      <InboxHeader user={user} logout={logout} />
      <Container>
        {hmails.length === 0 && !loadingHmails && (
          <p className={"text-center text-muted fst-italic"}>No h-mails</p>
        )}
        {hmails.map((hmail, index) => (
          <Fragment key={index}>
            {index !== 0 && <hr />}
            <Card
              onClick={() => {
                setViewingHmail(hmail)
              }}
            >
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
                setLoadingHmails(true)
                let new_hmails
                if (hmails.length > 0) {
                  new_hmails = await getHmails(
                    hmails[hmails.length - 1].incrementing_id,
                    3,
                    logout
                  )
                } else {
                  new_hmails = await getHmails(undefined, 3, logout)
                }
                if (new_hmails) {
                  if (new_hmails.length === 0) {
                    showToast({
                      header: "No More H-Mails",
                      body: "No more h-mails to load.",
                    })
                  } else {
                    setHmails([...hmails, ...new_hmails])
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
    </>
  )
}

export default InboxPage
