<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import '../../app.css';
	let { children, data } = $props();
</script>

<main class="h-[calc(100vh-64px)]">
	<Header user={data.user} />
	{@render children()}
</main>
