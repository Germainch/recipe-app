<script lang="ts">
    import RecipesLayout from "$lib/components/own/recipe/RecipesLayout.svelte";
    import Sidebar from "$lib/components/own/layout/Sidebar.svelte";
    import {Routes} from "$lib/state";
    import {currentState} from "$lib/stores";
    import IngredientCombobox from "$lib/components/own/ingredient-form/IngredientCombobox.svelte";
    import IngredientForm from "$lib/components/own/ingredient-form/IngredientForm.svelte";

    $: console.log($currentState.route);
</script>

<main id="main">

    <Sidebar/>

    {#if $currentState.route === Routes.search}
        <div>
            <IngredientForm/>
            <RecipesLayout></RecipesLayout>
        </div>
    {:else if $currentState.route === Routes.create}
        <IngredientCombobox></IngredientCombobox>
    {:else if $currentState.route === Routes.home}
        <IngredientCombobox></IngredientCombobox>
    {:else if $currentState.route === Routes.myRecipes}
        <RecipesLayout></RecipesLayout>
    {/if}

</main>



<style>
    #main{
        display: grid;
        grid-template-rows: 1fr;
        grid-template-columns: 1fr 6fr;
        gap: 4px;
    }
</style>