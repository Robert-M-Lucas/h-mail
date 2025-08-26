import { Button, Container, Navbar } from "react-bootstrap"
import { BoxArrowRight, Gear } from "react-bootstrap-icons"
import { useNavigate } from "react-router-dom"
import { AuthInfo } from "../contexts/AuthContext.tsx"

export interface Props {
  user: AuthInfo
  logout: () => void
}

export default function InboxHeader({ user, logout }: Props) {
  const navigate = useNavigate()

  return (
    <div>
      <Navbar bg="light" className="px-3">
        <Container fluid>
          <Navbar.Brand className="fw-bold fs-4 me-5">
            {user.domain}
          </Navbar.Brand>

          <div className="d-flex align-items-center gap-3 ms-2">
            <span className="fw-semibold">{user.name}</span>
            <Button
              variant="outline-danger"
              size="sm"
              className="d-flex align-items-center"
              onClick={logout}
            >
              <BoxArrowRight className="me-1" size={18} />
              Logout
            </Button>
            <Button
              variant="outline-secondary"
              size="sm"
              className="d-flex align-items-center"
              onClick={() => navigate("/settings", { viewTransition: true })}
            >
              <Gear className="me-1" size={18} />
              Settings
            </Button>
          </div>
        </Container>
      </Navbar>
      <hr className={"mt-0"} />
    </div>
  )
}
