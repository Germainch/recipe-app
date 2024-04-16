import type {Ingredient} from "$lib/models/ingredient";

export type Recipe = {
    title: string;
    ingredients: Ingredient[];
    steps: string[];
}