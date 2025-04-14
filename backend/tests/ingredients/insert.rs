use common::IngredientDTO;
use reqwest::{Client, StatusCode};
use serde_json::json;

use crate::setup::TestApp;

#[tokio::test]
async fn inserting_ingredient_succeeds() {
    let app = TestApp::new().await;
    let client = Client::new();
    let path = app.get_base("ingredient");

    let request = client
        .post(&path)
        .json(&json!({
            "name": "Tomato",
            "description": "Tomatoes are very squishy",
            "diet_violations": ["vegan", "vegetarian", "gluten_free"]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(request.status(), StatusCode::CREATED);

    let body = request.json::<IngredientDTO>().await.unwrap();

    let expected_body = IngredientDTO {
        id: uuid::Uuid::nil(),
        name: "Tomato".to_string(),
        description: "Tomatoes are very squishy".to_string(),
        diet_violations: vec![
            "vegan".to_string(),
            "vegetarian".to_string(),
            "gluten_free".to_string(),
        ],
    };

    assert_eq!(body.name, expected_body.name);
    assert_eq!(body.description, expected_body.description);
    assert_eq!(body.diet_violations, expected_body.diet_violations);
}

#[tokio::test]
async fn sending_insufficient_data_errors() {
    let app = TestApp::new().await;
    let client = Client::new();
    let path = app.get_base("ingredient");

    let request = client
        .post(&path)
        .json(&json!({
            "description": "This is an example without the name",
            "diet_violations": ["vegan", "vegetarian", "gluten_free"]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(request.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn incorrect_diets_are_ignored() {
    let app = TestApp::new().await;
    let client = Client::new();
    let path = app.get_base("ingredient");

    let request = client
        .post(&path)
        .json(&json!({
            "name": "Tomato",
            "description": "Tomatoes are very squishy",
            "diet_violations": ["vegan", "vegetarian", "gluten_free", "I_AM_INCORRECT"]
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(request.status(), StatusCode::CREATED);

    let body = request.json::<IngredientDTO>().await.unwrap();

    assert_eq!(
        body.diet_violations,
        vec![
            "vegan".to_string(),
            "vegetarian".to_string(),
            "gluten_free".to_string()
        ]
    );
}
