
<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import type {Recipe} from "$lib/models/recipe";
    export let recipe:Recipe;
</script>


<Card.Root>
    <Card.Header>
        <Card.Title> {recipe.title} </Card.Title>
        <Card.Description> {recipe.ingredients} </Card.Description>
    </Card.Header>
    <Card.Content>
        <p> {recipe.steps} </p>
    </Card.Content>
</Card.Root>