<script lang='ts'>

import {ListBox, ListBoxItem, popup} from '@skeletonlabs/skeleton';
import type { PopupSettings } from '@skeletonlabs/skeleton';

let loginCombobox: string;
const popupComboboxLogin: PopupSettings = {
    event: 'click',
    target: 'popupComboboxLogin',
    placement: 'bottom',
    closeQuery: '.listbox-item'
};

</script>

<div class="flex items-center">
    <button class="btn btn-sm mr-4 variant-filled-tertiary border-2" use:popup = {popupComboboxLogin}>
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

<div class="card shadow-xl py-2 variant-filled-tertiary"  data-popup="popupComboboxLogin" >
    <div class="flex flex-col justify-center justify-items-center">
        <button class="btn btn-sm variant-filled-tertiary">
            <a href="/login"> Login </a></button>
        <button class="btn btn-sm variant-filled-tertiary ">Sign up</button>
    </div>
</div>


