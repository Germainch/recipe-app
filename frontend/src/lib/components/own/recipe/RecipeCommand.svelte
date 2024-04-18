<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { cn } from "$lib/utils.js";
    import {foundRecipes, selectedIngredients} from "$lib/stores";
    import type {Ingredient} from "$lib/models/ingredient";


    // The input value
    export let input = "";

    $: handleInputChange(input);

    function handleInputChange(input: string){
        console.log(input);
    }

</script>

<Command.Root>
    <Command.Input placeholder="Type the name of a recipe..." bind:value={input}/>
</Command.Root>
