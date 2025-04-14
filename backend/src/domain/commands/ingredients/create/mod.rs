use uuid::Uuid;

use crate::domain::entities::ingredient::*;
use crate::domain::repositories::ingredients::{
    errors::InsertIngredientError, IngredientRepositoryService,
};
use crate::domain::services::message::MessageServiceImpl;

use self::errors::ValidationError;
use self::types::DietViolations;

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum CreateIngredientError {
    #[error("The field {0} was empty")]
    EmptyField(&'static str),
    #[error(
        "A conflict has occured - an ingredient with field {0} of the given value already exists."
    )]
    Conflict(String),
    #[error(transparent)]
    Internal(#[from] eyre::Error),
}

impl From<InsertIngredientError> for CreateIngredientError {
    fn from(value: InsertIngredientError) -> Self {
        match value {
            InsertIngredientError::Conflict(field) => Self::Conflict(field),
            e => Self::Internal(e.into()),
        }
    }
}

impl From<ValidationError> for CreateIngredientError {
    fn from(value: ValidationError) -> Self {
        match value {
            ValidationError::EmptyField(field) => Self::EmptyField(field[0]),
            e => Self::Internal(e.into()),
        }
    }
}

#[derive(Debug)]
pub struct CreateIngredient<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub diet_violations: Vec<String>,
}

impl<'a> TryFrom<&CreateIngredient<'a>> for Ingredient {
    type Error = ValidationError;
    fn try_from(value: &CreateIngredient<'a>) -> Result<Self, Self::Error> {
        Ok(Ingredient {
            id: Uuid::now_v7(),
            name: value.name.try_into()?,
            description: value.description.try_into()?,
            diet_violations: value
                .diet_violations
                .clone()
                .into_iter()
                .filter_map(|x| DietViolations::try_from(x).ok())
                .collect::<Vec<_>>()
                .into(),
        })
    }
}

#[tracing::instrument("[COMMAND] Creating a new ingredient", skip(repo, message_service))]
pub async fn create_ingredient(
    repo: IngredientRepositoryService,
    message_service: MessageServiceImpl,
    input: &CreateIngredient<'_>,
) -> Result<Ingredient, CreateIngredientError> {
    let ingredient = Ingredient::try_from(input)?;
    let ingredient = repo.insert(ingredient).await?;
    message_service.ingredient_added(&ingredient).await?;
    Ok(ingredient)
}

#[cfg(test)]
mod tests;
