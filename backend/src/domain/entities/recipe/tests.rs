use uuid::Uuid;

use crate::domain::entities::{
    ingredient::{
        types::{IngredientDescription, IngredientName, WhichDiets},
        Ingredient, IngredientModel,
    },
    recipe::errors::ValidationError,
};

use super::{IngredientWithAmount, IngredientWithAmountModel};

#[test]
fn converting_ingredient_with_amount_works() {
    let input = IngredientWithAmountModel {
        recipe_id: Uuid::nil(),
        ingredient: IngredientModel {
            id: Uuid::nil(),
            name: "Ingredient name".to_owned(),
            description: "Ingredient description".to_owned(),
            diet_violations: vec![],
        },
        amount: serde_json::json!({
            "grams": 20
        }),
        notes: None,
        optional: false,
    };

    let expected = IngredientWithAmount {
        ingredient: Ingredient {
            id: Uuid::nil(),
            name: IngredientName("Ingredient name".to_owned()),
            description: IngredientDescription("Ingredient description".to_owned()),
            diet_violations: WhichDiets::new(),
        },
        amount: super::IngredientUnit::Grams(20.0),
        notes: None,
        optional: false,
    };

    let result: IngredientWithAmount = input.try_into().unwrap();

    assert_eq!(result, expected);
}

#[test]
fn converting_ingredient_with_custom_amount_unit_works() {
    let input = IngredientWithAmountModel {
        recipe_id: Uuid::nil(),
        ingredient: IngredientModel {
            id: Uuid::nil(),
            name: "Ingredient name".to_owned(),
            description: "Ingredient description".to_owned(),
            diet_violations: vec![],
        },
        amount: serde_json::json!({
            "other": {
                "amount": 10,
                "unit": "cloves"
            }
        }),
        notes: None,
        optional: false,
    };

    let expected = IngredientWithAmount {
        ingredient: Ingredient {
            id: Uuid::nil(),
            name: IngredientName("Ingredient name".to_owned()),
            description: IngredientDescription("Ingredient description".to_owned()),
            diet_violations: WhichDiets::new(),
        },
        amount: super::IngredientUnit::Other {
            unit: "cloves".to_owned(),
            amount: 10.0,
        },
        notes: None,
        optional: false,
    };

    let result: IngredientWithAmount = input.try_into().unwrap();

    assert_eq!(result, expected);
}

#[test]
fn converting_ingredient_with_custom_amount_unit_but_without_unit_descriptor_fails() {
    let input = IngredientWithAmountModel {
        recipe_id: Uuid::nil(),
        ingredient: IngredientModel {
            id: Uuid::nil(),
            name: "Ingredient name".to_owned(),
            description: "Ingredient description".to_owned(),
            diet_violations: vec![],
        },
        amount: serde_json::json!({
            "other": {
                "amount": 10,
            }
        }),
        notes: None,
        optional: false,
    };

    let result: ValidationError =
        std::convert::TryInto::<IngredientWithAmount>::try_into(input).unwrap_err();

    assert!(matches!(
        result,
        ValidationError::DeserializationFailed("amount", _)
    ))
}

#[test]
fn malformed_ingredient_amount_fails() {
    let input = IngredientWithAmountModel {
        recipe_id: Uuid::nil(),
        ingredient: IngredientModel {
            id: Uuid::nil(),
            name: "Ingredient name".to_owned(),
            description: "Ingredient description".to_owned(),
            diet_violations: vec![],
        },
        amount: serde_json::json!("10 grams"),
        notes: None,
        optional: false,
    };

    let result: ValidationError =
        std::convert::TryInto::<IngredientWithAmount>::try_into(input).unwrap_err();

    assert!(matches!(
        result,
        ValidationError::DeserializationFailed("amount", _)
    ))
}
