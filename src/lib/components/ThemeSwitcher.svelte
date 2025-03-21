<script lang="ts">
	import {
		FloatingArrow,
		arrow,
		autoUpdate,
		flip,
		offset,
		useClick,
		useDismiss,
		useFloating,
		useInteractions,
		useRole
	} from '@skeletonlabs/floating-ui-svelte';
	import { fade } from 'svelte/transition';
	let themes = [{ name: 'Skeever', value: 'skeever' }];

	// Get initial theme from localStorage or default to 'skeever'
	let currentTheme = $state(themes[0].value);
	let open = $state(false);
	let elemArrow: HTMLElement | null = $state(null);

	// Use Floating
	const floating = useFloating({
		whileElementsMounted: autoUpdate,
		get open() {
			return open;
		},
		onOpenChange: (v) => {
			open = v;
		},
		placement: 'top',
		get middleware() {
			return [offset(10), flip(), elemArrow && arrow({ element: elemArrow })];
		}
	});

	// Interactions
	const role = useRole(floating.context);
	const click = useClick(floating.context);
	const dismiss = useDismiss(floating.context);
	const interactions = useInteractions([role, click, dismiss]);

	// Function to update theme
	function setTheme(newTheme: string) {
		currentTheme = newTheme;

		// Update localStorage
		if (typeof localStorage !== 'undefined') {
			localStorage.setItem('theme', newTheme);
		}

		// Update DOM
		document.documentElement.setAttribute('data-theme', newTheme);
		document.body.setAttribute('data-theme', newTheme);
	}
</script>

<div class="relative">
	<button
		bind:this={floating.elements.reference}
		{...interactions.getReferenceProps()}
		class="btn-gradient"
	>
		Theme
	</button>
	{#if open}
		<div
			bind:this={floating.elements.floating}
			style={floating.floatingStyles}
			{...interactions.getFloatingProps()}
			class="floating popover-neutral"
			transition:fade={{ duration: 200 }}
		>
			<div class="card bg-surface-900 p-4 shadow-xl">
				<div class="flex flex-col space-y-2">
					{#each themes as theme (theme.value)}
						<button
							class="btn variant-soft {currentTheme === theme.value ? 'variant-filled' : ''}"
							onclick={() => setTheme(theme.value)}
						>
							{theme.name}
						</button>
					{/each}
				</div>
				<FloatingArrow bind:ref={elemArrow} context={floating.context} fill="#575969" />
			</div>
		</div>
	{/if}
</div>
