import { GetHmailsHmail } from "../../interface/get-hmails-hmail.ts"
import { Button, Spinner } from "react-bootstrap"
import { ArrowLeft } from "react-bootstrap-icons"
import HmailUserText from "../../components/hmail-user-text/HmailUserText.tsx"
import { useState } from "react"
import { getHmailByHash } from "../../interface.ts"
import { useAuth } from "../../contexts/AuthContext.tsx"
import { useToast } from "../../contexts/ToastContext.tsx"
import { useNavigate } from "react-router-dom"

export interface Props {
  hmail: GetHmailsHmail | undefined
  toplevel?: boolean
  close: () => void
}

export default function HmailViewer({ hmail, toplevel, close }: Props) {
  const [parent, setParent] = useState<GetHmailsHmail | undefined>(undefined)
  const { showToast } = useToast()
  const [loadingParent, setLoadingParent] = useState<boolean>(false)
  const { logout } = useAuth()
  const navigate = useNavigate()

  const handleCompose = () => {
    if (!hmail || !hmail.reply_to) {
      return
    }
    const queryParams = new URLSearchParams({
      recipients: hmail.reply_to.address,
      subject: "Re: " + hmail.subject,
      parent: hmail.hash,
    })

    navigate(`/compose?${queryParams.toString()}`)
  }

  const handleComposeAll = () => {
    if (!hmail) {
      return
    }
    const queryParams = new URLSearchParams({
      recipients: hmail.recipients.map((r) => r.address).join(","),
      ccs: hmail.ccs.map((c) => c.address).join(","),
      subject: "Re: " + hmail.subject,
      parent: hmail.hash,
    })

    navigate(`/compose?${queryParams.toString()}`)
  }

  const handleComposeForward = () => {
    if (!hmail) {
      return
    }
    const queryParams = new URLSearchParams({
      subject: "Fw: " + hmail.subject,
      parent: hmail.hash,
    })

    navigate(`/compose?${queryParams.toString()}`)
  }

  if (!hmail) {
    throw new Error("Editing not implemented yet")
  }

  return (
    <div className={"w-100"}>
      {toplevel && (
        <a className={"m-0 p-0"} href={"#"} onClick={close}>
          <ArrowLeft /> Back
        </a>
      )}
      {toplevel ? (
        <hr className={"mt-0"} />
      ) : (
        <div
          className={"w-100 mb-3"}
          style={{ backgroundColor: "#dee2e6", height: "60px" }}
        ></div>
      )}
      {hmail.is_context && (
        <>
          <p className={"m-0 p-0 text-danger fw-bold fst-italic text-center"}>
            This h-mail was attached as context - any of its fields may be
            forged!
          </p>
          <hr />
        </>
      )}
      <div className={"d-flex m-3"}>
        <p className={"flex-grow-1 p-0 m-0"}>
          <span className={"me-3 d-inline-block"} style={{ width: "70px" }}>
            From:
          </span>
          <HmailUserText user={hmail.sender} />;
        </p>
        <p className={"flex-grow-1 p-0 m-0"}>
          <span className={"me-3"}>Classification:</span>
          {hmail.pow_classification}
        </p>
      </div>
      <hr />
      <p className={"m-3"}>
        <span className={"me-3 d-inline-block"} style={{ width: "70px" }}>
          To:
        </span>
        {hmail.recipients.map((recipient, i) => (
          <span className={"me-2"} key={i}>
            <HmailUserText user={recipient} />;
          </span>
        ))}
      </p>
      <hr />
      <p className={"m-3"}>
        <span className={"me-3 d-inline-block"} style={{ width: "70px" }}>
          CCs:
        </span>
        {hmail.ccs.map((cc, i) => (
          <span className={"me-2"} key={i}>
            <HmailUserText user={cc} />;
          </span>
        ))}
      </p>
      <hr />
      <p className={"m-3"}>
        <span className={"me-3 d-inline-block"} style={{ width: "70px" }}>
          Subject:
        </span>
        {hmail.subject}
      </p>
      <hr />
      <div className={"my-4"}>
        <p className={"m-3"}>{hmail.body}</p>
      </div>

      {hmail.reply_to ? (
        <div className={"m-3"}>
          <Button variant={"outline-success"} onClick={handleCompose}>
            Reply to <HmailUserText user={hmail.reply_to} />
          </Button>
          <Button
            className={"ms-2"}
            variant={"outline-success"}
            onClick={handleComposeAll}
          >
            Reply to all
          </Button>
          <Button
            className={"ms-2"}
            variant={"outline-primary"}
            onClick={handleComposeForward}
          >
            Forward
          </Button>
        </div>
      ) : (
        <p className={"text-muted text-center m-3"}>
          This sender does not accept replies.
        </p>
      )}

      {loadingParent && (
        <div
          className={
            "w-100 text-center d-flex justify-content-center align-content-center my-4"
          }
        >
          <Spinner />
        </div>
      )}

      {hmail.parent && !parent && !loadingParent && (
        <div className={"text-center m-3"}>
          <Button
            variant={"primary"}
            className={"w-100"}
            onClick={async () => {
              setLoadingParent(true)
              const parent = await getHmailByHash(hmail.parent!, logout)
              if (parent) {
                setParent(parent)
              } else {
                showToast({
                  header: "Missing H-Mail",
                  body: "Previous h-mail in chain not present on the server.",
                })
              }
              setLoadingParent(false)
            }}
          >
            Load previous h-mail in chain
          </Button>
        </div>
      )}

      {parent && <HmailViewer hmail={parent} close={close} />}
    </div>
  )
}
