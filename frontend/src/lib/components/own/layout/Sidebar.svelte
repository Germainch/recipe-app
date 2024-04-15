<script lang="ts">
    import {Button} from "$lib/components/ui/button";
    import { Toggle } from "$lib/components/ui/toggle";
    import * as ToggleGroup from "$lib/components/ui/toggle-group";
    import {Routes} from "$lib/state";
    import {currentState} from "$lib/stores";

    let value: string | undefined;

    switch (value){
        case "myRecipes":
            $currentState.route = Routes.myRecipes;
            break;
        case "search":
            $currentState.route = Routes.search;
            break;
        case "create":
            $currentState.route = Routes.create;
            break;
        case undefined:
            $currentState.route = Routes.home;
            break;
    }

</script>

<aside class="p-3 border-r-2 border-b-2 visible">
    <ToggleGroup.Root bind:value type="single">
        <ToggleGroup.Item value="myRecipes" aria-label="Toggle Browse">
            My recipes
        </ToggleGroup.Item>
        <ToggleGroup.Item value="search" aria-label="Toggle Search">
            Search Recipes
        </ToggleGroup.Item>
        <ToggleGroup.Item value="create" aria-label="Toggle Create">
            Create Recipe
        </ToggleGroup.Item>
    </ToggleGroup.Root>
    value={value}
</aside>

