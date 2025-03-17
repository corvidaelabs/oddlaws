<script lang="ts">
	interface Props {
		screenshots: { url: string; id: string }[];
		class?: string;
	}

	let { screenshots, class: className }: Props = $props();
</script>

<div class={`grid grid-cols-2 gap-4 ${className}`}>
	<h2 class="col-span-2 text-3xl font-bold">Screenshots</h2>
	{#each screenshots as screenshot (screenshot.id)}{/each}
	{#each screenshots as screenshot (screenshot.id)}
		<img src={screenshot.url} alt="Screenshot - {screenshot.id}" />
	{/each}
</div>
