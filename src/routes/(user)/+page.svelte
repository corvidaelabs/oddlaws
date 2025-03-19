<script lang="ts">
	let { data } = $props();

	let members = data.publishedMembers ?? [];
	let filterText = $state('');
	let filteredMembers = $derived.by(() =>
		members.filter((member) => member.member.name.toLowerCase().includes(filterText.toLowerCase()))
	);
</script>

<section class="flex min-h-full w-full grow flex-col items-center justify-start space-y-4 p-4">
	<h1 class="h1">Sneaky Crow's Oddlaws Sandbox</h1>
	<p class="text-xl">I use this website to test different data applications for oddlaws</p>

	<h1>Members:</h1>
	<div class="grid w-full grid-cols-4 gap-2">
		<input
			type="text"
			placeholder="Filter members..."
			bind:value={filterText}
			class="input col-span-4"
		/>
		{#each filteredMembers as member (member.member.discordId)}
			<div class="flex flex-col items-center">
				<a
					href={`/player/${member.member.name}`}
					class="chip preset-filled-primary-500 w-full text-base"
				>
					{member.member.name}
				</a>
			</div>
		{/each}
	</div>
</section>
