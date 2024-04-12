<script lang="ts">
	import '../app.postcss';
    import {AppBar, AppShell} from "@skeletonlabs/skeleton";
    import Hamburger from "$lib/components/Hamburger.svelte";
    import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
    import { storePopup } from '@skeletonlabs/skeleton';
    import UserMenu from "$lib/components/UserMenu.svelte";


    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
</script>



<AppShell class="h-lvh w-lvw">
    <svelte:fragment slot="header">
        <AppBar slotDefault="place-self-center" slotTrail="place-content-end" shadow="shadow-lg" >
            <svelte:fragment slot="lead">
                <Hamburger></Hamburger>

            </svelte:fragment>
            <h1 class="text-4xl font-extrabold"> RECIPES </h1>
            <svelte:fragment slot="trail">
                <UserMenu></UserMenu>

            </svelte:fragment>
        </AppBar>
    </svelte:fragment>


    <!-- PAGE Contents -->
    <slot/>


</AppShell>

