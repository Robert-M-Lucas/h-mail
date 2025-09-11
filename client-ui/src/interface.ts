import { invoke } from "@tauri-apps/api/core"
import { GetHmailsHmail } from "./interface/get-hmails-hmail.ts"
import { SendHmailPackage } from "./interface/send-hmail-package.ts"
import {
  SendHmailResponseAuthed,
  SendHmailResultPerDestination,
} from "./interface/send-hmail-response-authed.ts"
import { HmailUser } from "./interface/hmail-user.ts"
import { GetForeignPowPolicyResponseAuthed } from "./interface/get-foreign-pow-policy-response-authed.ts"
import { GetPowPolicyResponseAuthed } from "./interface/get-pow-policy-response-authed.ts"
import { PowPolicy } from "./interface/pow-policy.ts"

export type Ok<T> = {
  ok: true
  value: T
}

export type Err<E> = {
  ok: false
  error: E
}

export type Result<T, E> = Ok<T> | Err<E>
export type ReqResult<T> = Ok<T> | Err<string>

export type AuthErr = {
  ok: false
}

export type AuthResult<T> = Ok<T> | AuthErr

export const guidlineItersPerSecond = 6500

export type PowClassification = "Minimum" | "Accepted" | "Personal"
export const allPowClassifications: PowClassification[] = [
  "Minimum",
  "Accepted",
  "Personal",
]

export async function setPowPolicy(
  policy: PowPolicy,
  logout: () => void
): Promise<boolean> {
  let response: Result<AuthResult<any>, string>
  try {
    response = parseAuthResponse(await invoke("set_pow_policy", { policy }))
  } catch (error) {
    console.error(error)
    return false
  }

  if (!response.ok) {
    console.error(response.error)
    return false
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return false
  }
  return true
}

export async function getPowPolicy(
  logout: () => void
): Promise<PowPolicy | undefined> {
  let response: Result<AuthResult<GetPowPolicyResponseAuthed>, string>
  try {
    response = parseAuthResponse(await invoke("get_pow_policy"))
  } catch (error) {
    console.error(error)
    return undefined
  }

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }
  return result.value.policy
}

export async function getForeignPowPolicy(
  recipient: string,
  logout: () => void
): Promise<GetForeignPowPolicyResponseAuthed | undefined> {
  let response: Result<AuthResult<GetForeignPowPolicyResponseAuthed>, string>
  try {
    response = parseAuthResponse(
      await invoke("get_foreign_pow_policy", { recipient })
    )
  } catch (error) {
    console.error(error)
    return undefined
  }

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }
  return result.value
}

export async function sendHmail(
  hmail: SendHmailPackage,
  bccs: HmailUser[],
  classifications: [string, PowClassification][],
  logout: () => void
): Promise<SendHmailResultPerDestination[] | undefined> {
  let response: Result<AuthResult<SendHmailResponseAuthed>, string>
  try {
    response = parseAuthResponse(
      await invoke("send_hmail", { hmail, bccs, classifications })
    )
  } catch (error) {
    console.error(error)
    return undefined
  }
  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }

  const value = result.value
  if (typeof value === "object" && "DeliverResponse" in value) {
    return value.DeliverResponse
  }
  console.error(value)
  return undefined
}

export async function getHmails(
  until: number | undefined,
  limit: number,
  outbox: boolean,
  logout: () => void
): Promise<GetHmailsHmail[] | undefined> {
  let response: Result<AuthResult<GetHmailsHmail[]>, string>
  try {
    response = parseAuthResponse(
      await invoke("get_hmails", { until, limit, outbox })
    )
  } catch (error) {
    console.error(error)
    return undefined
  }

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }
  return result.value
}

export async function getHmailByHash(
  hash: string,
  logout: () => void
): Promise<GetHmailsHmail | undefined> {
  let response: Result<AuthResult<GetHmailsHmail | undefined | null>, string>
  try {
    response = parseAuthResponse(await invoke("get_hmail_by_hash", { hash }))
  } catch (error) {
    console.error(error)
    return undefined
  }

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }
  if (result.value) {
    return result.value as GetHmailsHmail
  } else {
    return undefined
  }
}

export async function addWhitelist(
  address: string,
  classification: PowClassification,
  logout: () => void
): Promise<boolean | undefined> {
  let response: Result<AuthResult<boolean>, string>
  try {
    response = parseAuthResponse(
      await invoke("add_whitelist", { address, classification })
    )
  } catch (error) {
    console.error(error)
    return undefined
  }

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }
  return result.value
}

export async function removeWhitelist(
  address: string,
  logout: () => void
): Promise<boolean | undefined> {
  let response: Result<AuthResult<boolean>, string>
  try {
    response = parseAuthResponse(await invoke("remove_whitelist", { address }))
  } catch (error) {
    console.error(error)
    return undefined
  }

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }
  return result.value
}

export async function getWhitelist(
  logout: () => void
): Promise<[string, string][] | undefined> {
  let response: Result<AuthResult<[string, string][]>, string>
  try {
    response = parseAuthResponse(await invoke("get_whitelist"))
  } catch (error) {
    console.error(error)
    return undefined
  }

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }
  return result.value
}

export async function checkAlive(): Promise<string> {
  return await invoke("check_alive")
}

export async function reauthenticate(
  username: string,
  password: string
): Promise<Result<string, string>> {
  return parseResponse(await invoke("reauthenticate", { username, password }))
}

export async function createAccount(
  username: string,
  password: string
): Promise<Result<string, string>> {
  return parseResponse(await invoke("create_account", { username, password }))
}

export async function createAccountRequirement(): Promise<
  Result<number, string>
> {
  return parseResponse(await invoke("create_account_requirement"))
}

export async function checkAuth(): Promise<string | undefined> {
  const response: Result<AuthResult<string>, string> = parseAuthResponse(
    await invoke("check_auth")
  )

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    return undefined
  }
  return result.value
}

export async function getServer(): Promise<string | undefined> {
  const response: ReqResult<string> = parseResponse(await invoke("get_server"))
  if (!response.ok) {
    return undefined
  }
  return response.value
}

export async function setServer(server: string): Promise<string | undefined> {
  const e: string | undefined | null = await invoke("set_server", { server })
  if (e) {
    return e
  }
  return undefined
}

function parseAuthResponse<T>(response: any): ReqResult<AuthResult<T>> {
  const result: ReqResult<any> = parseResponse(response)
  if (!result.ok) {
    return result
  }

  const auth_result = result.value
  if (auth_result === "Unauthorized") {
    return {
      ok: true,
      value: {
        ok: false,
      },
    }
  }
  return {
    ok: true,
    value: {
      ok: true,
      value: auth_result["Success"],
    },
  }
}

function parseResponse(response: any): ReqResult<any> {
  console.log(response)
  if ("Ok" in response) {
    return {
      ok: true,
      value: response["Ok"],
    }
  } else {
    // alert(`Backend Error: ${response["Err"]}`)
    return {
      ok: false,
      error: response["Err"] as string,
    }
  }
}
