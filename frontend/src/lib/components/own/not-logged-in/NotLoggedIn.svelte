<script>
    import SignUp from "$lib/components/own/authentication/SignUp.svelte";
    import Login from "$lib/components/own/authentication/Login.svelte";
</script>


<div class="flex flex-col justify-center self-center items-center w-1/2 p-5 m-4 gap-4">
    <h1> You have to log in to access this resource </h1>
    <div class="flex flex-row">
        <Login></Login>
        <SignUp></SignUp>
    </div>
</div>