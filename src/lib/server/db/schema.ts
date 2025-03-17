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
	id: uuid('id')
		.default(sql`gen_random_uuid()`)
		.primaryKey(),
	discordId: text('discord_id').notNull().unique(),
	name: text('name').notNull(),
	createdAt: timestamp('created_at', { withTimezone: true, mode: 'date' }).notNull(),
	updatedAt: timestamp('updated_at', { withTimezone: true, mode: 'date' }).notNull()
});

export const memberScreenshots = pgTable('member_screenshots', {
	id: uuid('id')
		.default(sql`gen_random_uuid()`)
		.primaryKey(),
	url: text('url').notNull(),
	memberId: uuid('member_id')
		.notNull()
		.references(() => publishedMembers.id),
	createdAt: timestamp('created_at', { withTimezone: true, mode: 'date' }).notNull(),
	updatedAt: timestamp('updated_at', { withTimezone: true, mode: 'date' }).notNull()
});

export const publishedEvents = pgTable('published_events', {
	id: uuid('id')
		.default(sql`gen_random_uuid()`)
		.primaryKey(),
	title: text('title').notNull(),
	description: text('description'),
	discordId: text('discord_id').notNull().unique(),
	startTime: timestamp('start_time', { withTimezone: true, mode: 'date' }).notNull(),
	endTime: timestamp('end_time', { withTimezone: true, mode: 'date' }),
	createdAt: timestamp('created_at', { withTimezone: true, mode: 'date' }).notNull(),
	updatedAt: timestamp('updated_at', { withTimezone: true, mode: 'date' }).notNull()
});

export const memberEvents = pgTable('member_events', {
	id: uuid('id')
		.default(sql`gen_random_uuid()`)
		.primaryKey(),
	member_id: uuid('member_id')
		.notNull()
		.references(() => publishedMembers.id),
	event_id: uuid('event_id')
		.notNull()
		.references(() => publishedEvents.id)
});

export type Session = typeof session.$inferSelect;

export type User = typeof user.$inferSelect;

export type PublishedMember = typeof publishedMembers.$inferSelect;

export type MemberScreenshot = typeof memberScreenshots.$inferSelect;

export type PublishedEvent = typeof publishedEvents.$inferSelect;
