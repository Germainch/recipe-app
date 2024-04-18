<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import {Button} from "$lib/components/ui/button";
    import * as Form from "$lib/components/ui/form";
    import {Input} from "$lib/components/ui/command";
    import type {SuperForm} from "sveltekit-superforms";


    let formData;

    function handleSubmitSignUP() {
        console.log("Signing up");
        //#todo validate form
    }
</script>

<AlertDialog.Root>
    <AlertDialog.Trigger>
        <Button> Sign Up </Button>
    </AlertDialog.Trigger>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title> Sign Up </AlertDialog.Title>
            <AlertDialog.Description>
                <form method="post" class="flex flex-col p-3 gap-4">
                    <label for="email">Username</label>
                    <input type="email" name="email" id="email" autocomplete="off" class="bg-secondary p-1 text-black dark:text-white">
                    <label for="password">Password</label>
                    <input type="password" name="password" id="password" autocomplete="off" class="bg-secondary p-1 text-black dark:text-white">
                </form>
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel> Cancel </AlertDialog.Cancel>
            <AlertDialog.Action on:click={handleSubmitSignUP}> Sign Up </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>