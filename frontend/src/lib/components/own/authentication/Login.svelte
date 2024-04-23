<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog";
    import {Button} from "$lib/components/ui/button";
    import * as Form from "$lib/components/ui/form";
    import {Input} from "$lib/components/ui/command";
    import type {SuperForm} from "sveltekit-superforms";
    import {currentState} from "$lib/stores";
    import {type User, type validatedUser} from "$lib/models/user";
    import {FormFieldErrors} from "$lib/components/ui/form";
    import {login} from "$lib/controllers/auth";


    let usernameValue: string = "";
    let passwordValue: string = "";
    let usernameError: boolean = false;
    let passwordError: boolean = false;
    let usernameErrorMessage: string[] = [];
    let passwordErrorMessage: string[] = [];
    let clientSideValidated = false;
    
    function clientSideValidation(): boolean
    {
        return (usernameValue.length > 3 && usernameValue.length < 9 && passwordValue.length > 3 && passwordValue.length < 9);
    }

    function handleInputChange(){
        if (clientSideValidation())
            document.getElementById("signin-button")?.removeAttribute("disabled");
    }

    function handleSubmitSignIn(username: string, password: string) {
        // validation server-side by comparing with database
        let data = {username, password};
        let result = login(data);

        $currentState.isLogged = true;
    }
</script>

<AlertDialog.Root>
    <AlertDialog.Trigger>
        <Button class="bg-transparent hover:opacity-100 hover:bg-secondary text-black dark:text-white"> Log in </Button>
    </AlertDialog.Trigger>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title> Sign in </AlertDialog.Title>
            <AlertDialog.Description>

                <form class="flex flex-col p-3 gap-4" autocomplete="off" on:input={handleInputChange} >


                    <label for="username">Username</label>

                    <input type="text"
                           name="username"
                           id="username"
                           autocomplete="off"
                           minlength="3"
                           maxlength="9"
                           required
                           class="bg-secondary p-1 text-black dark:text-white valid:bg-green-200"
                           bind:value={usernameValue}
                    >

                    <label for="password">Password</label>
                    <input type="password"
                           name="password"
                           id="password"
                           minlength="3"
                           maxlength="9"
                           autocomplete="off"
                           required
                           class="bg-secondary p-1 text-black dark:text-white valid:bg-green-200"
                           bind:value={passwordValue}
                    >
                </form>

            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel> Cancel </AlertDialog.Cancel>
            <AlertDialog.Action id="signin-button" disabled={!clientSideValidated} on:click={handleSubmitSignIn}> Sign in </AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>


