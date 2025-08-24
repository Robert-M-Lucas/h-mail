import "./on-hover.css"
import { Recipient } from "../../routes/compose-page/ComposePage.tsx"
import { PowClassification } from "../../interface/get-foreign-pow-policy-response-authed.ts"

export interface Props {
  recipient: Recipient
  setRecipients: (
    value: ((prevState: Recipient[]) => Recipient[]) | Recipient[]
  ) => void
}

function sToTime(duration: number): string {
  const seconds = Math.floor(duration % 60)
  const minutes = Math.floor(duration / 60)

  const mm = String(minutes).padStart(2, "0")
  const ss = String(seconds).padStart(2, "0")

  return `${mm}m${ss}s`
}

export default function HmailUserTextComposer({
  recipient,
  setRecipients,
}: Props) {
  if (recipient.status === "to-load" || recipient.status === "loading") {
    return (
      <span
        style={{ cursor: "pointer" }}
        className={"user-deletable"}
        onClick={() =>
          setRecipients((recipients) =>
            recipients.filter((r) => r.address !== recipient.address)
          )
        }
      >
        {recipient.address} <span className={"text-muted"}>(Loading...)</span>
      </span>
    )
  } else if (recipient.status === "failed") {
    return (
      <span
        style={{ cursor: "pointer" }}
        className={"user-deletable"}
        onClick={() =>
          setRecipients((recipients) =>
            recipients.filter((r) => r.address !== recipient.address)
          )
        }
      >
        {recipient.address} <span className={"text-danger"}>(Failed)</span>
      </span>
    )
  } else if (recipient.status === "loaded") {
    let estimate
    switch (recipient.selected) {
      case "Minimum":
        estimate = recipient.minimum_estimate
        break
      case "Accepted":
        estimate = recipient.accepted_estimate
        break
      case "Personal":
        estimate = recipient.personal_estimate
        break
    }

    return (
      <span>
        <span
          style={{ cursor: "pointer" }}
          className={"user-deletable"}
          onClick={() =>
            setRecipients((recipients) =>
              recipients.filter((r) => r.address !== recipient.address)
            )
          }
        >
          {recipient.address}
        </span>{" "}
        <span
          className={"text-muted user-switchable"}
          style={{ cursor: "pointer" }}
          onClick={() =>
            setRecipients((recipients) =>
              recipients.map((r) => {
                if (r.address === recipient.address) {
                  let next_selected: PowClassification
                  switch (recipient.selected) {
                    case "Minimum":
                      next_selected = "Accepted"
                      break
                    case "Accepted":
                      next_selected = "Personal"
                      break
                    case "Personal":
                      next_selected = "Minimum"
                      break
                  }

                  return {
                    address: r.address,
                    status: recipient.status,
                    minimum_estimate: recipient.minimum_estimate,
                    accepted_estimate: recipient.accepted_estimate,
                    personal_estimate: recipient.personal_estimate,
                    selected: next_selected,
                  }
                } else {
                  return r
                }
              })
            )
          }
        >
          ({recipient.selected} - {sToTime(estimate)})
        </span>
      </span>
    )
  }
}
