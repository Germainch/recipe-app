<script lang="ts">
import Button from "$lib/components/ui/button/Button.svelte";
import {handleSearchClick} from "$lib/controllers/recipeSearch";

export let mode: boolean
export let input: string

let handleClick = () => {
    handleSearchClick(mode,input)
}

</script>


<Button on:click={ handleClick }> Search </Button>