> [!NOTE]
> The specification for how to hash an object is currently defined by the Rust implementation e.g. in the `pow_hash` method in [email.rs](../../interface/src/interface/email.rs#L42). A specification is being worked on.

## General Idea
The general idea is as follows:
1. Get the POW requirement (e.g. [GetCreateAccountPowPolicyRequest](../generated/routes/native/get_create_account_pow_policy/GetCreateAccountPowPolicyRequest.md) / [GetCreateAccountPowPolicyResponse](../generated/routes/native/get_create_account_pow_policy/GetCreateAccountPowPolicyResponse.md)) from the server for some action (e.g. creating an account)
2. Get a generic POW token from the server ([GetPowTokenRequest](../generated/routes/foreign/get_pow_token/GetPowTokenRequest.md) / [GetPowTokenResponse](../generated/routes/foreign/get_pow_token/GetPowTokenResponse.md))
3. Solve the POW token to the requirement
4. Attach the POW token to a request requiring one (e.g. creating an account [CreateAccountRequest](../generated/routes/native/create_account/CreateAccountRequest.md))

## The Maths
The POW token is just one number, $n$. 