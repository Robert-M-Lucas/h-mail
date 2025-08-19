import { HmailAddress } from "./hmail-user.ts"

/**
 * Returns whether sending the h-mail succeeded and, if not, why for each recipient
 */
export type SendHmailResponseAuthed =
  | ("DuplicateDestination" | "BadRequest")
  | {
      DeliverResponse: SendHmailResultPerDestination[]
    }
  | {
      MissingPowFor: HmailAddress
    }
/**
 * The result of trying to send an h-mail
 */
export type SendHmailResult =
  | "Failed"
  | {
      DeliveryResult: DeliverHmailResponse
    }
/**
 * Returns whether the h-mail delivery succeeded and, if not, why
 */
export type DeliverHmailResponse =
  | ("Success" | "UserNotFound" | "BadRequest" | "SenderIpNotAuthed")
  | {
      DoesNotMeetPolicy: PowPolicy
    }
  | {
      PowFailure: PowFailureReason
    }
/**
 * Reason for a POW check failing
 */
export type PowFailureReason =
  | ("FailedNoRetry" | "NotFoundCanRetry" | "BadRequestCanRetry")
  | {
      DoesNotMeetPolicyMinimum: number
    }

/**
 * The result of trying to send an h-ail to one recipient
 */
export interface SendHmailResultPerDestination {
  recipient: HmailAddress
  result: SendHmailResult
  [k: string]: unknown
}
/**
 * Represents a user's pow policy that dictates how an incoming h-mail is categorised
 */
export interface PowPolicy {
  accepted: number
  minimum: number
  personal: number
  [k: string]: unknown
}
