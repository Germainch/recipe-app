<script lang="ts">



import Hambuger from "$lib/components/own/hamburger/Hambuger.svelte";
import UserMenu from "$lib/components/own/user-menu/UserMenu.svelte";
import UserPopover from "$lib/components/own/user-menu/UserPopover.svelte";
</script>


<header id="header" class="border-b-2 background">
        <Hambuger></Hambuger>
        <h1 class="font-bold"> TITLE </h1>
        <UserPopover/>
</header>

<style>
    #header{
        display: grid;
        grid-template-rows: 1fr;
        grid-template-columns: 1fr 3fr 1fr;
        grid-template-areas: "hamburger  title  end";
        padding: 1rem;
        align-content: center;
    }
    #header h1{
        place-self: center;
    }
</style>