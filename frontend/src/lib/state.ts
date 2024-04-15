export enum Routes {
    home,
    search,
    create,
    myRecipes,
}


export type AppState = {
    route: Routes,
    isLogged: boolean,
}

