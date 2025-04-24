use axum::{extract::State, response::IntoResponse};
use common::IngredientDTO;

use crate::{
    api::{errors::MakeError, extract::Json, AppState},
    domain::queries::ingredients::get_all::{get_all_ingredients, GetAllIngredientsError},
};

impl MakeError<String> for GetAllIngredientsError {
    fn get_kind(&self) -> String {
        self.as_ref().to_string()
    }
    fn get_message(&self) -> String {
        self.to_string()
    }
    fn get_status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::BAD_REQUEST
    }
}

impl IntoResponse for GetAllIngredientsError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_message()).into_response()
    }
}

#[tracing::instrument(
    "[ROUTE] Getting all available ingredients",
    skip(ingredient_repository)
)]
pub async fn get_all_ingredients_route(
    State(AppState {
        ingredient_repository,
        ..
    }): State<AppState>,
) -> Result<Json<Vec<IngredientDTO>>, GetAllIngredientsError> {
    tracing::info!("Getting all ingredients");
    let result = get_all_ingredients(ingredient_repository).await?;

    Ok(Json(result.iter().map(IngredientDTO::from).collect()))
}
