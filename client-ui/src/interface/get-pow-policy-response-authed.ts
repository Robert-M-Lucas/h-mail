import { PowPolicy } from "./pow-policy.ts"

/**
 * Returns the authenticated user's POW policy (note that the data returned is not secret)
 */
export interface GetPowPolicyResponseAuthed {
  policy: PowPolicy
  [k: string]: unknown
}
