import { HmailUser } from "../../interface/hmail-user.ts"
import "./on-hover.css"

export interface Props {
  user: HmailUser
  onDelete?: () => void
}

export default function HmailUserText({ user, onDelete }: Props) {
  return (
    <span
      onClick={onDelete}
      className={`
        ${onDelete ? "user-deletable" : ""}
      `}
      style={onDelete ? { cursor: "pointer" } : {}}
    >
      {user.display_name && <>{user.display_name} </>}
      {"<"}
      {user.address}
      {">"}
    </span>
  )
}
