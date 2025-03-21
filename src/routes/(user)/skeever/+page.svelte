<main class="flex h-full items-center justify-center overflow-hidden">
	<img src="/skeever.png" alt="Skeever" class="skeever-zoom absolute" />
	<p class="hidden-text text-8xl text-white">its squeakin time</p>
</main>

<style>
	.skeever-zoom {
		transform-origin: center;
		animation: zoomEffect 3s ease-in-out forwards;
	}

	.hidden-text {
		opacity: 0;
		animation: showText 0.5s ease-in forwards;
		animation-delay: 2.4s; /* 80% of 3s = 2.4s */
	}

	@keyframes zoomEffect {
		0% {
			opacity: 1;
			transform: scale(1);
		}
		30% {
			opacity: 1;
			transform: scale(0.5); /* Zoom out */
		}
		50% {
			opacity: 1;
			transform: scale(50); /* Zoom in */
		}
		80% {
			opacity: 1;
			transform: scale(50); /* Stay zoomed in */
		}
		81% {
			opacity: 1;
			transform: scale(50);
		}
		100% {
			opacity: 0;
			transform: scale(50);
		}
	}

	@keyframes showText {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
</style>
