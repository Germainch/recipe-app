<script lang="ts">
import {LightSwitch, popup} from "@skeletonlabs/skeleton";
import type {PopupSettings} from "@skeletonlabs/skeleton";



const popupComboboxLogin: PopupSettings = {
    event: 'click',
    target: 'popupComboboxLogin',
    placement: 'bottom-end',
    closeQuery: '.listbox-item'
};

</script>

<div class="flex items-center">
    <button class="btn btn-sm m-4 p-1 dark:variant-glass-tertiary variant-glass-secondary" use:popup = {popupComboboxLogin}>
        <span>
                <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="lucide lucide-user"
                >
                    <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/>
                    <circle cx="12" cy="7" r="4"/>
                </svg>
        </span>
    </button>
</div>


    <div class="card shadow-xl py-2 dark:variant-filled-tertiary variant-filled-secondary"  data-popup="popupComboboxLogin" >
        <div class="flex flex-col justify-center justify-items-center items-center">
            <button class="btn btn-sm">
                <a href="/login"> Login </a>
            </button>
            <button class="btn btn-sm">
                <a href="/signup"> Sign up </a>
            </button>
            <LightSwitch></LightSwitch>
        </div>
    </div>


    <div class="card shadow-xl py-2 dark:variant-filled-tertiary variant-filled-secondary"  data-popup="popupComboboxLogin" >
        <div class="flex flex-col justify-center justify-items-center items-center">
            <button class="btn btn-sm">
                <a href="/logout"> Logout </a>
            </button>
            <LightSwitch></LightSwitch>
        </div>
    </div>


    <div class="card shadow-xl py-2 dark:variant-filled-tertiary variant-filled-secondary"  data-popup="popupComboboxLogin" >
        <div class="flex flex-col justify-center justify-items-center items-center">
            <button class="btn btn-sm">
                <a href="/logout"> Logout </a>
            </button>
            <LightSwitch></LightSwitch>
        </div>
    </div>
