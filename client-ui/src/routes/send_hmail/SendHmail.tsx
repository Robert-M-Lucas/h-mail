import { useAuth } from "../../AuthContext.tsx"
import { useNavigate } from "react-router-dom"
import { useState } from "react"
import { PlusLg, XLg } from "react-bootstrap-icons"
import { HmailUser } from "../../interface/hmail-user.ts"
import { sendHmail } from "../../interface.ts"
import { SendHmailPackage } from "../../interface/send-hmail-package.ts"

export default function SendHmail() {
  const { user, logout } = useAuth()
  const navigate = useNavigate()

  const [recipients, setRecipients] = useState<string[]>([])
  const [recipientVal, setRecipientVal] = useState<string>("")
  const [ccs, setCcs] = useState<string[]>([])
  const [ccVal, setCcVal] = useState<string>("")
  const [bccs, setBccs] = useState<string[]>([])
  const [bccVal, setBccVal] = useState<string>("")
  const [subject, setSubject] = useState<string>("")
  const [body, setBody] = useState<string>("")

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
      <button
        className="btn btn-outline-secondary"
        onClick={() => navigate(-1)}
      >
        Back
      </button>
      <h1>
        Send from {user.name}#{user.domain}
      </h1>

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

      <p className="mb-0">Subject:</p>
      <input
        onChange={(e) => setSubject(e.currentTarget.value)}
        value={subject}
      />

      <p className="mb-0">Body:</p>
      <textarea onChange={(e) => setBody(e.currentTarget.value)} value={body} />

      <button className="btn btn-success" onClick={send}>
        Send
      </button>
    </>
  )
}
