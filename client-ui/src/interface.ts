import { invoke } from "@tauri-apps/api/core"
import { GetHmailsHmail } from "./interface/get-hmails-hmail.ts"
import { SendHmailPackage } from "./interface/send-hmail-package.ts"
import {
  SendHmailResponseAuthed,
  SendHmailResultPerDestination,
} from "./interface/send-hmail-response-authed.ts"
import { HmailUser } from "./interface/hmail-user.ts"

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

export type PowClassification = "MINIMUM" | "ACCEPTED" | "PERSONAL"
export const AllPowClassifications: PowClassification[] = [
  "MINIMUM",
  "ACCEPTED",
  "PERSONAL",
]

export async function sendHmail(
  hmail: SendHmailPackage,
  bccs: HmailUser[],
  logout: () => void
): Promise<SendHmailResultPerDestination[] | undefined> {
  const response: Result<
    AuthResult<SendHmailResponseAuthed>,
    string
  > = parseAuthResponse(await invoke("send_hmail", { hmail, bccs }))

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
  logout: () => void
): Promise<GetHmailsHmail[] | undefined> {
  const response: Result<AuthResult<any>, string> = parseAuthResponse(
    await invoke("get_hmails", { since: 0 })
  )

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }
  return result.value as GetHmailsHmail[]
}

export async function addWhitelist(
  address: string,
  classification: PowClassification,
  logout: () => void
): Promise<boolean | undefined> {
  const response: Result<AuthResult<boolean>, string> = parseAuthResponse(
    await invoke("add_whitelist", { address, classification })
  )

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
  const response: Result<AuthResult<boolean>, string> = parseAuthResponse(
    await invoke("remove_whitelist", { address })
  )

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
  const response: Result<AuthResult<any>, string> = parseAuthResponse(
    await invoke("get_whitelist")
  )

  if (!response.ok) {
    console.error(response.error)
    return undefined
  }
  const result = response.value
  if (!result.ok) {
    logout()
    return undefined
  }
  return result.value as [string, string][]
}

export async function checkAlive(): Promise<boolean> {
  return (await invoke("check_alive")) === "Alive"
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

export async function setServer(server: string): Promise<void> {
  await invoke("set_server", { server })
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
