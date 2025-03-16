<script lang="ts">
	let { data } = $props();

	let members = data.publishedMembers ?? [];
</script>

<section class="flex min-h-full w-full grow flex-col items-center justify-center space-y-4">
	<h1 class="h1">Sneaky Crow's Oddlaws Sandbox</h1>
	<p class="text-xl">I use this website to test different data applications for oddlaws</p>

	<h1>Members:</h1>
	<div class="flex gap-2">
		{#each members as member (member.member.discordId)}
			<div class="flex flex-col items-center">
				<a href={`/player/${member.member.name}`} class="chip preset-filled-primary-500">
					{member.member.name}
				</a>
			</div>
		{/each}
	</div>
</section>
