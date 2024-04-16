<script lang="ts">
    import RecipesLayout from "$lib/components/own/recipe/RecipesLayout.svelte";
    import {Routes} from "$lib/state";
    import {currentState} from "$lib/stores";
    import IngredientCombobox from "$lib/components/own/ingredient-form/IngredientCombobox.svelte";
    import IngredientForm from "$lib/components/own/ingredient-form/IngredientForm.svelte";
    import SubHeader from "$lib/components/own/layout/SubHeader.svelte";
    import RecipeCard from "$lib/components/own/recipe/RecipeCard.svelte";

    $: console.log($currentState.route);
</script>

<main id="main" class="flex flex-col">

    <SubHeader/>

    {#if $currentState.route === Routes.search}
        <div>
            <IngredientForm/>
            <RecipesLayout></RecipesLayout>
        </div>
    {:else if $currentState.route === Routes.create}
        <RecipeCard recipe={ {title: "alo", ingredients: [{id:1, name:"hha"}], steps: ["manger"]} }></RecipeCard>
    {:else if $currentState.route === Routes.home}
        <IngredientCombobox></IngredientCombobox>
    {:else if $currentState.route === Routes.myRecipes}
        <RecipesLayout></RecipesLayout>
    {/if}

</main>

