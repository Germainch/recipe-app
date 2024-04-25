import {writable, type Writable} from "svelte/store";
import {type AppState, Routes} from "$lib/state";
import type {Ingredient} from "$lib/models/ingredient";
import type {Recipe} from "$lib/models/recipe";

export const currentState: Writable<AppState> = writable({
    route: Routes.home,
    isLogged: false,
});

export const selectedIngredients: Writable<Ingredient[]> = writable([]);
export const userRecipes: Writable<Recipe[]> = writable([]);
export const foundRecipes: Writable<Recipe[]> = writable([]);

// RECIPE CREATION
export const steps: Writable<string[]> = writable([]);
export const ingredients: Writable<Ingredient[]> = writable([]);

export const backendURL = "http://localhost:3001";

