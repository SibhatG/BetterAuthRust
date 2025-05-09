use rand::{distributions::Alphanumeric, Rng};
use totp_rs::{Algorithm, TOTP};

pub struct MfaService {}

impl MfaService {
    pub fn new() -> Self {
        MfaService {}
    }

    pub fn generate_totp_secret(&self, username: &str) -> (String, String) {
        // Generate a random secret
        let secret = self.generate_random_string(32);
        
        // Create TOTP
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret.as_bytes().to_vec(),
            Some("Better Auth".to_string()),
            username.to_string(),
        )
        .unwrap();

        // Generate QR code URL
        let qr_code_url = totp.get_url();

        (base32::encode(base32::Alphabet::RFC4648 { padding: true }, secret.as_bytes()), qr_code_url)
    }

    pub fn verify_totp(&self, secret: &str, code: &str) -> bool {
        // Convert base32 secret to bytes
        let secret_bytes = match base32::decode(base32::Alphabet::RFC4648 { padding: true }, secret) {
            Some(bytes) => bytes,
            None => return false,
        };

        // Create TOTP
        let totp = match TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret_bytes,
            None,
            "".to_string(),
        ) {
            Ok(totp) => totp,
            Err(_) => return false,
        };

        // Check if the code is valid
        totp.check_current(code).unwrap_or(false)
    }

    pub fn generate_recovery_code(&self) -> String {
        // Generate a random alphanumeric string
        let code = self.generate_random_string(16);
        
        // Format as xxxx-xxxx-xxxx-xxxx
        format!(
            "{}-{}-{}-{}",
            &code[0..4],
            &code[4..8],
            &code[8..12],
            &code[12..16]
        )
    }

    fn generate_random_string(&self, length: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
}
