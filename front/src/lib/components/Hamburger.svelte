
<script lang="ts">
    import {ListBox, ListBoxItem, popup} from '@skeletonlabs/skeleton';
    import type { PopupSettings } from '@skeletonlabs/skeleton';
    let comboboxValue: string;

    const popupCombobox: PopupSettings = {
        event: 'click',
        target: 'popupCombobox',
        placement: 'bottom',
        closeQuery: '.listbox-item'
    };

</script>



<div class="flex items-center">
    <button class="btn btn-sm mr-4 hover:dark:bg-primary-700 hover:bg-primary-600" use:popup = {popupCombobox}>
        <span>
            <svg viewBox="0 0 100 80" class="fill-token w-4 h-4">
                <rect width="100" height="20" />
                <rect y="30" width="100" height="20" />
                <rect y="60" width="100" height="20" />
            </svg>
        </span>
    </button>
</div>


<div class="card  shadow-xl py-2" data-popup="popupCombobox">
    <div class="flex flex-col p-3 gap-4">
        <button class="btn btn-md variant-glass-primary">
            <a href="/recipes">My Recipes</a>
        </button>
        <button class="btn variant-glass-primary">
            <a href="/search">Find Recipes</a>
        </button>
        <button class="btn variant-glass-primary">
            <a href="/create">Create Recipe</a>
        </button>
    </div>
</div>
