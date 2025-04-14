use assert_json_diff::assert_json_include;
use common::{error::ErrorMessage, IngredientDTO, IngredientUnitDTO, RecipeDTO};
use futures::future::join_all;
use reqwest::{Client, StatusCode};
use uuid::Uuid;

use crate::{
    fixtures::{
        ingredient::{ingredient_fixture, ingredient_fixture_meat},
        recipe::recipe_fixture,
    },
    setup::TestApp,
};

#[tokio::test]
async fn updating_an_ingredient_in_a_recipe_works() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");
    let recipe_create_path = app.get_base("recipe");

    let ingredient: IngredientDTO = client
        .post(&ingredient_create_path)
        .json(&ingredient_fixture())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let data = recipe_fixture(&[ingredient.clone()]);

    let result: RecipeDTO = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(result.ingredients.len(), 1);

    let ingredient_update_path = app.get_base(&format!(
        "recipe/{}/ingredient/{}",
        result.id, ingredient.id
    ));
    let get_recipe_path = app.get_base(&format!("recipe/{}", result.id));

    let ingredient_to_add = IngredientUnitDTO::Grams(999.0);

    let result = client
        .put(&ingredient_update_path)
        .json(&ingredient_to_add)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::OK);

    let result: RecipeDTO = client
        .get(&get_recipe_path)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_json_include!(actual: result, expected: serde_json::json!({
        "ingredients": vec![
            serde_json::json!({
                "amount": {
                    "_type": "grams",
                    "amount": 999.0
                }
            })
        ]
    }))
}

#[tokio::test]
async fn updating_a_nonexistent_ingredient_in_a_recipe_errors() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");
    let recipe_create_path = app.get_base("recipe");

    let ingredients_to_create = [ingredient_fixture(), ingredient_fixture_meat()];

    let ingredients: Vec<IngredientDTO> =
        join_all(ingredients_to_create.iter().map(|ingredient| async {
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

    let [ref veg, ref meat, ..] = ingredients[..] else {
        panic!("Something went wrong with the ingredient adding step")
    };

    let data = recipe_fixture(&[veg.clone()]);

    let result: RecipeDTO = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(result.ingredients.len(), 1);

    let ingredient_update_path =
        app.get_base(&format!("recipe/{}/ingredient/{}", result.id, meat.id));

    let ingredient_to_add = IngredientUnitDTO::Grams(999.0);

    let result = client
        .put(&ingredient_update_path)
        .json(&ingredient_to_add)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::NOT_FOUND);

    let result: ErrorMessage<String> = result.json().await.unwrap();

    assert_eq!(result.kind, "MissingIngredient")
}

#[tokio::test]
async fn updating_an_ingredient_in_a_nonexistent_recipe_errors() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");

    let ingredient: IngredientDTO = client
        .post(&ingredient_create_path)
        .json(&ingredient_fixture())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let ingredient_update_path = app.get_base(&format!(
        "recipe/{}/ingredient/{}",
        Uuid::nil(),
        ingredient.id
    ));

    let ingredient_to_add = IngredientUnitDTO::Grams(999.0);

    let result = client
        .put(&ingredient_update_path)
        .json(&ingredient_to_add)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::NOT_FOUND);

    let result: ErrorMessage<String> = result.json().await.unwrap();

    assert_eq!(result.kind, "GetRecipe")
}
