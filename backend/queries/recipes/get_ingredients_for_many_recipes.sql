SELECT
ir.recipe_id,
ir.amount,
ir.notes,
ir.optional,
(
    i.id,
    i.name,
    i.description,
    i.diet_violations
) as "ingredient!: IngredientModel"
FROM ingredients_recipes AS ir
JOIN ingredients AS i
    ON i.id = ir.ingredient_id
WHERE ir.recipe_id = ANY($1)
