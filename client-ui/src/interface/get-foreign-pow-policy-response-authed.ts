import { PowPolicy } from "./pow-policy.ts"

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
