
<script lang="ts">

    import { onMount } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import type {Ingredient} from "$lib/models/ingredient";
    import {selectedIngredients} from "$lib/stores";
    const dispatch = createEventDispatcher();

    let searchInput = '';
    let searchResults: Ingredient[] = [];
    let selectedIngredient: Ingredient | null = null;

    async function fetchData() {

        let url = `http://localhost:3001/ingredients/containsStr/${searchInput}`;

        await fetch( url , {
            method: "GET",
        })
            .then((response) => response.json())
            .then((data) => {
                console.log(data);
                searchResults = data;
            })

    }

    function handleInputChange(e: any){
        if (searchInput.length <= 3){
            return;
        }

        fetchData().then(
            () => console.log(searchResults)
        );
    }
    onMount(() => {
        // Initial fetch when the component is mounted
        searchResults = [
            {
                id: 1,
                name: 'Ingredient 1'
            },
            {
                id: 2,
                name: 'Ingredient 2'
            }
        ]
    });

    $: dispatch('results', searchResults); // Dispatch an event with the search results
</script>





<div>
    <input type="text" bind:value={searchInput} on:input={handleInputChange} placeholder="Search..." />
    <ul>
        {#each searchResults as result}
            <li>{result.name}</li> <!-- Display your search results here -->
        {/each}
    </ul>
</div>