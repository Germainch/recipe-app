<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import RecipeCard from "$lib/components/own/recipe/RecipeCard.svelte";
    import {DefaultRecipe, type Recipe} from "$lib/models/recipe";
    import RecipeAlertDialog from "$lib/components/own/recipe/RecipeAlertDialog.svelte";

    let recipes: Recipe[] = [
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        DefaultRecipe,
        ]

    ;
</script>

<div id="recipe-grid">
    {#each recipes as recipe}
        <RecipeAlertDialog {recipe} />
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