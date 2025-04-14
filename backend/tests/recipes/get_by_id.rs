use assert_json_diff::assert_json_include;
use common::{ingredients::IngredientDTO, RecipeDTO};
use futures::future::join_all;
use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use uuid::Uuid;

use crate::{
    fixtures::{ingredient::ingredient_fixture, recipe::recipe_fixture},
    setup::TestApp,
};

#[tokio::test]
async fn getting_recipe_by_id_works_correctly() {
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

    let result: RecipeDTO = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let get_recipe_by_id_route = app.get_base(&format!("recipe/{}", result.id));
    let result: RecipeDTO = client
        .get(get_recipe_by_id_route)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let expected = serde_json::json!({
        "name": "A diced cucumber",
        "description": "Cucumber that's been diced",
        "time": {
            "Prep time": 6000
        },
        "ingredients": ingredients
            .iter()
            .map(|ing| serde_json::json!({
                "ingredient": ing,
                "optional": false
            }))
            .collect::<Vec<_>>(),
        "steps": ["Get a cucumber", "Dice it"],
        "servings": {
            "exact": 1
        },
    });

    assert_eq!(result.ingredients.len(), 1);
    assert_json_include!(actual: result, expected: expected);
}

#[tokio::test]
async fn getting_nonexistent_recipe_fails() {
    let app = TestApp::new().await;
    let client = Client::new();

    let get_recipe_by_id_route = app.get_base(&format!("recipe/{}", Uuid::nil()));

    let result = client.get(get_recipe_by_id_route).send().await.unwrap();

    assert_eq!(result.status(), StatusCode::NOT_FOUND);

    let body = result
        .json::<common::error::ErrorMessageWithJsonValue>()
        .await
        .unwrap();

    assert_eq!(body.kind, "NotFound");
}
