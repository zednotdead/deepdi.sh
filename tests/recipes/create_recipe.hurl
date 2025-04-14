POST http://localhost:8111/ingredient

{
    "name": "cucumber",
    "description": "Not a cumcuber"
}

HTTP 201
[Captures]
id: jsonpath "$.id"

POST http://localhost:8111/recipe

{
  "name": "Testing recipe",
  "description": "This is a recipe I am doing for testing",
  "steps": ["Do a test"],
  "time": {
    "Prep time": 1200
  },
  "ingredients": [
    {
      "ingredient_id": "{{id}}",
      "amount": {
        "_type": "grams",
        "amount": 100.0
      },
      "optional": false,
      "notes": null
    }
  ],
  "servings": {
    "from_to": [2, 4]
  }
}

HTTP 201
