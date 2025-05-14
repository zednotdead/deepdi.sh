use eyre::eyre;
use sqlx::Error as SQLXError;
use std::sync::PoisonError;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::entities::recipe::errors::ValidationError;

#[derive(Error, Debug)]
pub enum GetRecipeByIdError {
    #[error("The recipe with ID of {0} was not found")]
    NotFound(Uuid),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl GetRecipeByIdError {
    pub fn with_id(id: &Uuid, e: SQLXError) -> Self {
        match e {
            SQLXError::RowNotFound => Self::NotFound(*id),
            _ => Self::UnknownError(e.into()),
        }
    }
}

impl<T> From<PoisonError<T>> for GetRecipeByIdError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Recipe repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}

impl From<SQLXError> for GetRecipeByIdError {
    fn from(e: SQLXError) -> Self {
        Self::UnknownError(e.into())
    }
}

impl From<serde_json::Error> for GetRecipeByIdError {
    fn from(e: serde_json::Error) -> Self {
        Self::UnknownError(e.into())
    }
}

#[derive(Error, Debug)]
pub enum GetAllRecipesError {
    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    #[error(transparent)]
    UnknownError(#[from] eyre::Error),
}

impl<T> From<PoisonError<T>> for GetAllRecipesError {
    fn from(_value: PoisonError<T>) -> Self {
        eyre!("Recipe repository lock was poisoned during a previous access and can no longer be locked").into()
    }
}

impl From<SQLXError> for GetAllRecipesError {
    fn from(e: SQLXError) -> Self {
        Self::UnknownError(e.into())
    }
}

impl From<serde_json::Error> for GetAllRecipesError {
    fn from(e: serde_json::Error) -> Self {
        Self::UnknownError(e.into())
    }
}
