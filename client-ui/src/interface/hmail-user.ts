/**
 * Represents a valid h-mail address - same as email addresses but with a '#' replacing the '@'
 */
export type HmailAddress = string

/**
 * Represents a h-mail address, with an optional display name
 */
export interface HmailUser {
  address: HmailAddress
  display_name?: string | null
  [k: string]: unknown
}
