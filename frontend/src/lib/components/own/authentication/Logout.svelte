<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import {Button} from "$lib/components/ui/button";
    import * as Form from "$lib/components/ui/form";
    import {Input} from "$lib/components/ui/command";
    import type {SuperForm} from "sveltekit-superforms";
    import {currentState} from "$lib/stores";


    let formData;

    function handleLogout() {
        console.log("Logging out...");
        //#todo validate form
        $currentState.isLogged = false;
    }
</script>

<AlertDialog.Root>
    <AlertDialog.Trigger>
        <Button> Logout </Button>
    </AlertDialog.Trigger>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title> Are you sure ? </AlertDialog.Title>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel> Cancel </AlertDialog.Cancel>
            <AlertDialog.Action class="bg-destructive opacity-100 text-destructive-foreground hover:opacity-80 hover:bg-destructive hover:text-destructive-foreground" on:click={handleLogout}> Log out </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>