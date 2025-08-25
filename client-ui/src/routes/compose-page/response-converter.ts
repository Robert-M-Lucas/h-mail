import {
  DeliverHmailResponse,
  SendHmailResult,
  SendHmailResultPerDestination,
} from "../../interface/send-hmail-response-authed.ts"
import { HmailAddress } from "../../interface/hmail-user.ts"

export function sendHmailResultsToStrings(
  response: SendHmailResultPerDestination[]
): string[] {
  // Helper to format HmailAddress as string
  const formatAddress = (addr: HmailAddress) => `${addr}`

  // Helper to stringify DeliverHmailResponse
  const deliverResponseToString = (resp: DeliverHmailResponse): string => {
    if (typeof resp === "string") {
      switch (resp) {
        case "Success":
          return "Delivery succeeded"
        case "UserNotFound":
          return "User not found"
        case "BadRequest":
          return "Bad request"
        case "SenderIpNotAuthed":
          return "Sender IP not authorized"
      }
    } else if ("DoesNotMeetPolicy" in resp) {
      const p = resp.DoesNotMeetPolicy
      return `Does not meet policy: accepted=${p.accepted}, minimum=${p.minimum}, personal=${p.personal}`
    } else if ("PowFailure" in resp) {
      const f = resp.PowFailure
      if (typeof f === "string") {
        switch (f) {
          case "FailedNoRetry":
            return "POW failed, cannot retry"
          case "NotFoundCanRetry":
            return "POW not found, can retry"
          case "BadRequestCanRetry":
            return "POW bad request, can retry"
        }
      } else if ("DoesNotMeetPolicyMinimum" in f) {
        return `POW does not meet minimum: ${f.DoesNotMeetPolicyMinimum}`
      }
    }
    return "Unknown delivery result"
  }

  // Helper to stringify SendHmailResult
  const sendResultToString = (res: SendHmailResult): string => {
    if (res === "Failed") return "Send failed"
    if ("DeliveryResult" in res) {
      return deliverResponseToString(res.DeliveryResult)
    }
    return "Unknown send result"
  }

  return response.map(
    ({ recipient, result }) =>
      `${formatAddress(recipient)}: ${sendResultToString(result)}`
  )
}
