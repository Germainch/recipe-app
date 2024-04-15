<script lang="ts">
    import Check from "lucide-svelte/icons/check";
    import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { cn } from "$lib/utils.js";
    import { tick } from "svelte";

    import Ingredient from "$lib/components/own/ingredient-form/Ingredient.svelte";



    const ingredients = [
        {
            value: "sveltekit",
            label: "SvelteKit",
        },
        {
            value: "next.js",
            label: "Next.js",
        },
        {
            value: "nuxt.js",
            label: "Nuxt.js",
        },
        {
            value: "remix",
            label: "Remix",
        },
        {
            value: "astro",
            label: "Astro",
        },
    ];



    let open = false;
    let value = "";

    $: selectedValue =
        ingredients.find((f) => f.value === value)?.label ??
        "Select an Ingredient...";
    let valueArray : string[] = [];
    // We want to refocus the trigger button when the user selects
    // an item from the list so users can continue navigating the
    // rest of the form with the keyboard.
    function closeAndFocusTrigger(triggerId: string) {
        open = false;
        tick().then(() => {
            document.getElementById(triggerId)?.focus();
        });
        console.log(valueArray);
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
                {#each ingredients as framework}
                    <Command.Item

                        value={framework.value}
                        onSelect={(currentValue) => {
                            value = currentValue;
                            valueArray.push(currentValue);
                            valueArray=valueArray;
                            closeAndFocusTrigger(ids.trigger);
                        }}
                    >
                        <Check class={cn( "mr-2 h-4 w-4", value !== framework.value && "text-transparent")} />
                            {framework.label}

                    </Command.Item>
                {/each}
            </Command.Group>
        </Command.Root>
    </Popover.Content>
</Popover.Root>

<div    class="w-max h-max bg-primary">
    {#each valueArray as item}
        <Ingredient ingredientName={item} />
    {/each}
</div>