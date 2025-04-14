use common::{
    error::ErrorMessage, IngredientAmountDTO, IngredientDTO, IngredientUnitDTO, RecipeDTO,
};
use futures::future::join_all;
use pretty_assertions::assert_eq;
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
async fn adding_an_ingredient_to_a_recipe_works() {
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

    let ingredient_add_path = app.get_base(&format!("recipe/{}/ingredient", result.id));
    let get_recipe_path = app.get_base(&format!("recipe/{}", result.id));

    let ingredient_to_add = IngredientAmountDTO {
        ingredient_id: meat.id,
        optional: true,
        amount: IngredientUnitDTO::Grams(10.0),
        notes: None,
    };

    let result = client
        .post(&ingredient_add_path)
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

    assert_eq!(result.ingredients.len(), 2);
}

#[tokio::test]
async fn adding_a_nonexistent_ingredient_to_a_recipe_fails() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");
    let recipe_create_path = app.get_base("recipe");

    let ingredients_to_create = [ingredient_fixture()];

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

    let [ref veg, ..] = ingredients[..] else {
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

    let ingredient_add_path = app.get_base(&format!("recipe/{}/ingredient", result.id));

    let ingredient_to_add = IngredientAmountDTO {
        ingredient_id: Uuid::nil(),
        optional: true,
        amount: IngredientUnitDTO::Grams(10.0),
        notes: None,
    };

    let result = client
        .post(&ingredient_add_path)
        .json(&ingredient_to_add)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn adding_an_ingredient_to_a_nonexistent_recipe_fails() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");

    let ingredients_to_create = [ingredient_fixture()];

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

    let [ref veg, ..] = ingredients[..] else {
        panic!("Something went wrong with the ingredient adding step")
    };

    let ingredient_add_path = app.get_base(&format!("recipe/{}/ingredient", Uuid::nil()));

    let ingredient_to_add = IngredientAmountDTO {
        ingredient_id: veg.id,
        optional: true,
        amount: IngredientUnitDTO::Grams(10.0),
        notes: None,
    };

    let result = client
        .post(&ingredient_add_path)
        .json(&ingredient_to_add)
        .send()
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::NOT_FOUND);

    let result: ErrorMessage<String> = result.json().await.unwrap();

    assert_eq!(result.kind, "GetRecipe")
}
