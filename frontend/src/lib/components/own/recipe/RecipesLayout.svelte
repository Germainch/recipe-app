<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import RecipeCard from "$lib/components/own/recipe/RecipeCard.svelte";
    import type { Recipe } from "$lib/models/recipe";

    let recipes: Recipe[] = [
        {
            title: "Recipe 1",
            ingredients: [
                "Ingredient 1",
                "Ingredient 2",
                "Ingredient 3"
            ],
            steps: [
                "Step 1",
                "Step 2",
                "Step 3"
            ]
        },
        {
            title: "Recipe 2",
            ingredients: [
                "Ingredient 1",
                "Ingredient 2",
                "Ingredient 3"
            ],
            steps: [
                "Step 1",
                "Step 2",
                "Step 3"
            ]
        },
        {
            title: "Recipe 3",
            ingredients: [
                "Ingredient 1",
                "Ingredient 2",
                "Ingredient 3"
            ],
            steps: [
                "Step 1",
                "Step 2",
                "Step 3"
            ]
        },
        {
            title: "Recipe 4",
            ingredients: [
                "Ingredient 1",
                "Ingredient 2",
                "Ingredient 3"
            ],
            steps: [
                "Step 1",
                "Step 2",
                "Step 3"
            ]
        },
        {
            title: "Recipe 5",
            ingredients: [
                "Ingredient 1",
                "Ingredient 2",
                "Ingredient 3"
            ],
            steps: [
                "Step 1",
                "Step 2",
                "Step 3"
            ]
        },
        {
            title: "Recipe 6",
            ingredients: [
                "Ingredient 1",
                "Ingredient 2",
                "Ingredient 3"
            ],
            steps: [
                "Step 1",
                "Step 2",
                "Step 3"
            ]
        }
        ]
    ;
</script>

<div id="recipe-grid">
    {#each recipes as recipe}
        <RecipeCard {recipe} />
    {/each}
</div>

<style>
    #recipe-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));

        gap: 1rem;
        padding: 1rem;
    }
</style>