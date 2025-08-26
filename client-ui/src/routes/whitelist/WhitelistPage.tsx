import { useAuth } from "../../contexts/AuthContext.tsx"
import { useEffect, useState } from "react"
import {
  addWhitelist,
  allPowClassifications,
  getWhitelist,
  PowClassification,
  removeWhitelist,
} from "../../interface.ts"
import { Form } from "react-bootstrap"
import { useNavigate } from "react-router-dom"

export default function WhitelistPage() {
  const { user, logout } = useAuth()
  const navigate = useNavigate()

  const [whitelist, setWhitelist] = useState<[string, string][] | undefined>(
    undefined
  )
  const [toAdd, setToAdd] = useState<string>("")
  const [classification, setClassification] = useState<PowClassification>(
    allPowClassifications[0]
  )

  const updateWhitelist = () => {
    getWhitelist(logout).then((w) => setWhitelist(w))
  }

  useEffect(() => {
    updateWhitelist()
  }, [])

  const removeWhitelistF = (address: string) => {
    removeWhitelist(address, logout).then(() => updateWhitelist())
  }

  const addWhitelistF = () => {
    addWhitelist(toAdd, classification, logout).then(() => updateWhitelist())
  }

  return (
    <>
      <button
        className="btn btn-outline-secondary"
        onClick={() => navigate(-1)}
      >
        Back
      </button>
      <h1>{user.name}'s Whitelist</h1>
      {whitelist &&
        whitelist.map((item, i) => (
          <div key={i}>
            <p>
              {item[0]} - {item[1]}
            </p>
            <button
              className="btn btn-outline-danger"
              onClick={() => removeWhitelistF(item[0])}
            >
              Delete
            </button>
          </div>
        ))}
      <hr />
      <input
        onChange={(e) => setToAdd(e.currentTarget.value)}
        value={toAdd}
      ></input>
      <Form.Select
        onChange={(e) =>
          setClassification(e.currentTarget.value as PowClassification)
        }
        value={classification}
      >
        {allPowClassifications.map((item, i) => (
          <option key={i} value={item}>
            {item}
          </option>
        ))}
      </Form.Select>
      <button
        className="btn btn-outline-primary"
        onClick={() => addWhitelistF()}
      >
        Add
      </button>
    </>
  )
}
