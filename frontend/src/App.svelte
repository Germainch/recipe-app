<script lang="ts">
    import Header from "$lib/components/own/layout/Header.svelte";
    import Main from "$lib/components/own/layout/Main.svelte";
    import {currentState} from "$lib/stores";

    currentState.subscribe((value) => {
        console.log(value);
    });
    function loop(){
        setTimeout(() => {
            console.log($currentState);
            loop();
        }, 1000000);
    }
    loop();
</script>


<Header/>
<Main/>





