import { HmailUser } from "../interface/hmail-user.ts"

export interface Props {
  user: HmailUser
}

export default function HmailUserText({ user }: Props) {
  return (
    <>
      {user.display_name && <>{user.display_name} </>}
      {"<"}
      {user.address}
      {">"}
    </>
  )
}
