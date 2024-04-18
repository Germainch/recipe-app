<script lang="ts">
    import IngredientCombobox from "$lib/components/own/ingredient-form/IngredientCombobox.svelte";
    import * as Card from "$lib/components/ui/card";
    import Button from "$lib/components/ui/button/Button.svelte";
    import IngredientsSelected from "$lib/components/own/ingredient-form/IngredientsSelected.svelte";
    import IngredientInput from "$lib/components/own/ingredient-form/IngredientInput.svelte";
    import { Toggle } from "$lib/components/ui/toggle";
    import * as ToggleGroup from "$lib/components/ui/toggle-group";
    import {Switch} from "$lib/components/ui/switch";
    import RecipeCombobox from "$lib/components/own/recipe/RecipeCommand.svelte";

    let mode: boolean = false;

</script>

<div class="h-fit w-full p-3" >
    <Card.Root class="flex flex-col items-center">
        <Card.Header>
            <Card.Title>
                {#if mode}
                    <RecipeCombobox></RecipeCombobox>
                {:else}
                    <IngredientCombobox></IngredientCombobox>
                {/if}
            </Card.Title>
            <Card.Description>

                <div>
                    <Switch id="mode" bind:checked={mode}/>
                    <label for="mode">
                        {#if mode}
                            Recipe
                        {:else}
                            Ingredient
                        {/if}
                    </label>
                    
                </div>

            </Card.Description>
        </Card.Header>

            <div id="selected-ingredients" class="self-stretch p-3">
                <IngredientsSelected></IngredientsSelected>
            </div>

        <Card.Content>

            <Button> Search </Button>

        </Card.Content>
    </Card.Root>
</div>
