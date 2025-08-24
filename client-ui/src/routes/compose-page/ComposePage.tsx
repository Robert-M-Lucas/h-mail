import { useAuth } from "../../contexts/AuthContext.tsx"
import { useLocation, useNavigate } from "react-router-dom"
import { useEffect, useState } from "react"
import { ArrowLeft, PlusLg, XLg } from "react-bootstrap-icons"
import { HmailUser } from "../../interface/hmail-user.ts"
import { getHmailByHash, sendHmail } from "../../interface.ts"
import { SendHmailPackage } from "../../interface/send-hmail-package.ts"
import { GetHmailsHmail } from "../../interface/get-hmails-hmail.ts"
import { Container, Spinner } from "react-bootstrap"
import HmailViewer from "../inbox-page/HmailViewer.tsx"
import HmailUserText from "../../components/HmailUserText.tsx"
import "./no-border.css"

export default function ComposePage() {
  const { search } = useLocation()
  const query = new URLSearchParams(search)

  const iRecipients = query.get("recipients")?.split(",") || []
  const iCcs = query.get("ccs")?.split(",") || []
  const iSubject = query.get("subject") || ""
  const parentHash = query.get("parent") || undefined

  const { user, logout } = useAuth()
  const navigate = useNavigate()

  const [recipients, setRecipients] = useState<string[]>(iRecipients)
  const [recipientVal, setRecipientVal] = useState<string>("")
  const [ccs, setCcs] = useState<string[]>(iCcs)
  const [ccVal, setCcVal] = useState<string>("")
  const [bccs, setBccs] = useState<string[]>([])
  const [bccVal, setBccVal] = useState<string>("")
  const [subject, setSubject] = useState<string>(iSubject)
  const [body, setBody] = useState<string>("")

  const [parent, setParent] = useState<GetHmailsHmail | undefined>(undefined)

  useEffect(() => {
    if (parentHash) {
      getHmailByHash(parentHash, logout).then((parent) => {
        if (parent) {
          setParent(parent)
        }
      })
    }
  }, [])

  const send = async () => {
    const ccsM: HmailUser[] = ccs.map((c) => {
      return {
        address: c,
      }
    })
    const bccsM: HmailUser[] = bccs.map((c) => {
      return {
        address: c,
      }
    })
    const recipientsM: HmailUser[] = recipients.map((c) => {
      return {
        address: c,
      }
    })

    const hmailPackage: SendHmailPackage = {
      body: body,
      ccs: ccsM,
      random_id: Math.floor(Math.random() * 1_000_000),
      recipients: recipientsM,
      sent_at: Math.floor(Date.now()),
      subject: subject,
    }

    const responses = await sendHmail(hmailPackage, bccsM, logout)

    console.warn(responses)
  }

  return (
    <>
      <a className={"m-0 p-0"} href={"#"} onClick={() => navigate("/")}>
        <ArrowLeft /> Back
      </a>
      <hr className={"mt-0"} />
      <p className={"m-3"}>
        <span className={"me-3"}>From:</span>
        <HmailUserText user={{ address: `${user.name}#${user.domain}` }} />;
      </p>
      <hr />
      <p className={"m-3"}>
        <span className={"me-3"}>To:</span>
        {recipients.map((recipient, i) => (
          <span className={"me-2"} key={i}>
            <HmailUserText user={{ address: recipient }} />;
          </span>
        ))}
      </p>
      <hr />
      <p className={"m-3"}>
        <span className={"me-3"}>CCs:</span>
        {ccs.map((cc, i) => (
          <span className={"me-2"} key={i}>
            <HmailUserText user={{ address: cc }} />;
          </span>
        ))}
      </p>
      <hr />
      <p
        className={"m-3 w-auto d-flex justify-content-start align-items-center"}
      >
        <span className={"me-3"}>Subject:</span>
        <input
          className={"w-auto flex-grow-1 no-border"}
          onChange={(e) => setSubject(e.currentTarget.value)}
          value={subject}
        />
      </p>
      <hr />
      <Container className={"my-4"}>
        <textarea
          className={"m-3 w-100"}
          style={{ minHeight: "300px" }}
          onChange={(e) => setBody(e.currentTarget.value)}
          value={body}
        />
      </Container>

      <p className="mb-0">Recipients:</p>
      <div>
        {recipients.map((recipient, i) => (
          <span key={i} className="me-2">
            {recipient}{" "}
            <button
              className="btn btn-outline-danger"
              onClick={() => {
                setRecipients(recipients.filter((_, index) => index !== i))
              }}
            >
              <XLg />
            </button>
          </span>
        ))}

        <span>
          <input
            onChange={(e) => setRecipientVal(e.currentTarget.value)}
            value={recipientVal}
          />
          <button
            className="btn btn-outline-secondary"
            onClick={() => {
              setRecipients([...recipients, recipientVal])
              setRecipientVal("")
            }}
          >
            <PlusLg />
          </button>
        </span>
      </div>

      <p className="mb-0">CCs:</p>
      <div>
        {ccs.map((cc, i) => (
          <span key={i} className="me-2">
            {cc}{" "}
            <button
              className="btn btn-outline-danger"
              onClick={() => {
                setCcs(ccs.filter((_, index) => index !== i))
              }}
            >
              <XLg />
            </button>
          </span>
        ))}

        <span>
          <input
            onChange={(e) => setCcVal(e.currentTarget.value)}
            value={ccVal}
          />
          <button
            className="btn btn-outline-secondary"
            onClick={() => {
              setCcs([...ccs, ccVal])
              setCcVal("")
            }}
          >
            <PlusLg />
          </button>
        </span>
      </div>

      <p className="mb-0">BCCs:</p>
      <div>
        {bccs.map((bcc, i) => (
          <span key={i} className="me-2">
            {bcc}{" "}
            <button
              className="btn btn-outline-danger"
              onClick={() => {
                setBccs(bccs.filter((_, index) => index !== i))
              }}
            >
              <XLg />
            </button>
          </span>
        ))}

        <span>
          <input
            onChange={(e) => setBccVal(e.currentTarget.value)}
            value={bccVal}
          />
          <button
            className="btn btn-outline-secondary"
            onClick={() => {
              setBccs([...bccs, bccVal])
              setBccVal("")
            }}
          >
            <PlusLg />
          </button>
        </span>
      </div>

      <button className="btn btn-success" onClick={send}>
        Send
      </button>

      {parentHash && !parent && (
        <div
          className={
            "w-100 text-center d-flex justify-content-center align-content-center my-4"
          }
        >
          <Spinner />
        </div>
      )}

      {parent && <HmailViewer hmail={parent} close={() => {}} />}
    </>
  )
}
