<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import RecipeCard from "$lib/components/own/recipe/RecipeCard.svelte";
    import type {Recipe} from "$lib/models/recipe";
    import IngredientCombobox from "$lib/components/own/ingredient-form/IngredientCombobox.svelte";
    import * as Card from "$lib/components/ui/card";
    import {ingredients} from "$lib/stores.js";
    import {steps} from "$lib/stores.js";
    import Button from "$lib/components/ui/button/Button.svelte";
    import IngredientsSelected from "$lib/components/own/ingredient-form/IngredientsSelected.svelte";
    import IngredientComboboxCreate from "$lib/components/own/layout/create-recipe/IngredientComboboxCreate.svelte";
    import IngredientsSelectedCreate from "$lib/components/own/layout/create-recipe/IngredientsSelectedCreate.svelte";
    import {createRecipe} from "$lib/controllers/recipeSearch";
    import type {RecipeAndIngredients} from "$lib/models/recipeAndIngredients";


    let recipeCreated: Recipe = {
        id: 0,
        name: "",
        steps: "",
        created_by: undefined,
    };

    let recipeAndIngredients: RecipeAndIngredients = {
        recipe: recipeCreated,
        ingredients: [],
    }

    let userSteps = $steps;

    function addStep() {
        $steps = [...$steps, ""];
    }


    function removeStep(i:number){
       $steps.splice(i , 1);
       $steps = [...$steps];
       console.log($steps);
    }

    function handleInputChange(i: number){
        $steps[i] = (document.getElementById(`step${i}`) as HTMLInputElement).value;
    }

    function handleKeyPress(event){
        if (event.key === "Enter" || event.type === "click") {
            addStep();
        }
    }

    function handleCreateRecipe(){
        recipeCreated.name = (document.getElementById("title") as HTMLInputElement).value;
        recipeCreated.steps = $steps.join("\n");

        recipeAndIngredients.recipe = recipeCreated;
        recipeAndIngredients.ingredients = $ingredients;

        console.log(recipeCreated);
        createRecipe(recipeAndIngredients);
    }
</script>

<Card.Root class="border-[2px]  hover:border-primary flex flex-col">
    <Card.Header>
        <Card.Title>
            <input class="p-3 bg-secondary dark:text-white" type="text" name="Title" id="title" placeholder="recipe name...">
        </Card.Title>
        <Card.Description>

            <IngredientComboboxCreate/>
            <IngredientsSelectedCreate/>

        </Card.Description>

    </Card.Header>
    <Card.Content class="flex flex-col items-center">
        <!-- -->
        Steps:
        <div class="flex flex-col">
            {#each $steps as step, i}
                <div class="flex flex-row p-1">
                    <h3 class="p-2">{i + 1}</h3>
                    <input class="border-2 border-secondary bg-secondary dark:text-white p-1" value={step} on:input={() => handleInputChange(i)} on:keypress={handleKeyPress} type="text" name="Step" id="step{i}"  placeholder="step...">
                    <button class="text-destructive p-1" on:click={ () => removeStep(i) }>X</button>
                </div>
            {/each}
        </div>

        <Button variant="secondary" class="p-2" on:click={addStep}>Add Step</Button>

    </Card.Content>
</Card.Root>
<div class="flex items-center justify-center p-4">
    <Button variant="default" on:click={handleCreateRecipe}> Create Recipe </Button>
</div>
