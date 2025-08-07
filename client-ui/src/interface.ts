import { invoke } from "@tauri-apps/api/core"

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
    return {
      ok: false,
      error: response["Err"] as string,
    }
  }
}
