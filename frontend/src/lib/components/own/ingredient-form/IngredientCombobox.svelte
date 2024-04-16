<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { cn } from "$lib/utils.js";
    import {onMount, tick} from "svelte";
    import {selectedIngredients} from "$lib/stores";
    import type {Ingredient} from "$lib/models/ingredient";


    let ingredients: Ingredient[] = [];

    onMount(async () => {
        if (ingredients.length > 0) return;
        const response = await fetch("http://localhost:3001/ingredients",{
            method: "GET",
        });
        console.log("GET request to /ingredients")
        const data = await response.json();
        ingredients.push(...data);
    });

    let open = false;
    let value = "";

    $: selectedValue =
        ingredients.find((f) => f.name === value)?.name ??
        "Select an Ingredient...";

    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger(triggerId: string) {
        open = false;
        tick().then(() => {
            document.getElementById(triggerId)?.focus();
        });

    }

</script>
<Popover.Root bind:open let:ids>
    <Popover.Trigger asChild let:builder>
        <Button
                builders={[builder]}
                variant="secondary"
                role="search"
                aria-expanded={open}
                class="w-[200px] justify-between border-2 border-secondary-foreground opacity-70 hover:opacity-100"
        >
            {selectedValue}
        </Button>
    </Popover.Trigger>
    <Popover.Content class="w-[200px] p-0">
        <Command.Root>
            <Command.Input placeholder="Search framework..." />
            <Command.Empty>No framework found.</Command.Empty>
            <Command.Group>
                {#each ingredients as ingredient}
                    <Command.Item

                        value={ingredient.name}
                        onSelect={(currentValue) => {
                            value = currentValue;

                            // update the selectedIngredients store
                            selectedIngredients.update((selected) => {
                                if (selected.includes(ingredient)) {
                                    return selected;
                                }
                                return [...selected, ingredient];
                            });

                            closeAndFocusTrigger(ids.trigger);
                        }}
                    >
                        <Check class={cn( "mr-2 h-4 w-4", value !== ingredient.name && "text-transparent")} />
                            {ingredient.name}

                    </Command.Item>
                {/each}
            </Command.Group>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
