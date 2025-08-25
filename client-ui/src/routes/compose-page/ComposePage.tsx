import { useAuth } from "../../contexts/AuthContext.tsx"
import { useLocation, useNavigate } from "react-router-dom"
import { useEffect, useState } from "react"
import { ArrowLeft } from "react-bootstrap-icons"
import { HmailAddress, HmailUser } from "../../interface/hmail-user.ts"
import { getHmailByHash, sendHmail } from "../../interface.ts"
import { SendHmailPackage } from "../../interface/send-hmail-package.ts"
import { GetHmailsHmail } from "../../interface/get-hmails-hmail.ts"
import { Button, Container, Modal, Spinner } from "react-bootstrap"
import HmailViewer from "../inbox-page/HmailViewer.tsx"
import HmailUserText from "../../components/hmail-user-text/HmailUserText.tsx"
import "./no-border.css"
import { useLockout } from "../../contexts/LockoutContext.tsx"
import { useToast } from "../../contexts/ToastContext.tsx"
import { SendHmailResultPerDestination } from "../../interface/send-hmail-response-authed.ts"
import RecipientList from "../../components/RecipientList.tsx"
import { PowClassification } from "../../interface/get-foreign-pow-policy-response-authed.ts"
import { sendHmailResultsToStrings } from "./response-converter.ts"

export type Recipient =
  | {
      address: HmailAddress
      status: "to-load"
    }
  | {
      address: HmailAddress
      status: "loading"
    }
  | {
      address: HmailAddress
      status: "failed"
      reason: string
    }
  | {
      address: HmailAddress
      status: "loaded"
      minimum_estimate: number
      accepted_estimate: number
      personal_estimate: number
      selected: PowClassification
    }

function sToTime(duration: number): string {
  const seconds = Math.floor(duration % 60)
  const minutes = Math.floor(duration / 60)

  const mm = String(minutes).padStart(2, "0")
  const ss = String(seconds).padStart(2, "0")

  return `${mm}m${ss}s`
}

function handleQueryLists(q: string | null) {
  if (!q || q.length === 0) return []
  return q.split(",")
}

export default function ComposePage() {
  const { search } = useLocation()
  const query = new URLSearchParams(search)
  const { enterLockout, exitLockout } = useLockout()

  const iRecipients = handleQueryLists(query.get("recipients"))
  const iCcs = handleQueryLists(query.get("ccs"))
  const iSubject = query.get("subject") || ""
  const parentHash = query.get("parent") || undefined

  const { user, logout } = useAuth()
  const navigate = useNavigate()

  const [recipients, setRecipients] = useState<Recipient[]>(
    iRecipients.map((r) => {
      return { address: r, status: "to-load" }
    })
  )
  const [ccs, setCcs] = useState<Recipient[]>(
    iCcs.map((c) => {
      return { address: c, status: "to-load" }
    })
  )
  const [bccs, setBccs] = useState<Recipient[]>([])
  const [subject, setSubject] = useState<string>(iSubject)
  const [body, setBody] = useState<string>("")
  const [recipientsNotReady, setRecipientsNotReady] = useState(false)
  const { showToast } = useToast()

  const [deliverResponse, setDeliverResponse] = useState<
    SendHmailResultPerDestination[] | undefined
  >(undefined)

  const [parent, setParent] = useState<GetHmailsHmail | undefined>(undefined)

  const allRecipients = [...recipients, ...ccs, ...bccs]

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
    if (!allRecipients.every((r) => r.status === "loaded")) {
      setRecipientsNotReady(true)
      return
    }

    enterLockout()

    const ccsM: HmailUser[] = ccs.map((c) => {
      return {
        address: c.address,
      }
    })
    const bccsM: HmailUser[] = bccs.map((c) => {
      return {
        address: c.address,
      }
    })
    const recipientsM: HmailUser[] = recipients.map((c) => {
      return {
        address: c.address,
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

    const responses = await sendHmail(
      hmailPackage,
      bccsM,
      allRecipients.map((r) => [r.address, r.selected]),
      logout
    )

    exitLockout()

    if (responses) {
      setDeliverResponse(responses)
    } else {
      showToast({
        header: "Failed to Send H-Mail",
        body: "Failed to send h-mail.",
      })
    }
  }

  let timeEstimate = 0
  for (const recipient of allRecipients) {
    if (recipient.status === "loaded") {
      switch (recipient.selected) {
        case "Minimum":
          timeEstimate += recipient.minimum_estimate
          break
        case "Accepted":
          timeEstimate += recipient.accepted_estimate
          break
        case "Personal":
          timeEstimate += recipient.personal_estimate
          break
      }
    }
  }

  return (
    <>
      <Modal
        show={recipientsNotReady}
        onHide={() => setRecipientsNotReady(false)}
        centered
        size="lg"
      >
        <Modal.Header closeButton>
          <Modal.Title>Send H-Mail Result</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <Container className={"p-3"}>
            Not all recipients have finished loading or some have failed to be
            contacted.
          </Container>
        </Modal.Body>
      </Modal>

      <Modal show={deliverResponse !== undefined} centered size="lg">
        <Modal.Header>
          <Modal.Title>Send H-Mail Result</Modal.Title>
        </Modal.Header>
        {deliverResponse && (
          <>
            <Modal.Body>
              {sendHmailResultsToStrings(deliverResponse).map((r, i) => (
                <p key={i}>{r}</p>
              ))}
            </Modal.Body>
            <Modal.Footer>
              <Button
                className={"w-100"}
                variant={"outline-success"}
                onClick={() => navigate("/")}
              >
                Done
              </Button>
            </Modal.Footer>
          </>
        )}
      </Modal>

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
        <RecipientList
          title={"To"}
          recipients={recipients}
          setRecipients={setRecipients}
        />
      </p>
      <hr />

      <p
        className={"m-3 w-auto d-flex justify-content-start align-items-center"}
      >
        <RecipientList title={"CCs"} recipients={ccs} setRecipients={setCcs} />
      </p>
      <hr />

      <p
        className={"m-3 w-auto d-flex justify-content-start align-items-center"}
      >
        <RecipientList
          title={"BCCs"}
          recipients={bccs}
          setRecipients={setBccs}
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
          Send - {sToTime(timeEstimate)}
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
