import type {PageServerLoad} from "../../../.svelte-kit/types/src/routes/$types";
import {lucia} from "$lib/server/auth";
import {redirect} from "@sveltejs/kit";

export const load: PageServerLoad = async ( event ) => {

    // the user session is validated in the hooks.server.ts file on each request
    if(event.locals.user == null){
        console.log("User not logged in")
        throw redirect(303, "/login");
    }

    console.log(event.locals.user);

}