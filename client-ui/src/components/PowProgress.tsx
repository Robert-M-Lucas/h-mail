import { Dispatch, ReactNode, SetStateAction, useState } from "react"
import { listen } from "@tauri-apps/api/event"
import { Button, Modal, ProgressBar } from "react-bootstrap"
import { invoke } from "@tauri-apps/api/core"

interface Props {
  children: ReactNode
}

let gSetPowProgress:
  | Dispatch<SetStateAction<ProgressDetails | undefined>>
  | undefined = undefined

interface ProgressDetails {
  progress: number
  out_of: number
  text: string
}

listen<string>("pow-progress", (event) => {
  if (gSetPowProgress) {
    const p = event.payload
    if (p.length === 0) {
      gSetPowProgress(undefined)
    } else {
      const [start, text] = p.split("$")
      const [progress, out_of] = start.split("#")
      gSetPowProgress({
        progress: parseInt(progress),
        out_of: parseInt(out_of),
        text: text,
      })
    }
  }
})

export default function PowProgress({ children }: Props) {
  const [powProgress, setPowProgress] = useState<ProgressDetails | undefined>(
    undefined
  )

  gSetPowProgress = setPowProgress

  return (
    <div>
      <Modal show={powProgress !== undefined} centered size="lg">
        <Modal.Header>
          <Modal.Title>Solving Proof-of-Work</Modal.Title>
        </Modal.Header>
        {powProgress && (
          <>
            <Modal.Body>
              <ProgressBar
                animated
                now={(powProgress.progress / powProgress.out_of) * 100}
                label={`${powProgress.progress.toLocaleString("en-US")} / ${powProgress.out_of.toLocaleString("en-US")}`}
              />
              <div className="w-100 text-center">{powProgress.text}</div>
            </Modal.Body>
            <Modal.Footer>
              <Button
                className={"w-100"}
                variant={"danger"}
                onClick={async () => await invoke("cancel_current_pow")}
              >
                Cancel
              </Button>
            </Modal.Footer>
          </>
        )}
      </Modal>
      {children}
    </div>
  )
}
