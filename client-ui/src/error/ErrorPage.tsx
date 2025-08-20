import { useNavigate, useRouteError } from "react-router-dom"
import FullscreenCenter from "../components/FullscreenCenter.tsx"

export default function ErrorPage() {
  const navigate = useNavigate()
  const error = useRouteError()
  console.log(error)

  let statusText = "Unknown error"

  if (error && typeof error === "object") {
    if ("statusText" in error) {
      statusText = String(error.statusText)
    } else if ("message" in error) {
      statusText = String(error.message)
    }
  }

  let errorTitle = "An Error Occurred:"
  if (
    error &&
    typeof error === "object" &&
    "status" in error &&
    error.status === 404
  ) {
    errorTitle = "404 - Page Not Found"
  }

  return (
    <>
      <FullscreenCenter>
        <div className="text-center">
          <h1>{errorTitle}</h1>
          <p>{statusText}</p>
          <p style={{ color: "grey" }}>
            <b>
              <a
                style={{ textDecoration: "none" }}
                href="#"
                onClick={() => navigate(-1)}
              >
                Back
              </a>
            </b>{" "}
            •{" "}
            <b>
              <a
                style={{ textDecoration: "none" }}
                href="#"
                onClick={() => navigate("/", { viewTransition: true })}
              >
                Home
              </a>
            </b>
          </p>
        </div>
      </FullscreenCenter>
    </>
  )
}
