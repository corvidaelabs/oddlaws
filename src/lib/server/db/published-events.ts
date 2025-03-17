import type { PublishedEvent } from './schema';
import * as table from '$lib/server/db/schema';
import { db } from '$lib/server/db';

export const getAllScheduledEvents = async (): Promise<PublishedEvent[]> => {
	const results = await db.select().from(table.publishedEvents);

	if (!results.length) {
		return [];
	}

	return results;
};
