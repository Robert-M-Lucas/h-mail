import { HmailUser } from "./hmail-user.ts"

/**
 * A base-64 (standard alphabet, with padding) little-endian encoding of a large unsigned integer
 */
export type BigUintField = string
/**
 * A timestamp represented as milliseconds since epoch
 */
export type SystemTimeField = number

/**
 * Represents an email being sent. The email's hash is used to identify an email uniquely (for
 * replying to emails), with the `random_id` being used to differentiate two exactly identical
 * emails. As the `random_id` is client-chosen, the hash of the email should not be used as a UID
 * for servers as a client can easily construct two emails with identical hashes.
 */
export interface SendHmailPackage {
  body: string
  ccs: HmailUser[]
  parent?: BigUintField | null
  /**
   * If two emails are the same / have the same hash, differentiate them
   */
  random_id: number
  recipients: HmailUser[]
  reply_to?: HmailUser | null
  sender: HmailUser
  sent_at: SystemTimeField
  subject: string
  [k: string]: unknown
}
