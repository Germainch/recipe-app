<script lang="ts">
import SigningForm from "$lib/components/SigningForm.svelte";
import {redirectAfter2Sec} from "$lib/utils/utils";



</script>

<div id="page" class="flex flex-col content-center justify-items-center items-center p-3 w-full h-full justify-center gap-32">

    <!-- If user is not logged in, show the form -->

        <h1 class="text-center text-6xl font-extrabold">Sign In</h1>
        <SigningForm formType="Sign in"></SigningForm>

        redirection ....

</div>

