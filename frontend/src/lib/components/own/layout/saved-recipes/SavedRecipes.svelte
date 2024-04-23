<script lang="ts">

    import {onMount} from "svelte";
    import RecipesLayout from "$lib/components/own/recipe/RecipesLayout.svelte";
    import {currentState} from "$lib/stores";
    import {Button} from "$lib/components/ui/button";
    import Login from "$lib/components/own/authentication/Login.svelte";
    import SignUp from "$lib/components/own/authentication/SignUp.svelte";
    import Logout from "$lib/components/own/authentication/Logout.svelte";

</script>

{#if !$currentState.isLogged}
    <div class="flex flex-col items-center justify-center">
        <h1 class="font-bold text-xl"> You have to be signed in to see your saved recipes </h1>
        <div class="flex flex-row p-5">
            <Login/>
            <SignUp></SignUp>
        </div>
    </div>

{:else}
    <RecipesLayout></RecipesLayout>
{/if}