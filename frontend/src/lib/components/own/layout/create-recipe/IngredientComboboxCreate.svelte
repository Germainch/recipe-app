<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { cn } from "$lib/utils.js";
    import {onMount, tick} from "svelte";
    import {ingredients} from "$lib/stores";
    import type {Ingredient} from "$lib/models/ingredient";



    // The value the ingredients list shown on popover
    let ingredientsPopover: Ingredient[] = [];

    // popover state
    let open = false;

    // The selected value
    let selValue = "";

    // The input value
    let value = "";


    $: selectedValue =
        ingredientsPopover.find((f) => f.name === selValue)?.name ??
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


    async function fetchData() {

        let url = `http://localhost:3001/ingredients/containsStr/${value}`;

        await fetch( url , {
            method: "GET",
        })
            .then((response) => response.json())
            .then((data) => {
                console.log(data);
                ingredientsPopover = data;
            })

    }

    function handleInputChange(e: any){
        if (value.length <= 3){
            return;
        }

        fetchData().then(
            () => console.log(ingredientsPopover)
        );
    }


</script>
<Popover.Root bind:open let:ids>
    <Popover.Trigger asChild let:builder>
        <Button
                builders={[builder]}
                variant="secondary"
                role="search"
                aria-expanded={open}
                class="w-[200px] justify-between border-2 border-secondary-foreground opacity-70 hover:border-primary"
        >
        Select Ingredients...
        </Button>
    </Popover.Trigger>
    <Popover.Content class="flex flex-col content-stretch">
        <Command.Root>
                <input id="search" type="search" autocomplete="off" bind:value on:input={handleInputChange} placeholder="Search..." class="border-3 bg-secondary border-input p-3" aria-label="search"/>
            <Command.Empty>No Ingredients found.</Command.Empty>
            <Command.Group>
                {#each ingredientsPopover as ingredient}
                    <Command.Item

                        value={ingredient.name}
                        onSelect={(currentValue) => {
                            selValue = currentValue;

                            // update the selectedIngredients store
                            ingredients.update((selected) => {
                                if (selected.includes(ingredient)) {
                                    return selected;
                                }
                                return [...selected, ingredient];
                            });

                            closeAndFocusTrigger(ids.trigger);
                        }}
                    >
                        <Check class={cn( "mr-2 h-4 w-4", selValue !== ingredient.name && "text-transparent")} />
                            {ingredient.name}

                    </Command.Item>
                {/each}
            </Command.Group>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
