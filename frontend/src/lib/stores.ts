import {writable, type Writable} from "svelte/store";
import {type AppState, Routes} from "$lib/state";
import type {Ingredient} from "$lib/models/ingredient";

export const currentState: Writable<AppState> = writable({
    route: Routes.home,
    isLogged: false,
});

export const selectedIngredients: Writable<Ingredient[]> = writable([]);