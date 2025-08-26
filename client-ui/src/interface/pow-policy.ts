/**
 * Represents a user's pow policy that dictates how an incoming h-mail is categorised
 */
export interface PowPolicy {
  accepted: number
  minimum: number
  personal: number
  [k: string]: unknown
}
