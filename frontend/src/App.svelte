<script lang="ts">
    import Header from "$lib/components/own/layout/Header.svelte";
    import Main from "$lib/components/own/layout/Main.svelte";
    import {currentState} from "$lib/stores";


    console.log($currentState);

</script>


<Header/>
<Main/>





