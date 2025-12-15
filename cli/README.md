<br /><br /><br />

<h1 align="center">Email Validator CLI</h1>
<h4 align="center">Email verification from your terminal.</h4>

<br /><br /><br />

> Note: The CLI binary doesn't connect to any backend, it checks the email directly from your computer.

## Usage

Head to this repository's Releases page and download the binary for your platform.

Then run:

```bash
> $ email_validator --help
email-validator-cli
Check if an email address exists without sending any email.

USAGE:
    email_validator [OPTIONS] <TO_EMAIL>

ARGS:
    <TO_EMAIL>    The email to check

OPTIONS:
        --check-gravatar <CHECK_GRAVATAR>
            Whether to check if a gravatar image is existing for the given email [env:
            CHECK_GRAVATAR=] [default: false]

        --from-email <FROM_EMAIL>
            The email to use in the `MAIL FROM:` SMTP command [env: FROM_EMAIL=] [default:
            noreply@example.com]

        --gmail-verif-method <GMAIL_VERIF_METHOD>
            Select how to verify Gmail email addresses: api or smtp [env: GMAIL_VERIF_METHOD=]
            [default: smtp]

    -h, --help
            Print help information

        --haveibeenpwned-api-key <HAVEIBEENPWNED_API_KEY>
            HaveIBeenPnwed API key, ignore if not provided [env: HAVEIBEENPWNED_API_KEY=]

        --hello-name <HELLO_NAME>
            The name to use in the `EHLO:` SMTP command [env: HELLO_NAME=] [default: gmail.com]

        --hotmailb2b-verif-method <HOTMAILB2B_VERIF_METHOD>
            Select how to verify Hotmail B2B email addresses: smtp [env: HOTMAILB2B_VERIF_METHOD=]
            [default: smtp]

        --hotmailb2c-verif-method <HOTMAILB2C_VERIF_METHOD>
            Select how to verify Hotmail B2C email addresses: headless or smtp [env:
            HOTMAILB2C_VERIF_METHOD=] [default: headless]

        --proxy-host <PROXY_HOST>
            Use the specified SOCKS5 proxy host to perform email verification [env: PROXY_HOST=]

        --proxy-password <PROXY_PASSWORD>
            Username passed to the specified SOCKS5 proxy port to perform email verification. Only
            used when `--proxy-host` flag is set [env: PROXY_PASSWORD=]

        --proxy-port <PROXY_PORT>
            Use the specified SOCKS5 proxy port to perform email verification. Only used when
            `--proxy-host` flag is set [env: PROXY_PORT=] [default: 1080]

        --proxy-username <PROXY_USERNAME>
            Username passed to the specified SOCKS5 proxy port to perform email verification. Only
            used when `--proxy-host` flag is set [env: PROXY_USERNAME=]

        --smtp-port <SMTP_PORT>
            The port to use for the SMTP request [env: SMTP_PORT=] [default: 25]

    -V, --version
            Print version information

        --yahoo-verif-method <YAHOO_VERIF_METHOD>
            Select how to verify Yahoo email addresses: api, headless or smtp [env:
            YAHOO_VERIF_METHOD=] [default: headless]

```

**💡 PRO TIP:** To show debug logs when running the binary, run:

```bash
RUST_LOG=debug email_validator
```

## Build From Source

First, [install Rust](https://www.rust-lang.org/tools/install); you'll need Rust 1.37.0 or later. Then, run the following commands:

```bash
# Download the code
$ git clone https://github.com/yourorg/email-validator
$ cd email-validator

# Build in release mode
$ cargo build --release

# Run the binary
$ ./target/release/email_validator --help
```

## Legacy Bash Script

The 1st version of this tool was a simple bash script that made a telnet call.
