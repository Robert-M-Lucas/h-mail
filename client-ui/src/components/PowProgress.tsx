import { Dispatch, ReactNode, SetStateAction, useState } from "react"
import { listen } from "@tauri-apps/api/event"

interface Props {
  children: ReactNode
}

let gSetPowProgress: Dispatch<SetStateAction<string>> | undefined = undefined

listen<string>("pow-progress", (event) => {
  if (gSetPowProgress) {
    gSetPowProgress(event.payload)
  }
})

export default function PowProgress({ children }: Props) {
  const [powProgress, setPowProgress] = useState<string>("-")

  gSetPowProgress = setPowProgress

  return (
    <div>
      {children}
      <div style={{ position: "absolute", bottom: "0" }}>{powProgress}</div>
    </div>
  )
}
