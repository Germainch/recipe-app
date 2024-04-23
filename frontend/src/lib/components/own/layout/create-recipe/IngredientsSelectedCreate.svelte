<script lang="ts">

    import Button from "$lib/components/ui/button/Button.svelte";
    import type {Ingredient} from "$lib/models/ingredient";
    import {ingredients} from "$lib/stores";
    import {Card} from "$lib/components/ui/card";



    function removeIngredient(index: number) {
        ingredients.set($ingredients.filter((_, i) => i !== index)) ;
    }

</script>

<div id="selected-ingr-flex">
    {#each $ingredients as ingredient, index}
        <Card class="flex flex-row items-center justify-center text-[12px] p-1 ">
            <p> {ingredient.name} </p>
            <button class="p-1  hover:text-destructive"  on:click={ () => removeIngredient(index) }> X </button>
        </Card>
    {/each}
</div>


<style>
    #selected-ingr-flex{
        display: flex;
        gap: 0.5rem;
        align-items: center;
        justify-content: center;
        align-content: center;
        flex-direction: row;
        flex-wrap: wrap;
    }
</style>