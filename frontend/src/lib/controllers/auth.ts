import {currentState} from "$lib/stores";
import {backendURL} from "$lib/stores";
import type {validatedUser} from "$lib/models/user";

export function login(user: validatedUser): boolean{
    let url = backendURL;
    let endpoint = "/login";

    fetch(backendURL + endpoint, {
        method: "POST",
        headers:{
            "Content-Type": "application/json"
        },
        body:
            JSON.stringify(user)
    })
        .then(async r => {
            switch (r.status) {
                case 201:
                    currentState.update(state => {
                        return {...state, isLogged: true}
                    });
                    let sessionID = await r.text();
                    sessionStorage.setItem("sessionID", sessionID);
                    return true;
                case 401:
                    console.log("Invalid credentials");
                    return false;
                case 500:
                    console.log("Internal server error");
                    return false;
            }
        })
    return false;
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
    let endpoint = "/signup";
    let responseData: string;
    fetch(backendURL + endpoint, {
        method: "POST",
        headers:{
            "Content-Type": "application/json"
        },
        body:
            JSON.stringify(user)
    })
        .then(async r => {
            switch (r.status) {
                case 201:
                    console.log("User created successfully");
                    responseData = await r.text();
                    console.log(responseData);
                    return true;
                case 400:
                    console.log("Bad request");
                    return false;
                case 500:
                    console.log("Internal server error");
                    return false;
            }
        })
        .catch(e => {
            console.log(e);
            return false
        });

    // if successful, set the cookie sent by backend and set isLogged to true
    // else display an error message

    return true;

}




