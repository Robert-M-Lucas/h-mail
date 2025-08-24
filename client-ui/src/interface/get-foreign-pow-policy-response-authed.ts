/**
 * Returns whether this authenticated user is whitelisted by the recipient (and their POW policy
 * if not)
 */
export type GetForeignPowPolicyResponseAuthed =
  | ("RequestFailed" | "BadRequest" | "UserDoesNotExist")
  | {
      Whitelisted: ForeignWhitelistedResponse
    }
  | {
      NotWhitelisted: PowPolicy
    }
/**
 * Represents a classification in the `PowPolicy`
 */
export type PowClassification = "Minimum" | "Accepted" | "Personal"

/**
 * -
 */
export interface ForeignWhitelistedResponse {
  classification: PowClassification
  policy: PowPolicy
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
