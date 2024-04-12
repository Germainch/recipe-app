import type {PageServerLoad} from "../../../../.svelte-kit/types/src/routes/$types";
import {type Actions} from "@sveltejs/kit";



export const load: PageServerLoad = async ({ cookies }) => {
        // #TODO
};



// On form submission (POST request) to this route, run the following actions
// the user is signed in if he is in the database
// if the user is already signed in, redirect to the home page

export const actions = {
    default : async ({ cookies, request }) => {
        //#todo
    },
} satisfies Actions;