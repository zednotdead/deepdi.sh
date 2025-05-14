use axum::{extract::State, Json};
use common::RecipeDTO;

use crate::domain::queries::recipes::get_by_id::GetRecipeError;
use crate::{api::AppState, domain::queries::recipes::get_all::get_all_recipes};

#[tracing::instrument("[ROUTE] Getting all recipes", skip(recipe_repository))]
pub async fn get_all_recipes_route(
    State(AppState {
        recipe_repository, ..
    }): State<AppState>,
) -> Result<Json<Vec<RecipeDTO>>, GetRecipeError> {
    let result: Vec<RecipeDTO> = get_all_recipes(recipe_repository)
        .await
        .map_err(|e| GetRecipeError::Unknown(e.into()))?
        .into_iter()
        .map(RecipeDTO::from)
        .collect();

    Ok(axum::Json(result))
}
