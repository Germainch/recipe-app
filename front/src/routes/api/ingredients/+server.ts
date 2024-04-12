import {json, type RequestHandler} from "@sveltejs/kit";


export const GET : RequestHandler = async ({ url}) => {
    // Select an ingredient by name
    return json({message: "Ingredient selected"});
}

export const POST : RequestHandler = async ( event ) => {
    //Create an ingredient in the database
    return json({message: "Ingredient created"});
}

export const PATCH : RequestHandler = async ( event ) => {
    //Update an ingredient in the database
    return json({message: "Ingredient updated"});
}

export const DELETE : RequestHandler = async ( event ) => {
    //Delete an ingredient from the database
    return json({message: "Ingredient deleted"});
}