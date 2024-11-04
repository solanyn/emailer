use config::Config;
use emailer::EmailerConfig;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set default logging level to info
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    env_logger::init();

    // Parse config from environment
    let config = Config::builder()
        .add_source(config::Environment::with_prefix("EMAILER").try_parsing(true))
        .build()
        .unwrap();
    let c: EmailerConfig = config.try_deserialize().unwrap();
    log::info!("Emailer Configuration Values:\n{:?}", c);

    // Download attachment and store in memory
    let attachment_resp = reqwest::blocking::get(c.email_attachment_url).unwrap();

    // Set timezone for subject line
    let tz: chrono_tz::Tz = std::env::var("TZ")
        .unwrap_or(String::from("UTC"))
        .parse()
        .unwrap();
    let now = chrono::Utc::now().with_timezone(&tz);

    // Build email message with attachment
    let email = Message::builder()
        .from(Mailbox::new(None, c.from_email.parse().unwrap()))
        .to(Mailbox::new(None, c.to_email.parse().unwrap()))
        .subject(format!("{} - Weekly Email Print", now.format("%Y-%m-%d")))
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(String::from("")),
                )
                .singlepart(Attachment::new_inline(String::from("1")).body(
                    attachment_resp.bytes().unwrap().to_vec(),
                    ContentType::parse("image/jpeg").unwrap(),
                )),
        )
        .unwrap();

    // Authenticate to SMTP provider
    let creds = Credentials::new(c.smtp_username, c.smtp_password);

    // Open a remote connection to SMTP server
    let mailer = SmtpTransport::starttls_relay(&c.smtp_server)?
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => log::info!("Email sent successfully!"),
        Err(e) => log::error!("Could not send email: {e:?}"),
    }

    Ok(())
}
