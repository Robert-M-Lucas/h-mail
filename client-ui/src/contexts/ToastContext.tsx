import React, { createContext, useContext, ReactNode, useState } from "react"
import { Toast, ToastContainer } from "react-bootstrap"

interface ToastObject {
  header: string
  body: string
}

interface ToastContextType {
  showToast: (toast: ToastObject) => void
}

const ToastContext = createContext<ToastContextType | undefined>(undefined)

interface ToastProviderProps {
  children: ReactNode
}

export const ToastProvider: React.FC<ToastProviderProps> = ({ children }) => {
  const [toasts, setToasts] = useState<ToastObject[]>([])

  const showToast = (toast: ToastObject) => {
    setToasts((toasts) => [...toasts, toast])
  }

  return (
    <ToastContext.Provider value={{ showToast }}>
      {children}
      <ToastContainer className="position-static" position={"bottom-end"}>
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
              <strong className="me-auto">{toast.header}</strong>
            </Toast.Header>
            <Toast.Body>{toast.body}</Toast.Body>
          </Toast>
        ))}
      </ToastContainer>
    </ToastContext.Provider>
  )
}

export const useToast = (): ToastContextType => {
  const context = useContext(ToastContext)
  if (!context) {
    throw new Error("useAuth must be used within a AuthProvider")
  }
  return context
}
