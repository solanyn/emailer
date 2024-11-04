# emailer

This project is a Rust program that sends an email using IMAP. Epson inkjet printers can be prone to clogging if unused, so this project sends email to send to Epson Email Print online service. The service connects back to my printer and prints attachments and email body message. 

# Usage

To run the rust binary, use the following command:

```bash
cargo run
```

To build the container:

```bash
docker build -t emailer .
```

