*Does not need to be implemented to create a h-mail compatible server*

## General Idea
1. Get create account proof-of-work requirement (this may be quite high as this is done only once per user) - [GetCreateAccountPowPolicyRequest](../generated/routes/native/get_create_account_pow_policy/GetCreateAccountPowPolicyRequest.md)
2. Solve the POW requirement
3. Send a [CreateAccountRequest](../generated/routes/native/create_account/CreateAccountRequest.md)
4. [Authenticate](Authentication%20Flow.md) normally with the new account