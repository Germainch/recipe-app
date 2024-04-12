import type { Actions } from './$types';
import {json} from "@sveltejs/kit";




export const actions = {
    default: async ({cookies, request}) => {
        const data = await request.formData();
        const ingredients = data.getAll('ingredient');
        console.log(ingredients);
        const ingredientsJSON = json(ingredients);
        console.log(ingredientsJSON);
    },
} satisfies Actions;