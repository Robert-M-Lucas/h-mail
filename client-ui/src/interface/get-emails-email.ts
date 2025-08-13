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
 * Returns the emails in a user's inbox
 */
export interface GetEmailsResponseAuthed {
  emails: GetEmailsEmail[]
}
/**
 * An individual email in a user's inbox
 */
export interface GetEmailsEmail {
  body: string
  cc: EmailUser[]
  hash: BigUintField
  parent?: BigUintField | null
  pow_classification: PowClassification
  received_at: SystemTimeField
  reply_to?: EmailUser | null
  sent_at: SystemTimeField
  source: string
  subject: string
  to: EmailUser[]
  [k: string]: unknown
}
/**
 * Represents an email address, with an optional display name
 */
export interface EmailUser {
  display_name?: string | null
  email: string
  [k: string]: unknown
}
