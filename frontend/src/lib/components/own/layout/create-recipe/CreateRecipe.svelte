<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import RecipeCard from "$lib/components/own/recipe/RecipeCard.svelte";
    import type {Recipe} from "$lib/models/recipe";

    let recipe: Recipe = {
        title: "Spaghetti Carbonara",
        ingredients: [
            { name: "Spaghetti" },
            { name: "Eggs" },
            { name: "Pancetta" },
            { name: "Parmesan" },
            { name: "Pepper" }
        ],
        steps: [
            "Cook the spaghetti",
            "Fry the pancetta",
            "Mix the eggs with the cheese and pepper",
            "Mix everything together"
        ]

    };
</script>

<AlertDialog.Root>
    <AlertDialog.Trigger>
        <RecipeCard {recipe}></RecipeCard>

    </AlertDialog.Trigger>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title> { recipe.title } </AlertDialog.Title>
            <AlertDialog.Description>
                <h3 class="font-bold underline"> Ingredients: </h3>
                <div id="dialog-ingredients">
                    {#each recipe.ingredients as ingredient}
                        <div>{ ingredient.name }</div>
                    {/each}
                </div>
                <h3 class="font-bold underline"> Steps: </h3>
                <div id="dialog-steps">
                    {#each recipe.steps as step, i}
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