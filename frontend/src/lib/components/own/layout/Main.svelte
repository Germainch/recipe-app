<script lang="ts">
    import RecipesLayout from "$lib/components/own/recipe/RecipesLayout.svelte";
    import {Routes} from "$lib/state";
    import {currentState} from "$lib/stores";
    import IngredientCombobox from "$lib/components/own/ingredient-form/IngredientCombobox.svelte";
    import IngredientForm from "$lib/components/own/ingredient-form/IngredientForm.svelte";
    import SubHeader from "$lib/components/own/layout/SubHeader.svelte";
    import RecipeCard from "$lib/components/own/recipe/RecipeCard.svelte";
    import RecipeAlertDialog from "$lib/components/own/recipe/RecipeAlertDialog.svelte";
    import {DefaultRecipe} from "$lib/models/recipe";
    import Search from "$lib/components/own/layout/search-recipe/Search.svelte";
    import SavedRecipes from "$lib/components/own/layout/saved-recipes/SavedRecipes.svelte";
    import CreateRecipe from "$lib/components/own/layout/create-recipe/CreateRecipe.svelte";


    $: console.log($currentState.route);
</script>

<main id="main" class="flex flex-col">

    <SubHeader/>

    {#if $currentState.route === Routes.search}
        <div>
            <Search></Search>
            <RecipesLayout></RecipesLayout>
        </div>
    {:else if $currentState.route === Routes.create}
        <CreateRecipe></CreateRecipe>
    {:else if $currentState.route === Routes.home}
        <IngredientCombobox></IngredientCombobox>
    {:else if $currentState.route === Routes.myRecipes}
        <SavedRecipes></SavedRecipes>
    {/if}

</main>

