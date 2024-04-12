
<script>
    export let formType;
</script>



<div id="signup-container" class="card border-2 shadow-2xl border-primary-800">

    <form method="post" class="form-input dark:variant-glass-tertiary bg-secondary-100 flex flex-col gap-2">
        <label for="username">Username</label>
        <input type="text" id="username" name="username" placeholder="Your username.." class="text-black" required>

        <label for="password">Password</label>
        <input type="password" id="password" name="password" placeholder="Your password.." class="text-black" required>

        {#if formType === 'Sign Up'}
            <label for="password">Confirm Password</label>
            <input type="password" id="password" name="password" placeholder="Your password.." class="text-black" required>
        {/if}

        <button class="btn btn-md dark:variant-filled-primary variant-filled-secondary m-2">{ formType }</button>
    </form>

</div>