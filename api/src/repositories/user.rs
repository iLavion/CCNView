// src/repositories/user.rs
//! CRUD OPERATIONS FOR USER

use log::{info, debug, warn, error};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection, Client, error::Error as MongoError,
};
use crate::models::users::user::User;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] MongoError),
    #[error("User not found")]
    NotFound,
    #[error("User already exists with this email")]
    DuplicateEmail,
    #[error("User already exists with this username")]
    DuplicateUsername,
}

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(client: &Client) -> Self {
        debug!("Creating user repository");
        let db = client.database("invalsia");
        let collection = db.collection("users");
        UserRepository { collection }
    }

    #[allow(dead_code)]
    pub async fn create_user(&self, user: User) -> Result<ObjectId, RepositoryError> {
        debug!("Attempting to create user: {:?}", user);
        
        // Check if user with same email already exists
        debug!("Checking for duplicate email: {}", user.email);
        if self.find_by_email(&user.email).await.is_ok() {
            warn!("Duplicate email detected: {}", user.email);
            return Err(RepositoryError::DuplicateEmail);
        }

        // Check username uniqueness if provided
        let username = &user.uname;
        if !username.is_empty() {
            debug!("Checking for duplicate username: {}", username);
            if self.find_by_username(username).await.is_ok() {
                warn!("Duplicate username detected: {}", username);
                return Err(RepositoryError::DuplicateUsername);
            }
        }

        // Insert the user
        info!("Inserting new user with email: {}", user.email);
        let result = match self.collection.insert_one(user).await {
            Ok(result) => result,
            Err(e) => {
                error!("Database error during user creation: {}", e);
                return Err(RepositoryError::DatabaseError(e));
            }
        };
        
        let id = result.inserted_id.as_object_id()
            .expect("Failed to get inserted ID");
        
        info!("User created successfully with ID: {}", id);
        Ok(id)
    }

    #[allow(dead_code)]
    pub async fn find_by_id(&self, id: &ObjectId) -> Result<User, RepositoryError> {
        debug!("Finding user by ID: {}", id);
        let filter = doc! { "_id": id };
        let result = self.collection.find_one(filter).await;
        match result {
            Ok(Some(user)) => {
                info!("User found with ID: {}", id);
                Ok(user)
            },
            Ok(None) => {
                warn!("User not found with ID: {}", id);
                Err(RepositoryError::NotFound)
            },
            Err(e) => {
                error!("Database error during find_by_id: {}", e);
                Err(RepositoryError::DatabaseError(e))
            }
        }
    }

    #[allow(dead_code)]
    pub async fn find_by_email(&self, email: &str) -> Result<User, RepositoryError> {
        debug!("Finding user by email: {}", email);
        let filter = doc! { "email": email };
        let result = self.collection.find_one(filter).await;
        match result {
            Ok(Some(user)) => {
                info!("User found with email: {}", email);
                Ok(user)
            },
            Ok(None) => {
                warn!("User not found with email: {}", email);
                Err(RepositoryError::NotFound)
            },
            Err(e) => {
                error!("Database error during find_by_email: {}", e);
                Err(RepositoryError::DatabaseError(e))
            }
        }
    }

    #[allow(dead_code)]
    pub async fn find_by_username(&self, username: &str) -> Result<User, RepositoryError> {
        debug!("Finding user by username: {}", username);
        let filter = doc! { "username": username };
        let result = self.collection.find_one(filter).await;
        match result {
            Ok(Some(user)) => {
                info!("User found with username: {}", username);
                Ok(user)
            },
            Ok(None) => {
                warn!("User not found with username: {}", username);
                Err(RepositoryError::NotFound)
            },
            Err(e) => {
                error!("Database error during find_by_username: {}", e);
                Err(RepositoryError::DatabaseError(e))
            }
        }
    }

    #[allow(dead_code)]
    pub async fn update_user(&self, id: &ObjectId, update: Document) -> Result<(), RepositoryError> {
        debug!("Updating user with ID: {}", id);
        let filter = doc! { "_id": id };
        let update_doc = doc! { "$set": update };
        
        let result = self.collection.update_one(filter, update_doc).await;
        match result {
            Ok(update_result) => {
                if update_result.matched_count == 0 {
                    warn!("No user found to update with ID: {}", id);
                    return Err(RepositoryError::NotFound);
                }
                info!("User updated successfully with ID: {}", id);
                Ok(())
            },
            Err(e) => {
                error!("Database error during update_user: {}", e);
                Err(RepositoryError::DatabaseError(e))
            }
        }
    }

    #[allow(dead_code)]
    pub async fn delete_user(&self, id: &ObjectId) -> Result<(), RepositoryError> {
        debug!("Deleting user with ID: {}", id);
        let filter = doc! { "_id": id };
        let result = self.collection.delete_one(filter).await;
        match result {
            Ok(delete_result) => {
                if delete_result.deleted_count == 0 {
                    warn!("No user found to delete with ID: {}", id);
                    return Err(RepositoryError::NotFound);
                }
                info!("User deleted successfully with ID: {}", id);
                Ok(())
            },
            Err(e) => {
                error!("Database error during delete_user: {}", e);
                Err(RepositoryError::DatabaseError(e))
            }
        }
    }
}