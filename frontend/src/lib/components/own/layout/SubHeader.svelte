<script lang="ts">
    import {Button} from "$lib/components/ui/button";
    import { Toggle } from "$lib/components/ui/toggle";
    import * as ToggleGroup from "$lib/components/ui/toggle-group";
    import {type AppState, Routes} from "$lib/state";
    import {currentState} from "$lib/stores";


    let value: string | undefined;
    let appState: AppState = $currentState;

    $: switch (value){
        case "myRecipes":
            currentState.set({route: Routes.myRecipes, isLogged: $currentState.isLogged});
            break;
        case "search":
            currentState.set({route: Routes.search, isLogged: $currentState.isLogged});
            break;
        case "create":
            currentState.set({route: Routes.create, isLogged: $currentState.isLogged});
            break;
        case undefined:
            currentState.set({route: Routes.home, isLogged: $currentState.isLogged});
            break;
    }


</script>

<div class="p-3 visible border-b-2 border-b-destructive-foreground">
    <ToggleGroup.Root bind:value type="single">
        <ToggleGroup.Item value="myRecipes" aria-label="Toggle Browse" >
            My recipes
        </ToggleGroup.Item>
        <ToggleGroup.Item value="search" aria-label="Toggle Search">
            Search Recipes
        </ToggleGroup.Item>
        <ToggleGroup.Item value="create" aria-label="Toggle Create">
            Create Recipe
        </ToggleGroup.Item>
    </ToggleGroup.Root>
</div>

