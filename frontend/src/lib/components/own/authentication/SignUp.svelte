<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import {Button} from "$lib/components/ui/button";
    import * as Form from "$lib/components/ui/form";
    import {Input} from "$lib/components/ui/command";
    import type {SuperForm} from "sveltekit-superforms";
    import type {validatedUser} from "$lib/models/user";
    import {register} from "$lib/controllers/auth";
    import {currentState} from "$lib/stores";


    let formData;
    function clientSideValidation(usernameValue: string, passwordValue: string): boolean
    {
        return (usernameValue.length > 3 && usernameValue.length < 9 && passwordValue.length > 3 && passwordValue.length < 9);
    }


    function handleSubmitSignUP() {
        let username = (document.getElementById('email') as HTMLInputElement).value;
        let password = (document.getElementById('password') as HTMLInputElement).value;

        let isValid = clientSideValidation(username, password);
        if (isValid) {
            console.log("Valid");
        } else {
            console.log("Invalid");
        }
        let validated: validatedUser = {id:0, name: username, password_hash: password};

        let response = register(validated);
        if (response){
            alert("user registered successfully");
            $currentState.isLogged = true;
        } else {
            alert("User not registered");
        }
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