import React, { createContext, useContext, ReactNode, useState } from "react"
import { Spinner } from "react-bootstrap"

interface LockoutContextType {
  enterLockout: () => void
  exitLockout: () => void
}

const LockoutContext = createContext<LockoutContextType | undefined>(undefined)

interface LockoutProviderProps {
  children: ReactNode
}

export const LockoutProvider: React.FC<LockoutProviderProps> = ({
  children,
}) => {
  const [lockouts, setLockouts] = useState<number>(0)

  const enterLockout = () => {
    setLockouts((lockouts) => lockouts + 1)
  }

  const exitLockout = () => {
    setLockouts((lockouts) => lockouts - 1)
  }

  if (lockouts >= 0) {
    return (
      <LockoutContext.Provider value={{ enterLockout, exitLockout }}>
        {lockouts > 0 && (
          <div
            aria-live="polite"
            aria-busy="true"
            className="position-fixed top-0 start-0 w-100 h-100 d-flex align-items-center justify-content-center"
            style={{
              background: "rgba(0,0,0,0.35)",
              zIndex: 1050,
              pointerEvents: "auto",
            }}
          >
            <div className="text-center">
              <Spinner animation="border" color="white" role="status">
                <span className="visually-hidden">Loading…</span>
              </Spinner>
            </div>
          </div>
        )}
        {children}
      </LockoutContext.Provider>
    )
  } else {
    throw new Error("Negative lockout count")
  }
}

export const useLockout = (): LockoutContextType => {
  const context = useContext(LockoutContext)
  if (!context) {
    throw new Error("useLockout must be used within a LockoutProvider")
  }
  return context
}
