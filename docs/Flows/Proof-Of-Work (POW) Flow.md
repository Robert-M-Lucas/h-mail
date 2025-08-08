> [!NOTE]
> The specification for how to hash an object is currently defined by the Rust implementation e.g. in the `pow_hash` method in [email.rs](../../interface/src/interface/email.rs#L42). A specification is being worked on.

## General Idea
The general idea is as follows:
1. Get the POW requirement (e.g. [GetCreateAccountPowPolicyRequest](../generated/routes/native/get_create_account_pow_policy/GetCreateAccountPowPolicyRequest.md) / [GetCreateAccountPowPolicyResponse](../generated/routes/native/get_create_account_pow_policy/GetCreateAccountPowPolicyResponse.md)) from the server for some action (e.g. creating an account)
2. Get a generic POW token from the server ([GetPowTokenRequest](../generated/routes/foreign/get_pow_token/GetPowTokenRequest.md) / [GetPowTokenResponse](../generated/routes/foreign/get_pow_token/GetPowTokenResponse.md)) at `/get_pow_token`
3. Solve the POW token applied to the hash of the data being sent to the required number of iterations
4. Attach the POW token to a request requiring one (e.g. creating an account [CreateAccountRequest](../generated/routes/native/create_account/CreateAccountRequest.md))

## The Maths
The POW token is just one large number, $n$. A hash ($x$) is then squared modulo $n$ $i$ times to obtain a result.

The server (providing and checking the POW), takes a shortcut using the the two secret primes $p$ and $q$ that multiply to create $n$. 
1. $\phi = (p-1) \times (q-1)$
2. $e = 2^\text{i} \mod{\phi}$
3. Result: $x^e \mod{n}$ .

As such, iterations, $i$, can be increased and results in a proportionally increased solving time for the client, while the server calculates it in a constant time.
