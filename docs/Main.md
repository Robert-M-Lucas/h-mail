# 1. Client Requests POW

1. Client requests POW token from destination server, specifying the source and destination email address
	- The source email is unverified here
2. Server replies with POW token tied to email address. Also specifies POW requirements.

# 2. Client Sends Email
1. Client sends email with POW to own server
2. Own server sends email with POW to destination server
3. Destination server can reply with reason for rejection

# 3. Server Checks Whitelist
Whitelist allows emails to be sent with no POW

1. Own server asks destination server if own server is on whitelist
	- Must be own server, not client, to only allow checking if owned domain is on whitelist
2. Destination server responds with classification

> Maybe allow client to do this


