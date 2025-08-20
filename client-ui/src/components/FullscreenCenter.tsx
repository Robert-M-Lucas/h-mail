import { ReactNode } from "react"

type Props = {
  children: ReactNode
  insetShadow?: boolean
}

export default function FullscreenCenter({ children, insetShadow }: Props) {
  return (
    <div
      style={{
        width: "100vw",
        height: "100vh",
        boxShadow: insetShadow ? "inset 0 0 20px rgba(0,0,0,0.4)" : "none",
      }}
      className="d-flex justify-content-center align-items-center"
    >
      {children}
    </div>
  )
}
