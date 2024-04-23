import {currentState} from "$lib/stores";
import {backendURL} from "$lib/stores";
import type {validatedUser} from "$lib/models/user";

export function login(user: validatedUser){
    let url = backendURL;
    let endpoint = "/login";

    fetch(backendURL + endpoint, {
        method: "POST",
        body:
            JSON.stringify(user)
    })
        .then(r => console.log(r))
    // if successful, set the cookie sent by backend and set isLogged to true
    // else display an error message
    return true;
}
export function logout(){
    let url = backendURL + "/logout";
    let sessionID = localStorage.getItem("sessionID") ?? "";

    if( localStorage.getItem("sessionID") !== null){
        fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "sessionID": sessionID,
            }
        }).then(r => console.log(r));
    }
    // send a request to the backend to logout

    currentState.update(state => {
        return {...state, isLogged: false}
    });
}
export function register(user: validatedUser){
    // send a request to the backend to register
    let url = backendURL;
    let endpoint = "/login";
    let responseData = {};
    fetch(backendURL + endpoint, {
        method: "POST",
        body:
            JSON.stringify(user)
    })
        .then(r => {console.log(r); responseData = JSON.stringify(r)})
    // if successful, set the cookie sent by backend and set isLogged to true
    // else display an error message
    return true;

}




