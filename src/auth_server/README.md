## What it does?
This server has two endpoints
GET /generate_dropbox_auth
GET /exchange_code

First one redirects requester to dropbox authorization page
Second gets dropbox code (obtained from authorization) from Header X-Dropbox-Code and exchanges it for token.

## Ok, but why?

Previously, client had secrets stored in his binary which is a very bad thing.
Now, it sends user to /generate_dropbox_auth in order to get token and then /exchange_code for exchaning it.