#[derive(serde::Deserialize, Debug)]
pub struct EmailerConfig {
    pub from_email: String,
    pub to_email: String,
    pub email_attachment_url: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
}
