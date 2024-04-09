<script>

</script>
<div id="page" class="flex flex-col content-center justify-items-center items-center p-3 w-full h-full justify-center ">

    <div id="login-container" class="card shadow-2xl border-2 dark:variant-glass-primary variant-glass-secondary dark:border-amber-100 border-primary-800">
            <div style="text-align: center;"> Login</div>
            <form action="/api/login" method="post" class="form-input variant-glass-primary ">
                <label for="username">Username</label>
                <input type="text" id="username" name="username" placeholder="Your username.." class="dark:variant-ghost-tertiary variant-ghost-secondary" required>

                <label for="password">Password</label>
                <input type="password" id="password" name="password" placeholder="Your password.." class="dark:variant-ghost-tertiary variant-ghost-secondary" required>

                <button type="submit" class="btn-md dark:variant-filled-tertiary variant-filled-secondary">Sign In</button>
            </form>
    </div>
</div>

