import type {LayoutServerLoad} from "../../.svelte-kit/types/src/routes/$types";





export const load: LayoutServerLoad = async ( event ) => {
    if(!event.locals.user){
        console.log("No session found")
    }
    console.log(event.locals.user);

    return event.locals.user;
};