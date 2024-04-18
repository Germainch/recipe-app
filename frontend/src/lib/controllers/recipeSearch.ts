import {foundRecipes, selectedIngredients, userRecipes} from "$lib/stores";
import type {Recipe} from "$lib/models/recipe";
import {backendURL} from "$lib/stores";



export function handleSearchClick(mode :boolean, recipeInput :string) {
    console.log(recipeInput)
    console.log(mode)
    let endpoint = ""
    let url = "http://localhost:3000/recipes"
    let params= ""
    let ingredientsParam: string = ""

    if (mode) {
        console.log("searching for a recipe by name")
        endpoint = "recipes/search-by-name";
        params = "?name="+recipeInput;
    } else {
        console.log("searching for a recipe by ingredients")
        endpoint = "recipes/search-by-ingredients";

        // subscribe to the selectedIngredients store and set the ingredientsParam to the selected ingredients
        selectedIngredients.subscribe(ingredients => ingredientsParam = ingredients.map(ingredient => ingredient.id).join(","))
        params = "?ingredients="+ingredientsParam;
        console.log(params);
    }

    fetch(url + endpoint + params, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "Authorization": "Bearer " + localStorage.getItem("sessionID"),
        },
    })
        .then(response => response.json())
        .then(data => {
            foundRecipes.set(data);
        })
        .catch((error) => {
            console.error("Error:", error);
        });
}

export function handleLoadUserRecipes() {
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

export function addUserRecipes(recipe: Recipe){
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

export function deleteUserRecipe(recipe: Recipe){
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