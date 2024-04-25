<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import RecipeCard from "$lib/components/own/recipe/RecipeCard.svelte";
    import {DefaultRecipe, type Recipe} from "$lib/models/recipe";
    import RecipeAlertDialog from "$lib/components/own/recipe/RecipeAlertDialog.svelte";
    import {foundRecipes} from "$lib/stores";

</script>

<div id="recipe-grid">
    {#each $foundRecipes as recipe}
        <RecipeAlertDialog {recipe} />
    {/each}
</div>
{#if $foundRecipes.length === 0}
    <div class="flex flex-col items-center">
        <p>No recipes found</p>
        <p>There are no recipes that match your search criteria.</p>
    </div>
{/if}

<style>
    #recipe-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));

        gap: 1rem;
        padding: 1rem;
    }
</style>