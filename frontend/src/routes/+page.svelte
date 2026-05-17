<script lang="ts">
	import { env } from '$env/dynamic/public';
	import LuckGraph from '$lib/LuckGraph.svelte';
	import { getPokemonSpriteUrl } from '$lib/pokemon';

	type LuckEvent = {
		turn: number;
		pokemon: string;
		category: string;
		score: number;
		description: string;
		source_move: string;
		is_beneficial: boolean;
	};

	type AnalysisResult = {
		p1_luck_events: LuckEvent[];
		p2_luck_events: LuckEvent[];
	};

	let replayUrl = $state('');
	let loading = $state(false);
	let error = $state<string | null>(null);
	let result = $state<AnalysisResult | null>(null);

	async function analyzeReplay() {
		if (!replayUrl) return;
		
		loading = true;
		error = null;
		result = null;

		try {
			const apiUrl = env.PUBLIC_API_URL || 'http://localhost:8080';
			const response = await fetch(`${apiUrl}/analyze`, {
				method: 'POST',
				headers: {
					'Content-Type': 'text/plain'
				},
				body: replayUrl
			});

			if (!response.ok) {
				throw new Error(`Error: ${response.status} ${response.statusText}`);
			}

			result = await response.json();
		} catch (e: any) {
			error = e.message || 'Failed to analyze replay';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Pokémon Replay Luck Analyzer</title>
</svelte:head>

<main class="container">
	<h1>Pokémon Replay Luck Analyzer</h1>
	
	<div class="form-container">
		<input 
			type="url" 
			bind:value={replayUrl} 
			placeholder="Enter Pokémon Showdown replay URL..." 
			disabled={loading}
		/>
		<button onclick={analyzeReplay} disabled={loading || !replayUrl}>
			{loading ? 'Analyzing...' : 'Analyze'}
		</button>
	</div>

	{#if error}
		<div class="error">
			{error}
		</div>
	{/if}

	{#if result}
		<LuckGraph {result} />

		<div class="results">
			<div class="player-column">
				<h2>Player 1 Luck Events</h2>
				{#if result.p1_luck_events.length === 0}
					<p>No luck events found.</p>
				{:else}
					<ul class="event-list">
						{#each result.p1_luck_events as event}
							<li class="event {event.is_beneficial ? 'beneficial' : 'detrimental'}">
								<div class="event-header">
									<img src={getPokemonSpriteUrl(event.pokemon)} alt={event.pokemon} class="poke-sprite" />
									<div>
										<span class="turn">Turn {event.turn}</span>
										<span class="pokemon">{event.pokemon}</span>
										<span class="move">({event.source_move})</span>
									</div>
								</div>
								<div class="description">{event.description}</div>
								<div class="score">Impact Score: {event.score}</div>
							</li>
						{/each}
					</ul>
				{/if}
			</div>

			<div class="player-column">
				<h2>Player 2 Luck Events</h2>
				{#if result.p2_luck_events.length === 0}
					<p>No luck events found.</p>
				{:else}
					<ul class="event-list">
						{#each result.p2_luck_events as event}
							<li class="event {event.is_beneficial ? 'beneficial' : 'detrimental'}">
								<div class="event-header">
									<img src={getPokemonSpriteUrl(event.pokemon)} alt={event.pokemon} class="poke-sprite" />
									<div>
										<span class="turn">Turn {event.turn}</span>
										<span class="pokemon">{event.pokemon}</span>
										<span class="move">({event.source_move})</span>
									</div>
								</div>
								<div class="description">{event.description}</div>
								<div class="score">Impact Score: {event.score}</div>
							</li>
						{/each}
					</ul>
				{/if}
			</div>
		</div>
	{/if}
</main>

<style>
	:global(body) {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
		background-color: #f5f7fa;
		color: #333;
		margin: 0;
		padding: 0;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem;
	}

	h1 {
		text-align: center;
		color: #2c3e50;
		margin-bottom: 2rem;
	}

	.form-container {
		display: flex;
		gap: 1rem;
		max-width: 800px;
		margin: 0 auto 3rem auto;
	}

	input {
		flex: 1;
		padding: 0.75rem 1rem;
		font-size: 1rem;
		border: 1px solid #cbd5e0;
		border-radius: 6px;
		outline: none;
	}

	input:focus {
		border-color: #4299e1;
		box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.2);
	}

	button {
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: white;
		background-color: #4299e1;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	button:hover:not(:disabled) {
		background-color: #3182ce;
	}

	button:disabled {
		background-color: #a0aec0;
		cursor: not-allowed;
	}

	.error {
		background-color: #fed7d7;
		color: #c53030;
		padding: 1rem;
		border-radius: 6px;
		text-align: center;
		margin-bottom: 2rem;
	}

	.results {
		display: flex;
		gap: 2rem;
	}

	.player-column {
		flex: 1;
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
	}

	h2 {
		color: #4a5568;
		border-bottom: 2px solid #edf2f7;
		padding-bottom: 0.5rem;
		margin-top: 0;
	}

	.event-list {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.event {
		padding: 1rem;
		border-radius: 6px;
		border-left: 4px solid transparent;
		background-color: #f7fafc;
	}

	.event-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
	}

	.poke-sprite {
		width: 48px;
		height: 48px;
		object-fit: contain;
		background-color: rgba(255, 255, 255, 0.4);
		border-radius: 50%;
	}

	.event.beneficial {
		border-left-color: #48bb78;
		background-color: #f0fff4;
	}

	.event.detrimental {
		border-left-color: #f56565;
		background-color: #fff5f5;
	}

	.turn {
		font-weight: bold;
		color: #4a5568;
		margin-right: 0.5rem;
	}

	.pokemon {
		font-weight: 600;
	}

	.move {
		color: #718096;
		font-size: 0.9em;
	}

	.description {
		margin-top: 0.5rem;
		color: #2d3748;
	}

	.score {
		margin-top: 0.5rem;
		font-size: 0.85em;
		font-weight: bold;
		color: #4a5568;
	}

	@media (max-width: 768px) {
		.results {
			flex-direction: column;
		}
	}
</style>