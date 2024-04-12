import type {PageServerLoad} from "../../../../.svelte-kit/types/src/routes/$types";
import {type Actions } from "@sveltejs/kit";
import {randomUUID} from "node:crypto";


export const load: PageServerLoad = async ({ cookies }) => {

    const userSession = cookies.get('sessionId')
    return { userSession };
};



// On form submission (POST request) to this route, run the following actions
// the user is signed in if he is in the database
// if the user is already signed in, redirect to the home page
export const actions = {
    default : async ({ cookies, request }) => {

        const data = await request.formData();
        const username = data.get('username');
        const password = data.get('password');
        const sessionid= randomUUID();

        return {success: true};
    },
} satisfies Actions;