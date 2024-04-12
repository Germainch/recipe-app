import type {LayoutServerLoad} from "../../../../.svelte-kit/types/src/routes/$types";

export const load: LayoutServerLoad = async ({ cookies }) => {

    const userState = await parent();
};