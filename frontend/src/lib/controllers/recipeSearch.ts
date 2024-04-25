import {foundRecipes, selectedIngredients, userRecipes} from "$lib/stores";
import type {Recipe} from "$lib/models/recipe";
import {backendURL} from "$lib/stores";
import type {Ingredient} from "$lib/models/ingredient";
import type {RecipeAndIngredients} from "$lib/models/recipeAndIngredients";



export async function getRecipes(mode: boolean, recipeInput: string):Promise<Recipe[]>{
    let ingredients: Ingredient[] = [];
    const unsubscribe = selectedIngredients.subscribe((value) => ingredients = value);
    let result:Recipe[] = [];
    console.log(ingredients)
    if (mode) {
       result = await getRecipesByName(recipeInput);
    } else {
       result = await getRecipesByIngredients(ingredients);
    }
    unsubscribe();
    return result;
}

export async function getSavedRecipes() {
    let url = backendURL + "/recipes/saved/";
    let params = sessionStorage.getItem("sessionID") ?? "";

    const response = await fetch(url + params, {
        method: "GET",
    })
    const data: Recipe[] = await response.json();
    console.log(data);
    userRecipes.set(data);
}

export async function saveRecipe(recipe: Recipe){
    // update the local user recipe store
    userRecipes.update(recipes => { return [...recipes, recipe] });

    // update the server
    let url = (backendURL + "/recipes/saved/session/"+ sessionStorage.getItem("sessionID") + "/recipe/" + recipe.id);
    const response = await fetch(url , {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
    })
    console.log("SAVE RECIPE RESPONSE:" , response.status);
}

export function deleteRecipe(recipe: Recipe){
    // update the local user recipe store
    userRecipes.update(recipes => { return recipes.filter(item => item.id !== recipe.id) });
    // update the server
    let url = backendURL +" /recipes/saved";
    let params = "?recipeID=" + recipe.id;
    fetch(url + params, {
        method: "DELETE",
    })
        .then(response => response.json())
        .then(data => {
            console.log(data);
        })
}

export async function getRecipesByName(recipeName: string): Promise<Recipe[]> {
    let url = backendURL + "/recipes/search-by-name/";
    let result: Recipe[] = [];
    try {
        const response = await fetch(url + recipeName, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            }
        })
        const data = await response.json();
        console.log(data);
        result = data;
    } catch (error) {
        console.error("Error:", error);
    }
    return result
}

export async function getRecipesByIngredients(ingredients: Ingredient[]): Promise<Recipe[]>{
    let url = backendURL + "/recipes/search-by-ingredients";
    let recipes: Recipe[] = [];
    try {
        const response = await fetch(url , {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(ingredients),
        });
        const data = await response.json();
        console.log(data);
        recipes = data;
    } catch (error) {
        console.error("Error:", error);
    }
    return recipes;
}

export function createRecipe(recipe: RecipeAndIngredients) {
    let sessionID = sessionStorage.getItem("sessionID") ?? "";
    let url = backendURL + "/recipes/create/" + sessionID;
    let body = JSON.stringify(recipe);
    console.log(body);
    fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: body,
    })
        .then(data => {
            console.log(data);
        })
        .catch((error) => {
            console.error("Error:", error);
        });
}