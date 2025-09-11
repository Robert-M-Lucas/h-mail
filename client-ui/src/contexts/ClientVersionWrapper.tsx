import { ReactNode, useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/core"

export interface Props {
  children: ReactNode
}

export default function ClientVersionWrapper({ children }: Props) {
  const [version, setVersion] = useState("v-.-.-")

  useEffect(() => {
    invoke("client_version").then((v) => setVersion(("v" + v) as string))
  }, [])

  return (
    <>
      {children}
      <div
        className="position-fixed border"
        style={{
          left: 0,
          bottom: 0,
          padding: "2px 5px 2px 5px",
          backgroundColor: "white",
          borderRadius: "0 5px 0 0",
        }}
      >
        {version}
      </div>
    </>
  )
}
