<script lang="ts">
	import { enhance } from '$app/forms';
	import { AppBar } from '@skeletonlabs/skeleton-svelte';
	let { user } = $props();
</script>

<AppBar>
	{#snippet trail()}
		{#if user}
			<div class="user">
				<span>{user.name}</span>
				<form method="post" action="?/logout" use:enhance>
					<button class="btn preset-filled-primary-500">Sign out</button>
				</form>
			</div>
		{:else}
			<a href="/login" type="button" class="btn preset-filled-primary-500">Login</a>
		{/if}
	{/snippet}
	<span>Title</span>
</AppBar>
