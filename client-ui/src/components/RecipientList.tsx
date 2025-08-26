import { useEffect, useState } from "react"
import { getForeignPowPolicy } from "../interface.ts"
import { useAuth } from "../contexts/AuthContext.tsx"
import { Recipient } from "../routes/compose-page/ComposePage.tsx"
import { PowClassification } from "../interface/get-foreign-pow-policy-response-authed.ts"
import { useEstimate } from "../contexts/EstimateContext.tsx"
import HmailUserTextComposer from "./hmail-user-text/HmailUserTextComposer.tsx"
import { PowPolicy } from "../interface/pow-policy.ts"

export interface Props {
  title: string
  recipients: Recipient[]
  setRecipients: (
    value: ((prevState: Recipient[]) => Recipient[]) | Recipient[]
  ) => void
}

export default function RecipientList({
  title,
  recipients,
  setRecipients,
}: Props) {
  const { logout } = useAuth()
  const [recipientVal, setRecipientVal] = useState("")
  const estimate = useEstimate()

  const loadRecipient = (recipient: string) => {
    getForeignPowPolicy(recipient, logout).then((res) => {
      let r: Recipient

      if (!res || typeof res !== "object") {
        r = {
          address: recipient,
          status: "failed",
          reason: "Error",
        }
      } else {
        let policy: PowPolicy
        let classification: PowClassification | undefined = undefined
        let selected: PowClassification = "Accepted"

        if ("Whitelisted" in res) {
          classification = res.Whitelisted.classification
          policy = res.Whitelisted.policy
        } else {
          policy = res.NotWhitelisted
        }

        if (classification) {
          switch (classification) {
            case "Minimum":
              policy.minimum = 0
              break
            case "Accepted":
              policy.accepted = 0
              break
            case "Personal":
              policy.personal = 0
              selected = "Personal"
              break
          }
        }

        r = {
          address: recipient,
          status: "loaded",
          minimum_estimate: policy.minimum / estimate,
          accepted_estimate: policy.accepted / estimate,
          personal_estimate: policy.personal / estimate,
          selected,
        }
      }

      setRecipients((recipients) =>
        recipients.map((mr) => {
          if (mr.address === recipient) {
            return r
          } else {
            return mr
          }
        })
      )
    })
  }

  useEffect(() => {
    recipients.forEach((r) => {
      if (r.status === "to-load") {
        loadRecipient(r.address)
      }
    })
  }, [])

  const addRecipient = () => {
    if (recipientVal === "") {
      return
    }
    getForeignPowPolicy(recipientVal, logout).then((res) => {
      console.log(res)
    })
    setRecipients([
      ...recipients,
      {
        address: recipientVal,
        status: "loading",
      },
    ])
    loadRecipient(recipientVal)
    setRecipientVal("")
  }

  return (
    <>
      <span className={"me-3"} style={{ width: "70px" }}>
        {title}:
      </span>
      {recipients.map((recipient, i) => (
        <span className={"me-2"} key={i}>
          <HmailUserTextComposer
            recipient={recipient}
            setRecipients={setRecipients}
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
          addRecipient()
        }}
        onBlur={addRecipient}
        value={recipientVal}
      />
    </>
  )
}
