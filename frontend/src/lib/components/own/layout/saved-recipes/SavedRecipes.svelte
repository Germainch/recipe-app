<script lang="ts">

    import {onMount} from "svelte";
    import {validateUser} from "$lib/controllers/auth";
    import RecipesLayout from "$lib/components/own/recipe/RecipesLayout.svelte";
    import {currentState} from "$lib/stores";
    import {Button} from "$lib/components/ui/button";
    import Login from "$lib/components/own/authentication/Login.svelte";
    import SignUp from "$lib/components/own/authentication/SignUp.svelte";
    import Logout from "$lib/components/own/authentication/Logout.svelte";

    onMount(() => {
        validateUser();
        console.log('Hello World')
    });

</script>

{#if !$currentState.isLogged}
    <h1> You have to be signed in to see your saved recipes </h1>
    <Login/>
    <SignUp></SignUp>
    <Logout></Logout>
{:else}
    <RecipesLayout></RecipesLayout>
{/if}