use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    commands::ingredients::create::{create_ingredient, CreateIngredient, CreateIngredientError},
    entities::ingredient::{
        types::{DietViolations, WhichDiets},
        Ingredient,
    },
    repositories::ingredients::{IngredientRepository, IngredientRepositoryService},
    services::message::{MessageService, MessageServiceImpl},
};

pub async fn creates_an_ingredient(
    repo: impl IngredientRepository,
    message_service: impl MessageService,
) {
    let given = CreateIngredient {
        name: "Tomato",
        description: "Description of a tomato",
        diet_violations: vec!["Vegan".into()],
    };
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let ms: MessageServiceImpl = Arc::new(Box::new(message_service));

    let when = create_ingredient(repo.clone(), ms, &given).await.unwrap();

    // THEN

    assert_eq!(when.name.as_ref(), "Tomato");
    assert_eq!(when.description.as_ref(), "Description of a tomato");
    assert!(when.diet_violations.contains(&DietViolations::Vegan));
}

pub async fn incorrect_diets_do_not_get_included(
    repo: impl IngredientRepository,
    message_service: impl MessageService,
) {
    let given = CreateIngredient {
        name: "Tomato",
        description: "Description of a tomato",
        diet_violations: vec!["Vegan".into(), "INVALID DIET".into()],
    };

    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let ms: MessageServiceImpl = Arc::new(Box::new(message_service));

    let when = create_ingredient(repo.clone(), ms, &given).await.unwrap();

    // THEN

    assert!(when.diet_violations.contains(&DietViolations::Vegan));
    assert_eq!(when.diet_violations.len(), 1);
}

pub async fn empty_name_fails(
    repo: impl IngredientRepository,
    message_service: impl MessageService,
) {
    let given = CreateIngredient {
        name: "",
        description: "Description of a tomato",
        diet_violations: vec![],
    };

    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let ms: MessageServiceImpl = Arc::new(Box::new(message_service));

    let when = create_ingredient(repo.clone(), ms, &given)
        .await
        .unwrap_err();

    // THEN

    assert!(matches!(when, CreateIngredientError::EmptyField("name")));
}

pub async fn empty_description_fails(
    repo: impl IngredientRepository,
    message_service: impl MessageService,
) {
    let given = CreateIngredient {
        name: "Tomato",
        description: "",
        diet_violations: vec![],
    };

    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let ms: MessageServiceImpl = Arc::new(Box::new(message_service));

    let when = create_ingredient(repo.clone(), ms, &given).await.unwrap_err();

    // THEN

    assert!(matches!(
        when,
        CreateIngredientError::EmptyField("description")
    ));
}

pub async fn incorrect_ingredient_is_not_persisted(
    repo: impl IngredientRepository,
    message_service: impl MessageService,
) {
    let given = CreateIngredient {
        name: "",
        description: "Description of a tomato",
        diet_violations: vec![],
    };

    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let ms: MessageServiceImpl = Arc::new(Box::new(message_service));

    let when = create_ingredient(repo.clone(), ms, &given).await.unwrap_err();

    // THEN

    assert!(matches!(when, CreateIngredientError::EmptyField(_)));

    assert!(!&repo
        .get_all()
        .await
        .unwrap()
        .into_iter()
        .any(|x| x.name.as_str() == given.name))
}

pub async fn inserting_an_ingredient_with_a_name_that_already_exists_fails(
    repo: impl IngredientRepository,
    message_service: impl MessageService,
) {
    let given = Ingredient {
        id: Uuid::from_u128(1),
        name: "Ingredient name".try_into().unwrap(),
        description: "Ingredient description".try_into().unwrap(),
        diet_violations: WhichDiets::new(),
    };
    let repo: IngredientRepositoryService = Arc::new(Box::new(repo));
    let ms: MessageServiceImpl = Arc::new(Box::new(message_service));

    repo.insert(given.clone()).await.unwrap();

    let result = create_ingredient(
        repo,
        ms,
        &CreateIngredient {
            name: given.name.as_str(),
            description: "This is a different description",
            diet_violations: vec![],
        },
    )
    .await
    .unwrap_err();

    assert!(matches!(
        result,
        CreateIngredientError::Conflict(fieldname) if fieldname == "name"
    ))
}
