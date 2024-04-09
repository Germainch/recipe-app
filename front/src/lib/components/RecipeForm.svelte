<script>
    import IngredientSearch from "$lib/components/IngredientSearch.svelte";
</script>

<div id="recipe-form-container" class="card border-2 shadow-2xl border-primary-800">

    <form action="/api/createRecipe" method="post" class="form-input dark:variant-glass-tertiary bg-secondary-100 flex flex-col gap-2">
        <label for="recipeName">Name</label>
        <input type="text" id="recipeName" placeholder="Your recipe name.." class="text-black" required>


        <IngredientSearch></IngredientSearch>
        <label for="recipe-instructions">Instructions</label>
        <textarea name="instructions" id="recipe-instructions" class="text-black"></textarea>
        <button type="submit" class="btn btn-md dark:variant-filled-primary variant-filled-secondary m-2">Create</button>
    </form>

</div>