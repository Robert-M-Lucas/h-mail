import { Form, InputGroup } from "react-bootstrap"
import { useEstimate } from "../../contexts/EstimateContext.tsx"
import { guidlineItersPerSecond } from "../../interface.ts"

function sToTime(duration: number): string {
  const seconds = Math.floor(duration % 60)
  const minutes = Math.floor(duration / 60)

  const mm = String(minutes).padStart(2, "0")
  const ss = String(seconds).padStart(2, "0")

  return `${mm}m${ss}s`
}

export interface Props {
  title: string
  currentValue: number
  value: number
  setValue: (value: number) => void
}

export default function PowFormComponent({
  title,
  currentValue,
  value,
  setValue,
}: Props) {
  const estimate = useEstimate()
  return (
    <div className={"mb-3"}>
      <Form.Label>{title}</Form.Label>
      <InputGroup>
        <Form.Control
          value={value.toLocaleString("en-US")}
          placeholder={"Enter " + title.toLowerCase() + " for policy"}
          onChange={(e) => {
            let nv = e.target.value.replace(/\D/g, "")
            if (nv.length === 0) nv = "0"
            let ni = parseInt(nv)
            if (isNaN(ni)) {
              return
            }

            setValue(ni)
          }}
        />
        <InputGroup.Text>Your PC: {sToTime(value / estimate)}</InputGroup.Text>
        <InputGroup.Text>
          Average Pc: {sToTime(value / guidlineItersPerSecond)}
        </InputGroup.Text>
      </InputGroup>
      <Form.Text>
        Current value: {currentValue.toLocaleString("en-US")}
      </Form.Text>
    </div>
  )
}
