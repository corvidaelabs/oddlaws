<script lang="ts">
	let { data } = $props();

	let members = data.publishedMembers ?? [];
	console.log(members);
</script>

<section class="flex min-h-full w-full grow flex-col items-center justify-center space-y-4">
	<h1 class="h1">Sneaky Crow's Oddlaws Sandbox</h1>
	<p class="text-xl">I use this website to test different data applications for oddlaws</p>

	<h1>Members:</h1>
	<div class="flex gap-2">
		{#each members as member (member.member.discordId)}
			<div class="flex flex-col items-center">
				<h1>Screenshots by {member.member.name}</h1>
				{#each member.screenshots as screenshot (screenshot.id)}
					<img src={screenshot.url} alt={screenshot.id} class="max-w-3xs" />
				{/each}
			</div>
		{/each}
	</div>
</section>
