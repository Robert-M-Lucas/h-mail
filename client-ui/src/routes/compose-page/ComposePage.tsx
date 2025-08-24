import { useAuth } from "../../contexts/AuthContext.tsx"
import { useLocation, useNavigate } from "react-router-dom"
import { useEffect, useState } from "react"
import { ArrowLeft } from "react-bootstrap-icons"
import { HmailUser } from "../../interface/hmail-user.ts"
import { getHmailByHash, sendHmail } from "../../interface.ts"
import { SendHmailPackage } from "../../interface/send-hmail-package.ts"
import { GetHmailsHmail } from "../../interface/get-hmails-hmail.ts"
import { Button, Spinner } from "react-bootstrap"
import HmailViewer from "../inbox-page/HmailViewer.tsx"
import HmailUserText from "../../components/hmail-user-text/HmailUserText.tsx"
import "./no-border.css"
import { useLockout } from "../../contexts/LockoutProvider.tsx"

export default function ComposePage() {
  const { search } = useLocation()
  const query = new URLSearchParams(search)
  const { enterLockout, exitLockout } = useLockout()

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
    enterLockout()
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
      sender: { address: `${user.name}#${user.domain}` },
      body: body,
      ccs: ccsM,
      random_id: Math.floor(Math.random() * 1_000_000),
      recipients: recipientsM,
      sent_at: Math.floor(Date.now()),
      subject: subject,
      parent: parentHash,
      reply_to: { address: `${user.name}#${user.domain}` },
    }

    console.log(hmailPackage)

    const responses = await sendHmail(hmailPackage, bccsM, logout)

    exitLockout()

    console.warn(responses)
  }

  return (
    <>
      <a className={"m-0 p-0"} href={"#"} onClick={() => navigate("/")}>
        <ArrowLeft /> Back
      </a>
      <hr className={"mt-0"} />
      <p className={"m-3"}>
        <span className={"me-3 d-inline-block"} style={{ width: "70px" }}>
          From:
        </span>
        <HmailUserText user={{ address: `${user.name}#${user.domain}` }} />;
      </p>
      <hr />
      <p
        className={"m-3 w-auto d-flex justify-content-start align-items-center"}
      >
        <span className={"me-3"} style={{ width: "70px" }}>
          To:
        </span>
        {recipients.map((recipient, i) => (
          <span className={"me-2"} key={i}>
            <HmailUserText
              user={{ address: recipient }}
              onDelete={() => {
                setRecipients(recipients.filter((_, index) => index !== i))
              }}
            />
            ;
          </span>
        ))}
        <input
          className={"w-auto flex-grow-1 no-border"}
          onChange={(e) => setRecipientVal(e.currentTarget.value)}
          onKeyDown={(e) => {
            if (e.key !== "Enter") {
              return
            }
            setRecipients([...recipients, recipientVal])
            setRecipientVal("")
          }}
          value={recipientVal}
        />
      </p>
      <hr />

      <p
        className={"m-3 w-auto d-flex justify-content-start align-items-center"}
      >
        <span className={"me-3"} style={{ width: "70px" }}>
          CCs:
        </span>
        {ccs.map((cc, i) => (
          <span className={"me-2"} key={i}>
            <HmailUserText
              user={{ address: cc }}
              onDelete={() => {
                setCcs(ccs.filter((_, index) => index !== i))
              }}
            />
            ;
          </span>
        ))}
        <input
          className={"w-auto flex-grow-1 no-border"}
          onChange={(e) => setCcVal(e.currentTarget.value)}
          onKeyDown={(e) => {
            if (e.key !== "Enter") {
              return
            }
            setCcs([...ccs, ccVal])
            setCcVal("")
          }}
          value={ccVal}
        />
      </p>
      <hr />

      <p
        className={"m-3 w-auto d-flex justify-content-start align-items-center"}
      >
        <span className={"me-3"} style={{ width: "70px" }}>
          BCCs:
        </span>
        {bccs.map((bcc, i) => (
          <span className={"me-2"} key={i}>
            <HmailUserText
              user={{ address: bcc }}
              onDelete={() => {
                setBccs(bccs.filter((_, index) => index !== i))
              }}
            />
            ;
          </span>
        ))}
        <input
          className={"w-auto flex-grow-1 no-border"}
          onChange={(e) => setBccVal(e.currentTarget.value)}
          onKeyDown={(e) => {
            if (e.key !== "Enter") {
              return
            }
            setBccs([...bccs, bccVal])
            setBccVal("")
          }}
          value={bccVal}
        />
      </p>
      <hr />

      <p
        className={"m-3 w-auto d-flex justify-content-start align-items-center"}
      >
        <span className={"me-3"} style={{ width: "70px" }}>
          Subject:
        </span>
        <input
          className={"w-auto flex-grow-1 no-border"}
          onChange={(e) => setSubject(e.currentTarget.value)}
          value={subject}
        />
      </p>
      <hr />
      <div className={"mt-3 d-flex"}>
        <textarea
          className={"m-3 flex-grow-1 no-border"}
          style={{ minHeight: "300px" }}
          onChange={(e) => setBody(e.currentTarget.value)}
          value={body}
        />
      </div>

      <div className={"m-3 w-auto"}>
        <Button variant={"outline-success"} onClick={send}>
          Send
        </Button>
      </div>

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
