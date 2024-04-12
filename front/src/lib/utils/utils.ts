import {goto} from "$app/navigation";

export function redirectAfter2Sec() {
    setTimeout(() => {
        goto('/');
    }, 2000);
}