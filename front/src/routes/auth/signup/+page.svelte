<script lang="ts">
import SigningForm from "$lib/components/SigningForm.svelte";

export let form;
console.log(form?.message)
</script>


<div id="page" class="flex flex-col content-center justify-items-center items-center p-3 w-full h-full justify-center gap-32">
        <h1 class="text-center text-6xl font-extrabold">Sign Up</h1>
        <SigningForm formType="Sign up"></SigningForm>
        <h2>{form?.message}</h2>
</div>