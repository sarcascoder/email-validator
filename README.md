<h1 align="center">Email Validator</h1>
<h4 align="center">Verify if an email address exists without sending any email.<br/>Includes a ⚙️ self-hosted HTTP API backend and a CLI.</h4>


## Quick start

### Run the HTTP backend with Docker

Outbound port 25 must be open for SMTP checks.

```bash
docker run -p 8080:8080 yourorg/email-validator-backend:latest
```

Then send a POST to `http://localhost:8080/v1/check_email` with a body like:

```jsonc
{
	"to_email": "someone@gmail.com",
	"proxy": {                 // optional SOCKS5 proxy
		"host": "my-proxy.io",
		"port": 1080
	}
}
```

See the backend docs in `./backend` for configuration options (env vars prefixed with `EV__`).

### CLI

Download or build the CLI in `./cli` and run:

```bash
email_validator --help
```

The CLI checks emails directly from your machine; it does not require the backend.

## ✈️ JSON Output

The output will be a JSON with the below format, the fields should be self-explanatory. For `someone@gmail.com` (note that it is disabled by Gmail), here's the exact output:

```json
{
	"input": "someone@gmail.com",
	"is_reachable": "invalid",
	"misc": {
		"is_disposable": false,
		"is_role_account": false,
		"is_b2c": true
	},
	"mx": {
		"accepts_mail": true,
		"records": [
			"alt3.gmail-smtp-in.l.google.com.",
			"gmail-smtp-in.l.google.com.",
			"alt1.gmail-smtp-in.l.google.com.",
			"alt4.gmail-smtp-in.l.google.com.",
			"alt2.gmail-smtp-in.l.google.com."
		]
	},
	"smtp": {
		"can_connect_smtp": true,
		"has_full_inbox": false,
		"is_catch_all": false,
		"is_deliverable": false,
		"is_disabled": true
	},
	"syntax": {
		"domain": "gmail.com",
		"is_valid_syntax": true,
		"username": "someone",
		"suggestion": null
	}
}
```

## What Does This Tool Check?

| Included? | Feature                                       | Description                                                                                                                     | JSON field                                                                |
| --------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| ✅        | **Email reachability**                        | How confident are we in sending an email to this address? Can be one of `safe`, `risky`, `invalid` or `unknown`.                | `is_reachable`                                                            |
| ✅        | **Syntax validation**                         | Is the address syntactically valid?                                                                                             | `syntax.is_valid_syntax`                                                  |
| ✅        | **DNS records validation**                    | Does the domain of the email address have valid MX DNS records?                                                                 | `mx.accepts_mail`                                                         |
| ✅        | **Disposable email address (DEA) validation** | Is the address provided by a known [disposable email address](https://en.wikipedia.org/wiki/Disposable_email_address) provider? | `misc.is_disposable`                                                      |
| ✅        | **SMTP server validation**                    | Can the mail exchanger of the email address domain be contacted successfully?                                                   | `smtp.can_connect_smtp`                                                   |
| ✅        | **Email deliverability**                      | Is an email sent to this address deliverable?                                                                                   | `smtp.is_deliverable`                                                     |
| ✅        | **Mailbox disabled**                          | Has this email address been disabled by the email provider?                                                                     | `smtp.is_disabled`                                                        |
| ✅        | **Full inbox**                                | Is the inbox of this mailbox full?                                                                                              | `smtp.has_full_inbox`                                                     |
| ✅        | **Catch-all address**                         | Is this email address a [catch-all](https://debounce.io/blog/help/what-is-a-catch-all-or-accept-all/) address?                  | `smtp.is_catch_all`                                                       |
| ✅        | **Role account validation**                   | Is the email address a well-known role account?                                                                                 | `misc.is_role_account`                                                    |
| ✅        | **Gravatar Url**                              | The url of the [Gravatar](https://gravatar.com/) email address profile picture                                                  | `misc.gravatar_url`                                                       |
| ✅        | **Have I Been Pwned?**                        | Has this email been compromised in a [data breach](https://haveibeenpwned.com/)?                                                | `misc.haveibeenpwned`                                                     |
| 🔜        | **Free email provider check**                 | Is the email address bound to a known free email provider?                                                                      | - |
| 🔜        | **Syntax validation, provider-specific**      | According to the syntactic rules of the target mail provider, is the address syntactically valid?                               | - |
| 🔜        | **Honeypot detection**                        | Does email address under test hide a [honeypot](https://en.wikipedia.org/wiki/Spamtrap)?                                        | - |

## 🤔 Why?

Many online services (https://hunter.io, https://verify-email.org, https://email-checker.net) offer this service for a paid fee. Here is an open-source alternative to those tools.

## License

This project is licensed under the [AGPL-3.0](./LICENSE.AGPL).

## 🔨 Build From Source

Build the [CLI from source](./cli/README.md#build-from-source) or the [HTTP backend from source](./backend/README.md#build-from-source).
