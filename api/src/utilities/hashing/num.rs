use rand::Rng;
use totp_rs::{TOTP, Algorithm, Secret};

// Random number with a specified length
#[allow(dead_code)]
pub fn random_num(length: u8) -> u64 {
  let mut rng = rand::rng();

  if length == 0 { 
      return 0;
  }
  
  // Calculate lower and upper bounds for x digits
  let lower_bound = 10u64.pow((length - 1) as u32);
  let upper_bound = 10u64.pow(length as u32) - 1;
  
  // Generate a random number in the range
  rng.random_range(lower_bound..=upper_bound)
}

// Generate a time-based OTP token
#[allow(dead_code)]
pub fn time_num() -> Result<(String, String), String> {
  // Create a random secret
  let mut rng = rand::rng();
  let mut secret_bytes = [0u8; 32];
  rng.fill(&mut secret_bytes);
  
  // Encode secret to base32
  let secret = Secret::Raw(secret_bytes.to_vec()).to_encoded().to_string();
  
  // Create a TOTP with standard parameters
  let totp = TOTP::new(
    Algorithm::SHA1,
    6,
    1,
    30,
    Secret::Encoded(secret.clone()).to_bytes().unwrap_or_default(),
  ).map_err(|e| format!("Failed to create TOTP: {}", e))?;
  
  // Generate the current token
  match totp.generate_current() {
    Ok(token) => Ok((token, secret)),
    Err(e) => Err(format!("Failed to generate TOTP: {}", e)),
  }
}

// Verify user-provided code against a secret
#[allow(dead_code)]
pub fn verify_num(code: &str, encoded_secret: &str) -> bool {
  // Create TOTP from the secret
  let totp = match TOTP::new(
    Algorithm::SHA1,
    6,
    1,
    30,
    Secret::Encoded(encoded_secret.to_string()).to_bytes().unwrap_or_default(),
  ) {
    Ok(totp) => totp,
    Err(_) => return false,
  };
  
  // Match generated token with user-provided code
  match totp.check_current(code) {
    Ok(valid) => valid,
    Err(_) => false,
  }
}
