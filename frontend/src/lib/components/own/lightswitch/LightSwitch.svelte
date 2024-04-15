<script>
import {Switch} from "$lib/components/ui/switch";

let darkmode = false;
function handleClick() {
    darkmode = !darkmode;
    darkmode
        ? document.documentElement.classList.add('dark')
        : document.documentElement.classList.remove('dark');
}

</script>

<Switch on:click={handleClick}/>