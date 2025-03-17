import * as auth from '$lib/server/auth';
import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { getPublishedMembers } from '$lib/server/db/member';
import { getAllScheduledEvents } from '$lib/server/db/published-events';

export const load: PageServerLoad = async () => {
	const publishedMembers = await getPublishedMembers();
	const scheduledEvents = await getAllScheduledEvents();
	return {
		publishedMembers,
		scheduledEvents
	};
};

export const actions: Actions = {
	logout: async (event) => {
		if (!event.locals.session) {
			return fail(401);
		}
		await auth.invalidateSession(event.locals.session.id);
		auth.deleteSessionTokenCookie(event);

		return redirect(302, '/');
	}
};
