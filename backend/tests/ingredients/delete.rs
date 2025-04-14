use common::IngredientDTO;
use reqwest::{Client, StatusCode};
use uuid::Uuid;

use crate::{
    fixtures::{ingredient::ingredient_fixture, recipe::recipe_fixture},
    setup::TestApp,
};

#[tokio::test]
async fn deleting_works() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");
    let ingredient = ingredient_fixture();

    let ingredient: IngredientDTO = client
        .post(&ingredient_create_path)
        .json(&ingredient.clone())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let ingredient_delete_path = app.get_base(&format!("ingredient/{}", ingredient.id));

    let response = client.delete(ingredient_delete_path).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn deleting_nonexistent_ingredient_errors() {
    let app = TestApp::new().await;
    let client = Client::new();

    let ingredient_delete_path = app.get_base(&format!("ingredient/{}", Uuid::nil()));

    let response = client.delete(ingredient_delete_path).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn deleting_ingredient_in_use_by_recipe_errors() {
    let app = TestApp::new().await;
    let client = Client::new();
    let ingredient_create_path = app.get_base("ingredient");
    let recipe_create_path = app.get_base("recipe");

    let ingredient = ingredient_fixture();

    let ingredient: IngredientDTO = client
        .post(&ingredient_create_path)
        .json(&ingredient.clone())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let data = recipe_fixture(&[ingredient.clone()]);

    client
        .post(&recipe_create_path)
        .json(&data)
        .send()
        .await
        .unwrap();

    let ingredient_delete_path = app.get_base(&format!("ingredient/{}", ingredient.id));

    let response = client.delete(ingredient_delete_path).send().await.unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}
