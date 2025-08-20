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

  useEffect(() => {
    if (!estimate) {
      invoke("estimate_performance").then((n) => {
        setEstimate(n as number)
        console.log(`Estimate: ${n}`)
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
          <ProgressBar animated now={100} />
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
    throw new Error("useEstimate must be used within an EstimateProvider")
  }
  return context.estimate
}
