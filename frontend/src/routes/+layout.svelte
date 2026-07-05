<script lang="ts">
	import Header from './Header.svelte';
	import './layout.css';

	let { children } = $props();
</script>

<div class="app">
	<Header />

	<main>
		{@render children()}
	</main>

	<footer>
		<span class="footer-text">
			Built with
			<a href="https://kit.svelte.dev" target="_blank" rel="noopener noreferrer">SvelteKit</a>
			·
			<a
				href="https://github.com/czajkub/replay-analyzer-idk-yet"
				target="_blank"
				rel="noopener noreferrer">Source</a
			>
			·
			Sprites by
			<a href="https://pokemonshowdown.com" target="_blank" rel="noopener noreferrer">Pokémon Showdown</a>
		</span>
	</footer>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1.5rem 1rem;
		width: 100%;
		max-width: 72rem;
		margin: 0 auto;
		box-sizing: border-box;
	}

	footer {
		display: flex;
		justify-content: center;
		align-items: center;
		padding: 1.25rem 1rem;
		border-top: 1px solid rgba(0, 0, 0, 0.06);
		background: rgba(255, 255, 255, 0.6);
		font-size: 0.8rem;
		color: #718096;
	}

	.footer-text {
		display: inline-flex;
		align-items: center;
		gap: 0.35rem;
		flex-wrap: wrap;
		justify-content: center;
	}

	footer a {
		font-weight: 600;
	}
</style>
