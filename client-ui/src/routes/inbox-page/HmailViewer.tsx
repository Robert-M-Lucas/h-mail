import { GetHmailsHmail } from "../../interface/get-hmails-hmail.ts"
import { Button } from "react-bootstrap"
import { ArrowLeft } from "react-bootstrap-icons"
import HmailUserText from "../../components/HmailUserText.tsx"

export interface Props {
  hmail: GetHmailsHmail | undefined
  close: () => void
}

export default function HmailViewer({ hmail, close }: Props) {
  if (!hmail) {
    throw new Error("Editing not implemented yet")
  }

  return (
    <div className={"w-100"}>
      <a className={"m-0 p-0"} href={"#"} onClick={close}>
        <ArrowLeft /> Back
      </a>
      <hr className={"mt-0"} />
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
    </div>
  )
}
