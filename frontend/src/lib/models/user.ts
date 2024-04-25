import { z } from "zod";

// type to send when login or register
export type validatedUser = {
    id: number,
    name: string,
    password_hash: string,
}

export type User = {
    username: string,
}