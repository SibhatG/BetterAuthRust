use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::config::Config;
use crate::errors::AuthError;

pub struct EmailService {
    config: Config,
}

impl EmailService {
    pub fn new(config: Config) -> Self {
        EmailService { config }
    }

    pub async fn send_verification_email(
        &self,
        email: &str,
        token: &str,
    ) -> Result<(), AuthError> {
        let subject = "Verify your email address";
        let verification_url = format!("https://example.com/verify-email?token={}", token);
        
        let html_body = format!(
            r#"
            <html>
                <body>
                    <h1>Verify your email address</h1>
                    <p>Thank you for registering! Please click the link below to verify your email address:</p>
                    <p><a href="{}">Verify Email</a></p>
                    <p>Or copy and paste this link: {}</p>
                    <p>This link will expire in 24 hours.</p>
                </body>
            </html>
            "#,
            verification_url, verification_url
        );

        let text_body = format!(
            r#"
            Verify your email address
            
            Thank you for registering! Please visit the link below to verify your email address:
            
            {}
            
            This link will expire in 24 hours.
            "#,
            verification_url
        );

        self.send_email(email, subject, &html_body, &text_body).await
    }

    pub async fn send_password_reset_email(
        &self,
        email: &str,
        token: &str,
    ) -> Result<(), AuthError> {
        let subject = "Reset your password";
        let reset_url = format!("https://example.com/reset-password?token={}", token);
        
        let html_body = format!(
            r#"
            <html>
                <body>
                    <h1>Reset your password</h1>
                    <p>You requested a password reset. Please click the link below to reset your password:</p>
                    <p><a href="{}">Reset Password</a></p>
                    <p>Or copy and paste this link: {}</p>
                    <p>This link will expire in 24 hours.</p>
                    <p>If you didn't request a password reset, please ignore this email.</p>
                </body>
            </html>
            "#,
            reset_url, reset_url
        );

        let text_body = format!(
            r#"
            Reset your password
            
            You requested a password reset. Please visit the link below to reset your password:
            
            {}
            
            This link will expire in 24 hours.
            
            If you didn't request a password reset, please ignore this email.
            "#,
            reset_url
        );

        self.send_email(email, subject, &html_body, &text_body).await
    }

    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<(), AuthError> {
        let email = Message::builder()
            .from(self.config.email.from_email.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(text_body.to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html_body.to_string()),
                    ),
            )?;

        let creds = Credentials::new(
            self.config.email.smtp_username.clone(),
            self.config.email.smtp_password.clone(),
        );

        let mailer = SmtpTransport::relay(&self.config.email.smtp_host)
            .unwrap()
            .credentials(creds)
            .port(self.config.email.smtp_port)
            .build();

        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => Err(AuthError::EmailError(e.to_string())),
        }
    }
}
