<script>
    import SignUp from "$lib/components/own/authentication/SignUp.svelte";
    import Login from "$lib/components/own/authentication/Login.svelte";
</script>


<div>
    <h1> You have to log in to access this resource </h1>
    <div class="flex flex row">
        <Login></Login>
        <SignUp></SignUp>
    </div>
</div>