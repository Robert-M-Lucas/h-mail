import { Button, Container, Nav, Tab } from "react-bootstrap"
import { useNavigate } from "react-router-dom"

export default function SettingsPage() {
  const navigate = useNavigate()

  return (
    <Tab.Container defaultActiveKey={"whitelist"}>
      <div className="border-bottom bg-white sticky-top w-100">
        <Container fluid className="py-2 w-100">
          <div className="d-flex align-items-end w-100">
            <Button
              variant="outline-secondary"
              className="px-0 me-3 text-decoration-none"
              style={{ width: "20%" }}
              onClick={() => navigate("/")}
            >
              <span className="me-1" aria-hidden>
                ←
              </span>
              Back
            </Button>
            <Nav variant="tabs" style={{ width: "80%" }} role="tablist">
              <Nav.Item className={"text-center w-50"}>
                <Nav.Link eventKey="whitelist">Whitelist</Nav.Link>
              </Nav.Item>
              <Nav.Item className={"text-center w-50"} style={{ width: "40%" }}>
                <Nav.Link eventKey="pow">POW policy</Nav.Link>
              </Nav.Item>
            </Nav>
          </div>
        </Container>
      </div>

      <Container fluid className="py-3">
        <Tab.Content>
          <Tab.Pane eventKey="whitelist">
            <div className="p-3 border rounded-3">
              Whitelist content goes here.
            </div>
          </Tab.Pane>
          <Tab.Pane eventKey="pow">
            <div className="p-3 border rounded-3">
              POW policy content goes here.
            </div>
          </Tab.Pane>
        </Tab.Content>
      </Container>
    </Tab.Container>
  )
}
