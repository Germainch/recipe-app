import type {Ingredient} from "$lib/models/ingredient";
import type {Recipe} from "$lib/models/recipe";

export type RecipeAndIngredients = {
    recipe: Recipe,
    ingredients: Ingredient[]
}