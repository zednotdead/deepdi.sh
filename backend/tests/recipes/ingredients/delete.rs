use common::{error::ErrorMessage, IngredientDTO, RecipeDTO};
use futures::future::join_all;
use pretty_assertions::assert_eq;
use reqwest::{Client, StatusCode};
use uuid::Uuid;

use crate::{
    fixtures::{
        ingredient::{ingredient_fixture, ingredient_fixture_evil, ingredient_fixture_meat},
        recipe::recipe_fixture,
    },
    setup::TestApp,
};

#[tokio::test]
pub async fn deleting_an_existing_ingredient_works() {
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

    let data = recipe_fixture(&[veg.clone(), meat.clone()]);

    let result: RecipeDTO = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(result.ingredients.len(), 2);

    let ingredient_delete_path =
        app.get_base(&format!("recipe/{}/ingredient/{}", result.id, meat.id));
    let get_recipe_path = app.get_base(&format!("recipe/{}", result.id));

    let result = client.delete(&ingredient_delete_path).send().await.unwrap();

    assert_eq!(result.status(), StatusCode::OK);

    let recipe: RecipeDTO = client
        .get(&get_recipe_path)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(recipe.ingredients.len(), 1);
}

#[tokio::test]
pub async fn deleting_an_ingredient_that_doesnt_appear_in_recipe_errors() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");
    let recipe_create_path = app.get_base("recipe");

    let ingredients_to_create = [
        ingredient_fixture(),
        ingredient_fixture_meat(),
        ingredient_fixture_evil(),
    ];

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

    let [ref veg, ref meat, ref evil, ..] = ingredients[..] else {
        panic!("Something went wrong with the ingredient adding step")
    };

    let data = recipe_fixture(&[veg.clone(), meat.clone()]);

    let result: RecipeDTO = client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(result.ingredients.len(), 2);

    let ingredient_delete_path =
        app.get_base(&format!("recipe/{}/ingredient/{}", result.id, evil.id));

    let result = client.delete(&ingredient_delete_path).send().await.unwrap();

    assert_eq!(result.status(), StatusCode::BAD_REQUEST);

    let body: ErrorMessage<String> = result.json().await.unwrap();

    assert_eq!(body.kind, "RecipeHasNoIngredientError");
}

#[tokio::test]
pub async fn deleting_an_ingredient_in_recipe_that_doesnt_exist_errors() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");

    let ingredients_to_create = [
        ingredient_fixture(),
        ingredient_fixture_meat(),
        ingredient_fixture_evil(),
    ];

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

    let ingredient_delete_path =
        app.get_base(&format!("recipe/{}/ingredient/{}", Uuid::nil(), veg.id));

    let result = client.delete(&ingredient_delete_path).send().await.unwrap();

    assert_eq!(result.status(), StatusCode::NOT_FOUND);

    let body: ErrorMessage<String> = result.json().await.unwrap();

    assert_eq!(body.kind, "RecipeNotFoundError");
}

#[tokio::test]
pub async fn deleting_the_last_ingredient_in_recipe_errors() {
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

    let ingredient_delete_path =
        app.get_base(&format!("recipe/{}/ingredient/{}", result.id, veg.id));

    let result = client.delete(&ingredient_delete_path).send().await.unwrap();

    assert_eq!(result.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body: ErrorMessage<String> = result.json().await.unwrap();

    assert_eq!(body.kind, "LastIngredientError");
}
