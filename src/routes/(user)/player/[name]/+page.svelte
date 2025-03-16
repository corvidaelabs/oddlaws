<script lang="ts">
	let { data } = $props();

	const sortedScreenshots = data.member.screenshots.sort(
		(a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
	);
</script>

<section class="grid min-h-full w-full grow grid-rows-2 items-center justify-center space-y-4">
	<h1 class="text-8xl font-bold">{data.member.member.name}</h1>
	<h2 class="text-xl font-bold">Screenshots</h2>
	{#each sortedScreenshots as screenshot (screenshot.id)}
		<img src={screenshot.url} alt={screenshot.id} class="max-w-md" />
	{/each}
</section>
