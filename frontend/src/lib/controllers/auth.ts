




export function login(){}
export function logout(){}
export function register(){}

export function validateUser(){

    //#todo fetch validation from server

    // to remove
    return localStorage.getItem("sessionID") !== null;
}


