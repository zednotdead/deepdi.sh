use axum::{extract::State, response::IntoResponse};
use common::{CreateIngredientDTO, IngredientDTO};
use reqwest::StatusCode;

use crate::{
    api::{errors::MakeError, extract::Json, AppState},
    domain::commands::ingredients::create::{
        create_ingredient, CreateIngredient, CreateIngredientError,
    },
};

impl MakeError<String> for CreateIngredientError {
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

impl IntoResponse for CreateIngredientError {
    fn into_response(self) -> axum::response::Response {
        (self.get_status_code(), self.get_message()).into_response()
    }
}

#[tracing::instrument("[ROUTE] Creating a new ingredient", skip(ingredient_repository, message_service))]
pub async fn create_ingredient_route(
    State(AppState {
        ingredient_repository,
        message_service,
        ..
    }): State<AppState>,
    Json(body): Json<CreateIngredientDTO>,
) -> Result<impl IntoResponse, CreateIngredientError> {
    let input = CreateIngredient {
        name: &body.name,
        description: &body.description,
        diet_violations: body.diet_violations.unwrap_or_default(),
    };
    let result = create_ingredient(ingredient_repository, message_service, &input).await?;
    let result: IngredientDTO = result.into();

    Ok((StatusCode::CREATED, Json(result)))
}
