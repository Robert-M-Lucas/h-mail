import {
  Button,
  Container,
  FormControl,
  FormSelect,
  Nav,
  Spinner,
  Tab,
  Table,
} from "react-bootstrap"
import { useNavigate } from "react-router-dom"
import { Check, Trash, X } from "react-bootstrap-icons"
import { useEffect, useState } from "react"
import {
  addWhitelist,
  allPowClassifications,
  getWhitelist,
  PowClassification,
  removeWhitelist,
} from "../../interface.ts"
import { useAuth } from "../../contexts/AuthContext.tsx"
import { useToast } from "../../contexts/ToastContext.tsx"
import { invoke } from "@tauri-apps/api/core"

export default function SettingsPage() {
  const navigate = useNavigate()
  const { logout } = useAuth()
  const { showToast } = useToast()

  const [whitelist, setWhitelist] = useState<[string, string][] | undefined>(
    undefined
  )
  const [newWhitelistUser, setNewWhitelistUser] = useState<string>("")
  const [newClassification, setNewClassification] = useState<PowClassification>(
    allPowClassifications[0]
  )
  const updateWhitelist = async () => {
    setWhitelist(undefined)
    const whitelist = await getWhitelist(logout)
    if (!whitelist) {
      showToast({
        header: "Failed to Fetch Whitelist",
        body: "Failed to fetch whitelist.",
      })
      return
    }
    setWhitelist(whitelist)
  }

  const removeWhitelistF = async (to_remove: string) => {
    setWhitelist(undefined)
    if (
      !(await removeWhitelist(to_remove, () => {
        showToast({
          header: "Failed to Remove from Whitelist",
          body: "Authentication failed.",
        })
        logout()
      }))
    ) {
      showToast({
        header: "Failed to Remove User from Whitelist",
        body: "Failed to remove user from whitelist.",
      })
    }
    await updateWhitelist()
  }

  const addWhitelistF = async () => {
    if (!(await invoke("validate_hmail", { address: newWhitelistUser }))) {
      showToast({
        header: "Couldn't Add User to Whitelist",
        body: "Invalid h-mail address",
      })
      return
    }
    setWhitelist(undefined)
    if (
      !(await addWhitelist(newWhitelistUser, newClassification, () => {
        showToast({
          header: "Couldn't Add User to Whitelist",
          body: "Authentication failed.",
        })
        logout()
      }))
    ) {
      showToast({
        header: "Couldn't Add User to Whitelist",
        body: "Couldn't add user to whitelist.",
      })
    } else {
      setNewWhitelistUser("")
      setNewClassification(allPowClassifications[0])
    }
    await updateWhitelist()
  }

  useEffect(() => {
    updateWhitelist().then()
  }, [])

  return (
    <>
      <a className={"m-0 p-0"} href={"#"} onClick={() => navigate("/")}>
        <X /> Close
      </a>
      <Tab.Container defaultActiveKey={"whitelist"}>
        <div className="bg-white sticky-top w-100">
          <Nav variant="tabs" role="tablist">
            <Nav.Item className={"text-center w-50"}>
              <Nav.Link eventKey="whitelist">Whitelist</Nav.Link>
            </Nav.Item>
            <Nav.Item className={"text-center w-50"} style={{ width: "40%" }}>
              <Nav.Link eventKey="pow">POW policy</Nav.Link>
            </Nav.Item>
          </Nav>
        </div>

        <Container fluid className="py-3">
          <Tab.Content>
            <Tab.Pane eventKey="whitelist">
              <Container>
                {whitelist ? (
                  <>
                    <h1>Whitelist</h1>
                    <Table className={"align-middle"} striped bordered hover>
                      <thead>
                        <tr>
                          <th>#</th>
                          <th className={"ps-3"}>Address</th>
                          <th className={"ps-3"}>Classification</th>
                          <th>Remove</th>
                        </tr>
                      </thead>
                      <tbody>
                        {whitelist.map(([user, classification], i) => (
                          <tr>
                            <td>{i + 1}</td>
                            <td className={"ps-3"}>{user}</td>
                            <td className={"ps-3"}>{classification}</td>
                            <td>
                              <Button
                                size={"sm"}
                                variant={"danger w-100"}
                                onClick={async () => {
                                  await removeWhitelistF(user)
                                }}
                              >
                                <Trash />
                              </Button>
                            </td>
                          </tr>
                        ))}
                        <tr>
                          <td>{whitelist.length + 1}</td>
                          <td>
                            <FormControl
                              size={"sm"}
                              value={newWhitelistUser}
                              onChange={(e) =>
                                setNewWhitelistUser(e.target.value)
                              }
                            />
                          </td>
                          <td>
                            <FormSelect size={"sm"}>
                              {allPowClassifications.map((c, i) => (
                                <option value={c} key={i}>
                                  {c}
                                </option>
                              ))}
                            </FormSelect>
                          </td>
                          <td>
                            <Button
                              variant={"success w-100"}
                              onClick={async () => {
                                await addWhitelistF()
                              }}
                            >
                              <Check />
                            </Button>
                          </td>
                        </tr>
                      </tbody>
                    </Table>
                  </>
                ) : (
                  <div className={"w-100 text-center mt-5"}>
                    <Spinner />
                  </div>
                )}
              </Container>
            </Tab.Pane>
            <Tab.Pane eventKey="pow">
              <div className="p-3 border rounded-3">
                POW policy content goes here.
              </div>
            </Tab.Pane>
          </Tab.Content>
        </Container>
      </Tab.Container>
    </>
  )
}
