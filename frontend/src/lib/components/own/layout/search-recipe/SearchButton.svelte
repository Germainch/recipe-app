<script lang="ts">
import Button from "$lib/components/ui/button/Button.svelte";
import {getRecipes} from "$lib/controllers/recipeSearch";

export let mode: boolean
export let input: string

let handleClick = () => {
    getRecipes(mode,input)
}

</script>


<Button on:click={ handleClick }> Search </Button>