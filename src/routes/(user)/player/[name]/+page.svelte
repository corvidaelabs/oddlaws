<script lang="ts">
	import ScreenshotGrid from '$lib/components/ScreenshotGrid.svelte';

	let { data } = $props();

	const sortedScreenshots = data.member.screenshots.sort(
		(a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
	);
</script>

<section class="flex flex-col items-start justify-evenly space-y-4 p-4">
	<h1 class="text-8xl font-bold">{data.member.member.name}</h1>
	<div class="grid w-full grid-cols-6">
		<ScreenshotGrid screenshots={sortedScreenshots} class="col-span-2" />
	</div>
</section>
