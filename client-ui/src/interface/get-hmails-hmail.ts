import { HmailUser } from "./hmail-user.ts"

/**
 * A base-64 (standard alphabet, with padding) little-endian encoding of a large unsigned integer
 */
export type BigUintField = string
/**
 * Represents a classification in the `PowPolicy`
 */
export type PowClassification = "Minimum" | "Accepted" | "Personal"
/**
 * A timestamp represented as milliseconds since epoch
 */
export type SystemTimeField = number

/**
 * An individual h-mail in a user's inbox
 */
export interface GetHmailsHmail {
  body: string
  ccs: HmailUser[]
  hash: BigUintField
  incrementing_id: number
  parent?: BigUintField | null
  pow_classification: PowClassification
  received_at: SystemTimeField
  recipients: HmailUser[]
  reply_to?: HmailUser | null
  sender: HmailUser
  sent_at: SystemTimeField
  subject: string
  [k: string]: unknown
}
