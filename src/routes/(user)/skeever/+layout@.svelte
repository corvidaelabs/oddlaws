<script lang="ts">
	import '../../../app.css';
	let { children } = $props();
</script>

<main class="bg-primary-500 h-screen">
	{@render children()}
</main>
