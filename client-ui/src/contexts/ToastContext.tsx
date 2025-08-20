import React, { createContext, useContext, ReactNode, useState } from "react"
import { Toast, ToastContainer } from "react-bootstrap"

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

export const ToastProvider: React.FC<ToastProviderProps> = ({ children }) => {
  const [toasts, setToasts] = useState<ToastObjectTimed[]>([])

  const removeExpired = () => {
    setToasts((toasts) => {
      const new_toasts: ToastObjectTimed[] = []
      for (const toast of toasts) {
        if (toast.at + 3000 > Date.now()) {
          new_toasts.push(toast)
        }
      }
      return new_toasts
    })
  }

  const showToast = (toast: ToastObject) => {
    setToasts((toasts) => [...toasts, { toast, at: Date.now() }])
    setTimeout(removeExpired, 3200)
  }

  return (
    <ToastContext.Provider value={{ showToast }}>
      <ToastContainer className="position-absolute p-3" position={"bottom-end"}>
        {toasts.map((toast, i) => (
          <Toast
            key={i}
            onClose={() => setToasts(toasts.filter((_, index) => index !== i))}
          >
            <Toast.Header>
              <img
                src="holder.js/20x20?text=%20"
                className="rounded me-2"
                alt=""
              />
              <strong className="me-auto">{toast.toast.header}</strong>
            </Toast.Header>
            <Toast.Body>{toast.toast.body}</Toast.Body>
          </Toast>
        ))}
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
