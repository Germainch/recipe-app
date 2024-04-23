import {foundRecipes, selectedIngredients, userRecipes} from "$lib/stores";
import type {Recipe} from "$lib/models/recipe";
import {backendURL} from "$lib/stores";
import type {Ingredient} from "$lib/models/ingredient";



export function getRecipes(mode: boolean, recipeInput: string) {
    let ingredients: Ingredient[] = [];
    const unsubscribe = selectedIngredients.subscribe((value) => ingredients = value);

    if (mode) {
        getRecipesByName(recipeInput);
    } else {
        getRecipesByIngredients(ingredients);
    }
    unsubscribe();
}

export function getSavedRecipes() {
    let url = backendURL + "/recipes/saved";
    let params = "?sessionID=" + localStorage.getItem("sessionID");

    fetch(url + params, {
        method: "GET",
    })
        .then(response => response.json())
        .then(data => {
            userRecipes.set(data);
        })
}

export function saveRecipe(recipe: Recipe){
    // update the local user recipe store
    userRecipes.update(recipes => { return [...recipes, recipe] });
    // update the server
    let url = backendURL + "/recipes/saved";
    let params = "&recipeID=" + recipe.id;
    fetch(url + params, {
        method: "POST",
    })
        .then(response => response.json())
        .then(data => {
            console.log(data);
        })
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

export function getRecipesByName(recipeName: string){
    let url = backendURL + "/recipes/search-by-name";
    let params = "?name=" + recipeName;
    fetch(url + params, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        }
    }).then(response => response.json())
        .then(data => {
            foundRecipes.set(data);
        })
        .catch((error) => {
            console.error("Error:", error);
        });
}

export function getRecipesByIngredients(ingredients: Ingredient[]){
    let url = backendURL + "/recipes/search-by-ingredients";
    let params = "?ingredients=" + ingredients.join(",");
    fetch(url + params, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        }
    }).then(response => response.json())
        .then(data => {
            foundRecipes.set(data);
        })
        .catch((error) => {
            console.error("Error:", error);
        });

}

export function createRecipe(recipe: Recipe) {
    let url = backendURL + "/recipes";
    fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + localStorage.getItem("sessionID"),
        },
        body: JSON.stringify(recipe),
    })
        .then(response => response.json())
        .then(data => {
            console.log(data);
        })
        .catch((error) => {
            console.error("Error:", error);
        });
}