<script lang="ts">
	import { enhance } from '$app/forms';
	import { AppBar } from '@skeletonlabs/skeleton-svelte';
	import { LogOut, LogIn } from 'lucide-svelte';

	let { user } = $props();
</script>

<AppBar classes="justify-center leading-[2]">
	{#snippet lead()}
		<a href="/" class="text-left font-semibold uppercase">Oddlaws Experiments</a>
		<a href="/skeever" class="text-primary-500 hover:text-primary-300 uppercase">Skeever</a>
	{/snippet}
	{#snippet trail()}
		{#if user}
			<div class="user">
				<span>{user.name}</span>
				<form method="post" action="?/logout" use:enhance>
					<button class="btn preset-filled-primary-500">
						Sign out <LogOut size={16} />
					</button>
				</form>
			</div>
		{:else}
			<a href="/login" type="button" class="btn preset-filled-primary-500">
				Login <LogIn size={16} />
			</a>
		{/if}
	{/snippet}
</AppBar>
