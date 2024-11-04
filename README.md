# emailer

This project is a Rust program that sends an email using IMAP. Epson inkjet printers can be prone to clogging if unused, so this project sends email to print to Epson Email Print online service. The service connects back to my printer and prints the attachments and email body message. 

# Usage

To run the rust binary, set the following environment variables to configure SMTP server and credentials: 

```bash
EMAILER_FROM_EMAIL="solanyn@example.com" \
EMAILER_TO_EMAIL="recipient@example.com" \
EMAILER_SMTP_SERVER="smtp.gmail.com" \
EMAILER_SMTP_USERNAME="solanyn@example.com" \
EMAILER_SMTP_PASSWORD="super_secret-p@ssw0rd" \
EMAILER_EMAIL_ATTACHMENT_URL="https://colortest.page/wp-content/uploads/2023/05/printer-color-test-page-pdf.jpg" \
RUST_LOG="info" \
cargo run
```

To build the container:

```bash
docker build -t emailer .
```

To run and configure the container:

```bash
docker run \
    -e EMAILER_FROM_EMAIL="solanyn@example.com" \
    -e EMAILER_TO_EMAIL="recipient@example.com" \
    -e EMAILER_SMTP_SERVER="smtp.gmail.com" \
    -e EMAILER_SMTP_USERNAME="solanyn@example.com" \
    -e EMAILER_SMTP_PASSWORD="super_secret-p@ssw0rd" \
    -e EMAILER_EMAIL_ATTACHMENT_URL="https://colortest.page/wp-content/uploads/2023/05/printer-color-test-page-pdf.jpg" \
    -e RUST_LOG="info" \
    emailer
```
