<script>
    import IngredientCombobox from "$lib/components/own/ingredient-form/IngredientCombobox.svelte";
    import * as Card from "$lib/components/ui/card";
    import Button from "$lib/components/ui/button/Button.svelte";
    import IngredientsSelected from "$lib/components/own/ingredient-form/IngredientsSelected.svelte";
</script>

<div class="h-fit w-full p-3" >
    <Card.Root class="flex flex-col items-center">
        <Card.Header>
            <Card.Title>
                <IngredientCombobox></IngredientCombobox>
            </Card.Title>
            <Card.Description>  </Card.Description>
        </Card.Header>

        <div id="selected-ingredients" class="self-stretch p-3">
            <IngredientsSelected></IngredientsSelected>
        </div>
        <Card.Content>
            <Button> Search </Button>
        </Card.Content>
    </Card.Root>
</div>
