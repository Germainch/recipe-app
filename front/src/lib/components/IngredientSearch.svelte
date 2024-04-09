<script lang="ts">

    import {InputChip, popup, type PopupSettings} from "@skeletonlabs/skeleton";
    let input = '';
    let chipList: string[] = [];
    let flavorsWhitelist = ['vanilla', 'chocolate', 'strawberry', 'peach', 'rocky road'];
    function onInvalidHandler(event: any){
       console.log( event.detail.label);
    }

    const popupCombobox: PopupSettings = {
        event: 'click',
        target: 'popupCombobox',
        placement: 'bottom',
        closeQuery: '.listbox-item'
    };
</script>

<div class="card p-4 flex flex-col align-items-center justify-items-center items-center
gap-2" >
    <h2> Select your ingredients</h2>
    <InputChip bind:value={chipList} name="chips" placeholder="Enter an Ingredient..." whitelist={flavorsWhitelist} on:invalid={onInvalidHandler} />
    <button use:popup={popupCombobox}>autocomplete</button>
</div>

<div class="card  shadow-xl py-2" data-popup="popupCombobox">
    <div class="flex flex-col p-3 gap-4">
        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Voluptatem nam repudiandae natus veritatis magnam sed amet est perspiciatis. Quasi ullam nisi unde vel quia eveniet enim excepturi laudantium est asperiores.
    </div>
</div>