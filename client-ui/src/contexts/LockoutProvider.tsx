import React, {
  createContext,
  useContext,
  ReactNode,
  useState,
  useRef,
  useEffect,
} from "react"
import { Spinner } from "react-bootstrap"
import { AnimatePresence, motion } from "framer-motion"

interface LockoutContextType {
  enterLockout: () => void
  exitLockout: () => void
}

const LockoutContext = createContext<LockoutContextType | undefined>(undefined)

interface LockoutProviderProps {
  children: ReactNode
}

const MIN_LOCKOUT = 400

export const LockoutProvider: React.FC<LockoutProviderProps> = ({
  children,
}) => {
  const [lockouts, setLockouts] = useState<number>(0)
  const [, setFlip] = useState<boolean>(false)
  const lastLock = useRef<number>(0)

  const enterLockout = () => {
    setLockouts((lockouts) => {
      if (lockouts === 0) {
        lastLock.current = Date.now()
      }
      return lockouts + 1
    })
  }

  useEffect(() => {
    let timeout = undefined
    if (lockouts === 0 && Date.now() - lastLock.current < MIN_LOCKOUT) {
      console.log("short")
      timeout = setTimeout(
        () => setFlip((flip) => !flip),
        MIN_LOCKOUT + 50 - (Date.now() - lastLock.current)
      )
    }
    return () => {
      if (timeout) {
        clearTimeout(timeout)
      }
    }
  }, [lockouts])

  let show = lockouts !== 0
  if (lockouts === 0 && Date.now() - lastLock.current < MIN_LOCKOUT) {
    show = true
  }

  const exitLockout = () => {
    setLockouts((lockouts) => lockouts - 1)
  }

  if (lockouts >= 0) {
    return (
      <LockoutContext.Provider value={{ enterLockout, exitLockout }}>
        <AnimatePresence>
          {show && (
            <motion.div
              aria-live="polite"
              aria-busy="true"
              className="position-fixed top-0 start-0 w-100 h-100 d-flex align-items-center justify-content-center"
              style={{
                background: "rgba(0,0,0,0.35)",
                zIndex: 1050,
                pointerEvents: "auto",
              }}
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.3 }}
            >
              <div className="text-center">
                <Spinner animation="border" color="white" role="status">
                  <span className="visually-hidden">Loading...</span>
                </Spinner>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
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
