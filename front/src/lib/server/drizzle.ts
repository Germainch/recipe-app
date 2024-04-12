import { DrizzlePostgreSQLAdapter } from '@lucia-auth/adapter-drizzle';
import postgres from "postgres";
import { pgTable, text, timestamp } from "drizzle-orm/pg-core";
import { drizzle } from "drizzle-orm/postgres-js";
import {sessionTable, userTable} from "../../../drizzle/schemas/schema";

/**
 * Initialize the database Query client connection
 */
const queryClient = postgres("postgres://postgres:adminadmin@0.0.0.0:5432/db");

/**
 * Initialize the Drizzle ORM database connection
 */
const db = drizzle(queryClient);


/**
 * Define the tables for the database
 */


export const adapter = new DrizzlePostgreSQLAdapter(db, sessionTable, userTable);