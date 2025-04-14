use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    api::{errors::MakeError, AppState},
    domain::commands::ingredients::delete::{delete_ingredient, DeleteIngredientError},
};

impl MakeError<String> for DeleteIngredientError {
    fn get_kind(&self) -> String {
        self.as_ref().to_string()
    }
    fn get_message(&self) -> String {
        self.to_string()
    }
    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InUseByRecipe => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for DeleteIngredientError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_message()).into_response()
    }
}

#[tracing::instrument(
    "[ROUTE] Deleting an ingredient",
    skip(ingredient_repository, recipe_repository, message_service)
)]
pub async fn delete_ingredient_route(
    State(AppState {
        ingredient_repository,
        recipe_repository,
        message_service,
        ..
    }): State<AppState>,
    Path(ingredient_id): Path<Uuid>,
) -> Result<(), DeleteIngredientError> {
    delete_ingredient(
        ingredient_repository,
        recipe_repository,
        message_service,
        &ingredient_id,
    )
    .await?;

    Ok(())
}
