<script lang="ts">

    import Button from "$lib/components/ui/button/Button.svelte";
    import type {Ingredient} from "$lib/models/ingredient";

    export let ingredients: Ingredient[] = [];

    function removeIngredient(index: number) {
        ingredients = ingredients.filter((_, i) => i !== index);
    }
</script>

{#each ingredients as ingredient}

    <p> {ingredient.name} </p>
    <Button>X</Button>
{/each}