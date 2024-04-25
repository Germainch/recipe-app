
<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import type {Recipe} from "$lib/models/recipe";
    export let recipe:Recipe;

    function handleClick(){
        console.log("clicked");
    }

</script>


<Card.Root class="border-[2px]  hover:border-primary">
    <Card.Header>
        <Card.Title> {recipe.name} </Card.Title>
        <Card.Description>
        </Card.Description>
    </Card.Header>
    <Card.Content>
        <!-- -->
    </Card.Content>
</Card.Root>