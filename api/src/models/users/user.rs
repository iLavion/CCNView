use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use crate::services::auth::signup::AuthProvider;

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedProvider {
  pub provider: AuthProvider, // Auth Provider (e.g., Google, GitHub)
  pub p_uid: String,          // Provider User ID
  pub p_email: String,        // Provider Email   
  pub cat: DateTime,          // Creation time 
  pub uat: DateTime,          // Last update time 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,       // User ID
  pub email: String,              // User email
  pub email_v: Option<bool>,      // Email verified
  pub uname: String,              // Username (will auto set if not provided)
  pub pwd: Option<String>,        // Optional for non-local users
  pub links: Vec<LinkedProvider>, // Linked providers 
  pub phone: Option<String>,      // Optional phone number
  pub phone_v: Option<bool>,      // Phone verified
  pub mfa: Option<bool>,          // Multi-factor authentication
  pub mfa_v: Option<bool>,        // MFA verified
  pub mfa_key: Option<String>,    // MFA key
  pub news: bool,                 // Newsletter subscription  
  pub tos: bool,                  // Terms of service acceptance
  pub cat: DateTime,              // Creation time
  pub uat: DateTime,              // Last update time
}

impl User {
    pub fn new(
        email: String,
        username: Option<String>,
        password_hash: Option<String>,
        provider: AuthProvider,
        newsletter: bool,
        tos: bool,
    ) -> Self {
        let now = DateTime::now();
        let actual_username = username.unwrap_or_else(|| {
            // Generate username from email if not provided
            email.split('@').next().unwrap_or("user").to_string()
        });
        
        // Create linked providers list if using OAuth
        let links = if provider != AuthProvider::Local {
            vec![LinkedProvider {
                provider: provider.clone(),
                p_uid: String::new(), // This would need to be set with actual provider user ID during OAuth flow
                p_email: email.clone(),
                cat: now,
                uat: now,
            }]
        } else {
            Vec::new()
        };
        
        User {
            id: None,  // MongoDB will assign this
            email,
            email_v: Some(false),  // Default to unverified
            uname: actual_username,
            pwd: password_hash,
            links,
            phone: None,
            phone_v: None,
            mfa: Some(false),
            mfa_v: None,
            mfa_key: None,
            news: newsletter,
            tos,
            cat: now,
            uat: now,
        }
    }
}