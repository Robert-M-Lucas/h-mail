import React, { createContext, useContext, ReactNode, useState } from "react"
import { Toast, ToastContainer } from "react-bootstrap"
import { AnimatePresence, motion } from "framer-motion"
import { InfoCircle } from "react-bootstrap-icons"

interface ToastObject {
  header: string
  body: string
}

interface ToastObjectTimed {
  toast: ToastObject
  at: number
}

interface ToastContextType {
  showToast: (toast: ToastObject) => void
}

const ToastContext = createContext<ToastContextType | undefined>(undefined)

interface ToastProviderProps {
  children: ReactNode
}

const TOAST_LIFETIME = 5000

export const ToastProvider: React.FC<ToastProviderProps> = ({ children }) => {
  const [toasts, setToasts] = useState<ToastObjectTimed[]>([])

  const removeExpired = () => {
    setToasts((toasts) => {
      const new_toasts: ToastObjectTimed[] = []
      for (const toast of toasts) {
        if (toast.at + TOAST_LIFETIME > Date.now()) {
          new_toasts.push(toast)
        }
      }
      return new_toasts
    })
  }

  const showToast = (toast: ToastObject) => {
    setToasts((toasts) => [...toasts, { toast, at: Date.now() }])
    setTimeout(removeExpired, TOAST_LIFETIME + 50)
  }

  return (
    <ToastContext.Provider value={{ showToast }}>
      <ToastContainer className="position-fixed p-3" position={"bottom-end"}>
        <AnimatePresence>
          {toasts.map((toast, i) => (
            <motion.div
              className={"mt-3"}
              key={i}
              layout
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.3, ease: "easeInOut" }}
            >
              <Toast
                onClose={() =>
                  setToasts(toasts.filter((_, index) => index !== i))
                }
              >
                <Toast.Header>
                  <InfoCircle className={"me-2"} />
                  <strong className="me-auto">{toast.toast.header}</strong>
                </Toast.Header>
                <Toast.Body>{toast.toast.body}</Toast.Body>
              </Toast>
            </motion.div>
          ))}
        </AnimatePresence>
      </ToastContainer>
      {children}
    </ToastContext.Provider>
  )
}

export const useToast = (): ToastContextType => {
  const context = useContext(ToastContext)
  if (!context) {
    throw new Error("useToast must be used within a ToastProvider")
  }
  return context
}
