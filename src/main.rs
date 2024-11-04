use config::Config;
use emailer::EmailerConfig;
use lettre::message::header::ContentType;
use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set default logging level to info
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    env_logger::init();

    log::info!("Parsing config from environment...");
    // Parse config from environment
    let config = Config::builder()
        .add_source(config::Environment::with_prefix("EMAILER").try_parsing(true))
        .build()
        .unwrap();
    let c: EmailerConfig = config.try_deserialize().unwrap();
    log::info!("Emailer config: {:#?}", c);

    // Download attachment and store in memory
    log::info!("Downloading attachment...");
    let attachment: Vec<u8> = reqwest::blocking::get(&c.email_attachment_url)
        .unwrap()
        .bytes()
        .unwrap()
        .to_vec();

    let fname = Url::parse(&c.email_attachment_url.to_string())
        .unwrap()
        .path()
        .split("/")
        .into_iter()
        .last()
        .unwrap()
        .to_string();
    log::info!("Downloaded {} from {}", fname, c.email_attachment_url);

    // Set timezone for subject line
    log::info!("Getting TZ from environment variable...");
    let tz: chrono_tz::Tz = std::env::var("TZ")
        .unwrap_or(String::from("UTC"))
        .parse()
        .unwrap();
    let now = chrono::Utc::now().with_timezone(&tz);
    log::info!("Got current datetime {} in timezone {}", now, tz);

    // Build email message with attachment
    log::info!("Building email message...");
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
                .singlepart(
                    Attachment::new(fname)
                        .body(attachment, ContentType::parse("image/jpeg").unwrap()),
                ),
        )
        .unwrap();

    // Authenticate to SMTP provider
    let creds = Credentials::new(c.smtp_username, c.smtp_password);

    // Open a remote connection to SMTP server
    log::info!("Logging into SMTP server {}...", c.smtp_server);
    let mailer = SmtpTransport::starttls_relay(&c.smtp_server)?
        .credentials(creds)
        .build();

    // Send the email
    log::info!("Sending email...");
    match mailer.send(&email) {
        Ok(_) => log::info!("Email sent successfully!"),
        Err(e) => log::error!("Could not send email: {e:?}"),
    }

    log::info!("Exiting...");
    Ok(())
}
