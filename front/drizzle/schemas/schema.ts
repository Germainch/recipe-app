import {integer, pgTable, serial, uuid, varchar} from 'drizzle-orm/pg-core';
import {text, timestamp} from "drizzle-orm/pg-core";



export const userTable = pgTable("user", {
    id: varchar("id").primaryKey(),
    username : varchar("username").notNull(),
    password : varchar("password").notNull(),
});

export const sessionTable = pgTable("session", {
    id: varchar("id").primaryKey(),
    userId: text("user_id")
        .notNull()
        .references(() => userTable.id),
    expiresAt: timestamp("expires_at").notNull(),
});

export const ingredientTable = pgTable("ingredient", {
    id: integer("id").primaryKey(),
    name: varchar("name").notNull(),
});

export const recipeTable = pgTable("recipe", {
    id: integer("id").primaryKey(),
    name: text("name").notNull(),
    description: text("description").notNull(),
    createdBy: text("created_by").references(() => userTable.id),
});




