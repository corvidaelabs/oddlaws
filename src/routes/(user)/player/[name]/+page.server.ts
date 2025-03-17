import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getPublishedMemberByName } from '$lib/server/db/member';

export const load: PageServerLoad = async ({ params }) => {
	const memberName = params.name;

	const member = await getPublishedMemberByName(memberName);
	if (!member) {
		throw redirect(302, '/404');
	}
	return {
		member
	};
};
