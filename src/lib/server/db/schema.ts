import { sql } from 'drizzle-orm';
import { pgTable, text, integer, timestamp, uuid } from 'drizzle-orm/pg-core';

export const user = pgTable('user', {
	id: text('id').primaryKey(),
	age: integer('age'),
	username: text('username').notNull().unique(),
	passwordHash: text('password_hash').notNull()
});

export const session = pgTable('session', {
	id: text('id').primaryKey(),
	userId: text('user_id')
		.notNull()
		.references(() => user.id),
	expiresAt: timestamp('expires_at', { withTimezone: true, mode: 'date' }).notNull()
});

export const publishedMembers = pgTable('published_members', {
	discordId: text('discord_id').primaryKey(),
	name: text('name').notNull(),
	createdAt: timestamp('created_at', { withTimezone: true, mode: 'date' }).notNull(),
	updatedAt: timestamp('updated_at', { withTimezone: true, mode: 'date' }).notNull()
});

export const memberScreenshots = pgTable('member_screenshots', {
	id: uuid('id')
		.default(sql`gen_random_uuid()`)
		.primaryKey(),
	url: text('url').notNull(),
	memberId: text('member_id')
		.notNull()
		.references(() => publishedMembers.discordId),
	createdAt: timestamp('created_at', { withTimezone: true, mode: 'date' }).notNull(),
	updatedAt: timestamp('updated_at', { withTimezone: true, mode: 'date' }).notNull()
});

export type Session = typeof session.$inferSelect;

export type User = typeof user.$inferSelect;

export type PublishedMember = typeof publishedMembers.$inferSelect;

export type MemberScreenshot = typeof memberScreenshots.$inferSelect;
