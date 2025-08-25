import React, {
  createContext,
  useContext,
  ReactNode,
  useState,
  useEffect,
} from "react"
import { ProgressBar } from "react-bootstrap"
import FullscreenCenter from "../components/FullscreenCenter.tsx"
import { invoke } from "@tauri-apps/api/core"
import { useToast } from "./ToastContext.tsx"

const EstimateContext = createContext<EstimateContextType | undefined>(
  undefined
)

interface EstimateContextType {
  estimate: number
}

interface EstimateProviderProps {
  children: ReactNode
}

export const EstimateProvider: React.FC<EstimateProviderProps> = ({
  children,
}) => {
  const [estimate, setEstimate] = useState<number | undefined>(undefined)
  const [seconds, setSeconds] = useState<number>(0)
  const { showToast } = useToast()

  useEffect(() => {
    const id = setInterval(
      () => setSeconds((seconds) => Math.min(seconds + 0.05, 5)),
      50
    )
    return () => clearInterval(id)
  }, [])

  useEffect(() => {
    if (!estimate) {
      invoke("load_estimate").then((sn) => {
        if (sn) {
          setEstimate(sn as number)
          console.log(`Loaded estimate: ${sn}`)
          showToast({
            header: "Loaded Saved Measurement",
            body: "Loaded saved performance measurement. Time estimates may be inaccurate while performance is being remeasured.",
          })
        }

        // Update estimate anyway
        invoke("estimate_performance").then((n) => {
          setEstimate(n as number)
          console.log(`Estimate: ${n}`)
          if (sn) {
            showToast({
              header: "Performance Measurement Completed",
              body: "Completed performance measurement.",
            })
          }
        })
      })
    }
  }, [estimate])

  if (!estimate) {
    return (
      <FullscreenCenter>
        <div>
          <h1 className={"text-center"}>Loading</h1>
          <p className="text-muted">
            Measuring computing performance for time estimates...
          </p>
          <ProgressBar animated now={seconds * 20} />
          <p className={"text-muted text-center mt-3"}>
            Saved measurement will be used next time
          </p>
        </div>
      </FullscreenCenter>
    )
  } else {
    return (
      <EstimateContext.Provider value={{ estimate }}>
        {children}
      </EstimateContext.Provider>
    )
  }
}

export const useEstimate = (): number => {
  const context = useContext(EstimateContext)
  if (!context) {
    throw new Error("useEstimate must be used within an EstimateContext")
  }
  return context.estimate
}
