# Request

| Name          | Type             | Additional Info                                          |
| ------------- | ---------------- | -------------------------------------------------------- |
| source_user   | String           | Adhering to what a user can be in a normal email address |
| source_domain | String           | Must have SPF                                            |
| email         | String           |                                                          |
| iters         | u64              | Iterations of POW completed                              |
| token         | [[BigUintField]] | POW token                                                |
| pow_result    | [[BigUintField]] | Result of POW                                            |
| destination   | String           | Adhering to what a user can be in a normal email address |


# Response


| Variant           | Extra Data           | Additional Info  |
| ----------------- | -------------------- | ---------------- |
| Success           |                      |                  |
| UserNotFound      |                      |                  |
| DoesNotMeetPolicy | [[PowPolicy]]        |                  |
| PowFailure        | [[PowFailureReason]] |                  |
| SenderIpNotAuthed |                      | SPF check failed |
