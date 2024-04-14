import type {PageServerLoad} from "../../../../.svelte-kit/types/src/routes/$types";
import {type Actions, fail, redirect} from "@sveltejs/kit";
import {db} from "$lib/server/drizzle";
import {userTable} from "../../../../drizzle/schemas/schema";
import {generateId} from "lucia";
import {Argon2id} from "oslo/password";
import {lucia} from "$lib/server/auth";



export const load: PageServerLoad = async ( event ) => {
        if(event.locals.user){
           throw redirect(303, "/");
        }
};



// On form submission (POST request) to this route, run the following actions
// the user is signed in if he is in the database
// if the user is already signed in, redirect to the home page

export const actions = {
    default : async ({ cookies, request }) => {

        const formData = await request.formData();


        const username = formData.get("username");
        const password = formData.get("password");

        if (
            typeof username !== "string" ||
            username.length < 3 ||
            username.length > 31 ||
            !/^[a-z0-9_-]+$/.test(username)
        ) {
            return fail(400, {
                message: "Invalid username"
            });
        }
        if (typeof password !== "string" || password.length < 6 || password.length > 255) {
            return fail(400, {
                message: "Invalid password"
            });
        }

        const hashedPassword = await new Argon2id().hash(password);
        const userId = generateId(15);


        await db.insert(userTable).values({
            id: userId,
            username,
            password: hashedPassword,
        });

        const session = await lucia.createSession(userId, {});
        const sessionCookie = lucia.createSessionCookie(session.id);
        cookies.set(sessionCookie.name, sessionCookie.value, {
            path: ".",
           ...sessionCookie.attributes
        });

        return{message: "successfully created the account"}

    },
} satisfies Actions;