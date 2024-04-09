<script lang="ts">

    import {InputChip} from "@skeletonlabs/skeleton";
    let input = '';
    let chipList: string[] = [];
    let flavorsWhitelist = ['vanilla', 'chocolate', 'strawberry', 'peach', 'rocky road'];
    function onInvalidHandler(event: any){
       console.log( event.detail.label);
    }


</script>

<div class="card p-4 flex flex-col align-items-center justify-items-center items-center gap-2">
    <h2> Select your ingredients</h2>
    <InputChip bind:value={chipList} name="chips" placeholder="Enter an Ingredient..." whitelist={flavorsWhitelist} on:invalid={onInvalidHandler}/>
    <form>
        <button type="button" class="btn variant-filled">
            Search
        </button>
    </form>
</div>