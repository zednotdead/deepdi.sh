use crate::domain::{entities::recipe::Recipe, repositories::recipe::RecipeRepositoryService};

#[derive(thiserror::Error, Debug, strum::AsRefStr)]
pub enum GetAllRecipesError {
    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}

#[tracing::instrument("[QUERY] Get all recipes", skip(recipe_repo))]
pub async fn get_all_recipes(
    recipe_repo: RecipeRepositoryService,
) -> Result<Vec<Recipe>, GetAllRecipesError> {
    let result = recipe_repo
        .get_all()
        .await
        .map_err(|e| GetAllRecipesError::Unknown(e.into()))?;

    Ok(result)
}
