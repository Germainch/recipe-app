<script>
import IngredientSearch from "$lib/components/IngredientSearch.svelte";
</script>

<div class=" flex flex-col content-center justify-items-center items-center p-3 w-full h-full justify-center">
    <form method="post" class="form-input dark:variant-glass-tertiary bg-secondary-100 flex flex-col gap-2" >
        <IngredientSearch></IngredientSearch>
        <button class="btn btn-md dark:variant-filled-primary variant-filled-secondary m-2">
            Search
        </button>
    </form>
</div>