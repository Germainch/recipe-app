<script>
    import IngredientCombobox from "$lib/components/own/ingredient-form/IngredientCombobox.svelte";
    import * as Card from "$lib/components/ui/card";
</script>

<div class="h-fit w-full p-3 flex flex-col" >
    <Card.Root>
        <Card.Header>
            <Card.Title>
                <IngredientCombobox></IngredientCombobox>
            </Card.Title>
            <Card.Description>  </Card.Description>
        </Card.Header>
        <Card.Content class="flex flex-col items-center">
            <p> <button class="btn btn-primary">Add</button> </p>
        </Card.Content>
    </Card.Root>
</div>
