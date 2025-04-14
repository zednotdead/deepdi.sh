use assert_json_diff::assert_json_include;
use common::{ingredients::IngredientDTO, RecipeDTO};
use futures::future::join_all;
use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};

use crate::{
    fixtures::{
        ingredient::{ingredient_fixture, ingredient_fixture_meat},
        recipe::recipe_fixture,
    },
    setup::TestApp,
};

#[tokio::test]
async fn inserts_recipe_correctly() {
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
        "diet_violations": []
    });

    assert_eq!(result.ingredients.len(), 1);
    assert_json_include!(actual: result, expected: expected);
}

#[tokio::test]
async fn inserting_recipe_with_multiple_ingredients_generates_diet_violations_correctly() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");
    let recipe_create_path = app.get_base("recipe");

    let ingredients_input = [ingredient_fixture(), ingredient_fixture_meat()];

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
        "diet_violations": ["vegan", "vegetarian"]
    });

    assert_eq!(result.ingredients.len(), 2);
    assert_json_include!(actual: result, expected: expected);
}

#[tokio::test]
async fn inserting_recipe_with_incorrect_ingredients_fails() {
    let app = TestApp::new().await;
    let client = Client::new();
    let recipe_create_path = app.get_base("recipe");

    let data = serde_json::json!({
        "name": "A diced cucumber",
        "description": "Cucumber that's been diced",
        "ingredients": [
            {
                "ingredient_id": uuid::Uuid::nil(),
                "optional": false,
                "amount": {
                    "_type": "grams",
                    "amount": 100.0
                },
            }
        ],
        "time": {
            "Prep time": 6000
        },
        "steps": ["Get a cucumber", "Dice it"],
        "servings": {
            "exact": 1
        },
    });

    let result = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::BAD_REQUEST);

    let result = result
        .json::<common::error::ErrorMessageWithJsonValue>()
        .await
        .unwrap();

    assert_eq!(result.kind, "IngredientsNotFound");
}

#[tokio::test]
async fn inserting_recipe_with_partially_incorrect_ingredients() {
    let app = TestApp::new().await;
    let client = Client::new();
    let recipe_create_path = app.get_base("recipe");
    let ingredient_create_path = app.get_base("ingredient");

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

    let data = serde_json::json!({
        "name": "A diced cucumber",
        "description": "Cucumber that's been diced",
        "ingredients": [
            {
                "ingredient_id": uuid::Uuid::nil(),
                "optional": false,
                "amount": {
                    "_type": "grams",
                    "amount": 100.0
                },
            },
            {
                "ingredient_id": ingredients[0].id,
                "optional": false,
                "amount": {
                    "_type": "cups",
                    "amount": 2.0
                },
            }
        ],
        "time": {
            "Prep time": 6000
        },
        "steps": ["Get a cucumber", "Dice it"],
        "servings": {
            "exact": 1
        },
    });

    let result = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::BAD_REQUEST);

    let result = result
        .json::<common::error::ErrorMessageWithJsonValue>()
        .await
        .unwrap();

    assert_eq!(result.kind, "IngredientsNotFound");
}
