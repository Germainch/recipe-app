import { DrizzlePostgreSQLAdapter } from '@lucia-auth/adapter-drizzle';
import postgres from "postgres";
import { drizzle } from "drizzle-orm/postgres-js";
import {sessionTable, userTable} from "../../../drizzle/schemas/schema";
import * as dotenv from "dotenv";
dotenv.config({path: "../.env"})



/**
 * Initialize the database Query client connection
 */

const connectionString = process.env.DATABASE_URL ?? ""
export const queryClient = postgres(connectionString);

console.log(connectionString)

/**
 * Initialize the Drizzle ORM database connection
 */
export const db = drizzle(queryClient);


/**
 * Define the tables for the database
 */


export const adapter = new DrizzlePostgreSQLAdapter(db, sessionTable, userTable);