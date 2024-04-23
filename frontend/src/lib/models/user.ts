import { z } from "zod";

// type to send when login or register
export type validatedUser = {
    username: string,
    password: string,
}

export type User = {
    username: string,
}