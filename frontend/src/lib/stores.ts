import {writable, type Writable} from "svelte/store";
import {type AppState, Routes} from "$lib/state";

export const currentState: Writable<AppState> = writable({
    route: Routes.home,
    isLogged: false,
});