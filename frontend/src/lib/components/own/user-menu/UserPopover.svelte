<script lang="ts">
    import * as Popover from "$lib/components/ui/popover";
    import UserMenu from "$lib/components/own/user-menu/UserMenu.svelte";
    import {currentState} from "$lib/stores";
    import SignUp from "$lib/components/own/authentication/SignUp.svelte";
    import Login from "$lib/components/own/authentication/Login.svelte";
    import Logout from "$lib/components/own/authentication/Logout.svelte";
    import LightSwitch from "$lib/components/own/lightswitch/LightSwitch.svelte";
</script>
<div class="flex flex-row gap-2 justify-evenly items-center">
<LightSwitch></LightSwitch>
<Popover.Root>
    <Popover.Trigger>
        <UserMenu></UserMenu>
    </Popover.Trigger>
    <Popover.Content class="z-50 w-fit rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none">
        {#if $currentState.isLogged === false}
            <div class="flex flex-col gap-2">
                <Login></Login>
                <SignUp></SignUp>
            </div>
        {:else}
            <div>
                Hi
                <Logout></Logout>
            </div>
        {/if}
    </Popover.Content>
</Popover.Root>
</div>