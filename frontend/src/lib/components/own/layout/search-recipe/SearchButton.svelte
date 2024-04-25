<script lang="ts">
    import Button from "$lib/components/ui/button/Button.svelte";
    import {getRecipes} from "$lib/controllers/recipeSearch";
    import type {Recipe} from "$lib/models/recipe";
    import {foundRecipes} from "$lib/stores";

    export let mode: boolean
export let input: string

let handleClick = () => {
    getRecipes(mode, input)
        .then((recipes) => {
            foundRecipes.set(recipes);
            console.log("FOUND RECIPES : " + $foundRecipes)
            $foundRecipes = [...$foundRecipes];
        });
}

</script>


<Button on:click={ handleClick }> Search </Button>