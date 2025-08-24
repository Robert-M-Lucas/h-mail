import { GetHmailsHmail } from "../../interface/get-hmails-hmail.ts"
import { Button } from "react-bootstrap"
import { ArrowLeft } from "react-bootstrap-icons"
import HmailUserText from "../../components/HmailUserText.tsx"
import { useState } from "react"
import { useLockout } from "../../contexts/LockoutProvider.tsx"
import { getHmailByHash } from "../../interface.ts"
import { useAuth } from "../../contexts/AuthContext.tsx"
import { useToast } from "../../contexts/ToastContext.tsx"

export interface Props {
  hmail: GetHmailsHmail | undefined
  toplevel?: boolean
  close: () => void
}

export default function HmailViewer({ hmail, toplevel, close }: Props) {
  const [parent, setParent] = useState<GetHmailsHmail | undefined>(undefined)
  const { showToast } = useToast()
  const { enterLockout, exitLockout } = useLockout()
  const { logout } = useAuth()

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
      <hr className={"mt-0"} />
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
          <span className={"me-3"}>From:</span>
          <HmailUserText user={hmail.sender} />;
        </p>
        <p className={"flex-grow-1 p-0 m-0"}>
          <span className={"me-3"}>Classification:</span>
          {hmail.pow_classification}
        </p>
      </div>
      <hr />
      <p className={"m-3"}>
        <span className={"me-3"}>To:</span>
        {hmail.recipients.map((recipient, i) => (
          <span className={"me-2"} key={i}>
            <HmailUserText user={recipient} />;
          </span>
        ))}
      </p>
      <hr />
      <p className={"m-3"}>
        <span className={"me-3"}>CCs:</span>
        {hmail.ccs.map((cc, i) => (
          <span className={"me-2"} key={i}>
            <HmailUserText user={cc} />;
          </span>
        ))}
      </p>
      <hr />
      <p className={"m-3"}>
        <span className={"me-3"}>Subject:</span>
        {hmail.subject}
      </p>
      <hr />
      <p className={"m-3"}>{hmail.body}</p>

      {hmail.reply_to ? (
        <div className={"text-center m-3"}>
          <Button variant={"outline-success"}>
            Reply to <HmailUserText user={hmail.reply_to} />
          </Button>
        </div>
      ) : (
        <p className={"text-muted text-center m-3"}>
          This sender does not accept replies.
        </p>
      )}

      {hmail.parent && !parent && (
        <div className={"text-center m-3"}>
          <Button
            variant={"primary"}
            className={"w-100"}
            onClick={async () => {
              enterLockout()
              console.log(hmail.parent!)
              const parent = await getHmailByHash(hmail.parent!, logout)
              if (parent) {
                setParent(parent)
              } else {
                showToast({
                  header: "Missing H-Mail",
                  body: "Previous h-mail in chain not present on the server.",
                })
              }

              exitLockout()
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
