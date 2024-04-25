<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import RecipeCard from "$lib/components/own/recipe/RecipeCard.svelte";
    import type {Recipe} from "$lib/models/recipe";

    export let recipe;

    function parseSteps(recipe:Recipe): String[]{
        return recipe.steps.split("\n");
    }
    let steps = parseSteps(recipe);
</script>

<AlertDialog.Root>
    <AlertDialog.Trigger>
        <RecipeCard recipe={recipe}></RecipeCard>
    </AlertDialog.Trigger>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title> { recipe.name } </AlertDialog.Title>
            <AlertDialog.Description>
                <h3 class="font-bold underline"> Ingredients: </h3>
                <h3 class="font-bold underline"> Steps: </h3>
                <div id="dialog-steps">
                    {#each steps as step, i}
                        <div> {i} : { step }</div>
                    {/each}
                </div>
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel> Close </AlertDialog.Cancel>
            <AlertDialog.Action> Save Recipe </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>


<style>
    #dialog-ingredients {
        margin-top: 1rem;
        display: flex;
        justify-content: center;
    }

    #dialog-steps {
        margin-top: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;

    }
</style>