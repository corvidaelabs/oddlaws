import * as table from '$lib/server/db/schema';
import { db } from '$lib/server/db';
import { eq } from 'drizzle-orm';
import type { PublishedMember, MemberScreenshot } from '$lib/server/db/schema';

// A function for getting a published member by their discord id
export const getPublishedMemberByDiscordId = async (
	discordId: string
): Promise<PublishedMember | null> => {
	const results = await db
		.select()
		.from(table.publishedMembers)
		.where(eq(table.publishedMembers.discordId, discordId));

	if (!results.length) {
		return null;
	}

	return results[0];
};

// A function for getting a published member by their name
export const getPublishedMemberByName = async (
	name: string
): Promise<{ member: PublishedMember; screenshots: MemberScreenshot[] } | null> => {
	const results = await db
		.select()
		.from(table.publishedMembers)
		.leftJoin(
			table.memberScreenshots,
			eq(table.publishedMembers.id, table.memberScreenshots.memberId)
		)
		.where(eq(table.publishedMembers.name, name));

	if (!results.length) {
		return null;
	}

	const member = results[0].published_members;
	const screenshots = results
		.map((row) => row.member_screenshots)
		.filter((s): s is MemberScreenshot => s !== null);

	return {
		member,
		screenshots
	};
};

// A function for getting all published members
export const getPublishedMembers = async () => {
	const results = await db
		.select()
		.from(table.publishedMembers)
		.leftJoin(
			table.memberScreenshots,
			eq(table.publishedMembers.id, table.memberScreenshots.memberId)
		);

	const grouped = results.reduce<
		Record<string, { member: PublishedMember; screenshots: MemberScreenshot[] }>
	>((acc, row) => {
		const member = row.published_members;
		const screenshot = row.member_screenshots;

		if (!acc[member.id]) {
			acc[member.id] = {
				member,
				screenshots: []
			};
		}

		if (screenshot) {
			acc[member.id].screenshots.push(screenshot);
		}

		return acc;
	}, {});

	return Object.values(grouped);
};
