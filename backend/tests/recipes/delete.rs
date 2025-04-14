use common::{IngredientDTO, RecipeDTO};
use futures::future::join_all;
use reqwest::{Client, StatusCode};

use crate::{
    fixtures::{ingredient::ingredient_fixture, recipe::recipe_fixture},
    setup::TestApp,
};

#[tokio::test]
async fn deleting_a_recipe_works() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");
    let recipe_create_path = app.get_base("recipe");

    let ingredients_input = [ingredient_fixture()];

    let ingredients: Vec<IngredientDTO> =
        join_all(ingredients_input.iter().map(|ingredient| async {
            client
                .post(&ingredient_create_path)
                .json(&ingredient.clone())
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap()
        }))
        .await;

    let data = recipe_fixture(&ingredients);

    let inserted_recipe: RecipeDTO = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let recipe_delete_path = app.get_base(&format!("recipe/{}", inserted_recipe.id));
    let result = client.delete(recipe_delete_path).send().await.unwrap();

    assert_eq!(result.status(), StatusCode::OK)
}

#[tokio::test]
async fn deleting_a_nonexistent_recipe_errors() {
    let app = TestApp::new().await;
    let client = Client::new();

    let recipe_delete_path = app.get_base(&format!("recipe/{}", uuid::Uuid::nil()));
    let result = client.delete(recipe_delete_path).send().await.unwrap();

    assert_eq!(result.status(), StatusCode::NOT_FOUND)
}
