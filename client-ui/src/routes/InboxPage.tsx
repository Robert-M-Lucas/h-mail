import { useAuth } from "../contexts/AuthContext.tsx"
import { Fragment, useEffect, useState } from "react"
import { getHmails } from "../interface.ts"
import { GetHmailsHmail } from "../interface/get-hmails-hmail.ts"
import { Button, Card } from "react-bootstrap"
import { useLockout } from "../contexts/LockoutProvider.tsx"
import InboxHeader from "../components/InboxHeader.tsx"

function InboxPage() {
  const { user, logout } = useAuth()
  const { enterLockout, exitLockout } = useLockout()

  const [hmails, setHmails] = useState<GetHmailsHmail[] | undefined>(undefined)

  useEffect(() => {
    getHmails(undefined, 10, logout).then((es) => {
      setHmails(es)
    })
  }, [])

  return (
    <>
      <InboxHeader user={user} logout={logout} />
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
