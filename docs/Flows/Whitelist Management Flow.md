*Does not need to be implemented to create a h-mail compatible server*

- Use [GetWhitelistRequest](../generated/routes/native/get_whitelist/GetWhitelistRequest.md) to get the whitelist for the current [authenticated](Authentication%20Flow.md) user
- Use [AddWhitelistRequest](../generated/routes/native/add_whitelist/AddWhitelistRequest.md) to add an address to the whitelist, and specify the category its emails should go in
- Use [RemoveWhitelistRequest](../generated/routes/native/remove_whitelist/RemoveWhitelistRequest.md) to remove an address from the whitelist