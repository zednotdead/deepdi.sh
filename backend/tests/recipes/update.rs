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
async fn updates_recipe_correctly() {
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

    let changeset = serde_json::json!({
        "name": "WE UPDATED THIS THING",
        "description": "WE UPDATED THAT THING",
        "steps": ["WE UPDATED ANOTHER THING"],
        "time": {
            "Prep time": 9000,
            "Cook time": 3000,
        },
        "servings": {
            "from_to": [3, 4]
        }
    });

    let recipe_update_path = app.get_base(&format!("recipe/{}", &result.id));

    let result = client
        .put(&recipe_update_path)
        .json(&changeset)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::OK);

    let result: RecipeDTO = result.json().await.unwrap();

    let expected = serde_json::json!({
        "name": "WE UPDATED THIS THING",
        "description": "WE UPDATED THAT THING",
        "time": {
            "Prep time": 9000,
            "Cook time": 3000
        },
        "ingredients": ingredients
            .iter()
            .map(|ing| serde_json::json!({
                "ingredient": ing,
                "optional": false
            }))
            .collect::<Vec<_>>(),
        "steps": ["WE UPDATED ANOTHER THING"],
        "servings": {
            "from_to": [3, 4]
        },
    });

    assert_json_include!(actual: result, expected: expected);
}

#[tokio::test]
async fn updating_nonexistent_recipe_errors() {
    let app = TestApp::new().await;
    let client = Client::new();
    let changeset = serde_json::json!({
        "name": "WE NEED THIS TO FAIL",
    });

    let recipe_update_path = app.get_base(&format!("recipe/{}", &Uuid::nil()));

    let result = client
        .put(&recipe_update_path)
        .json(&changeset)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn updating_empty_changeset_errors() {
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

    let changeset = serde_json::json!({});

    let recipe_update_path = app.get_base(&format!("recipe/{}", &result.id));

    let result = client
        .put(&recipe_update_path)
        .json(&changeset)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::BAD_REQUEST);
}
