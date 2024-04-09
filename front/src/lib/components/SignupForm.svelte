<div id="signup-container" class="card border-2 shadow-2xl border-primary-800">

    <form action="/api/login" method="post" class="form-input dark:variant-glass-tertiary bg-secondary-100 flex flex-col gap-2">
        <label for="username">Username</label>
        <input type="text" id="username" name="username" placeholder="Your username.." class="text-black" required>

        <label for="password">Password</label>
        <input type="password" id="password" name="password" placeholder="Your password.." class="text-black" required>

        <button type="submit" class="btn btn-md dark:variant-filled-primary variant-filled-secondary m-2">Sign up</button>
    </form>

</div>