<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
	import LuckGraph from '$lib/LuckGraph.svelte';
	import {
		getPokemonSpriteUrl,
		getPokemonStaticSpriteUrl,
		POKEMON_FALLBACK_URL,
		getAvatarUrl,
		AVATAR_FALLBACK_URL,
	} from '$lib/pokemon';

	// serde externally-tagged enum: variants like "CriticalHit" become strings,
	// `Other(String)` becomes { Other: "..." }.
	type RawCategory = string | { Other: string };
	type SortMode = 'turn' | 'score-desc' | 'score-asc' | 'beneficial' | 'detrimental';
	type LuckEvent = {
		turn: number;
		pokemon: string;
		category: RawCategory;
		score: number;
		description: string;
		source_move: string | null;
		is_beneficial: boolean;
	};

	type PlayerData = {
		name: string;
		avatar: string;
		events: LuckEvent[];
	};

	type AnalysisResult = {
		p1: PlayerData;
		p2: PlayerData;
	};

	function categoryLabel(c: RawCategory): string {
		return typeof c === 'string' ? c : c.Other ? `Other: ${c.Other}` : 'Other';
	}

	let replayUrl = $state('');
	let loading = $state(false);
	let error = $state<string | null>(null);
	let result = $state<AnalysisResult | null>(null);

	// Filter state
	let hiddenCategories = $state<Set<string>>(new Set());
	let hiddenPokemon = $state<Set<string>>(new Set());
	let disabledEvents = $state<Set<string>>(new Set());
	let sortMode = $state<SortMode>('turn');

	function eventKey(e: LuckEvent): string {
		return `${e.turn}|${e.pokemon}|${categoryLabel(e.category)}|${e.description}`;
	}

	function toggleEvent(e: LuckEvent) {
		const key = eventKey(e);
		const next = new Set(disabledEvents);
		if (next.has(key)) next.delete(key);
		else next.add(key);
		disabledEvents = next;
	}

	function toggleCategory(cat: string) {
		const next = new Set(hiddenCategories);
		if (next.has(cat)) next.delete(cat);
		else next.add(cat);
		hiddenCategories = next;
	}

	function togglePokemon(p: string) {
		const next = new Set(hiddenPokemon);
		if (next.has(p)) next.delete(p);
		else next.add(p);
		hiddenPokemon = next;
	}

	function resetFilters() {
		hiddenCategories = new Set();
		hiddenPokemon = new Set();
		disabledEvents = new Set();
		sortMode = 'turn';
	}

	const allCategories = $derived(
		result
			? Array.from(
					new Set(
						[...result.p1.events, ...result.p2.events].map((e) =>
							categoryLabel(e.category)
						)
					)
				).sort()
			: []
	);

	const allPokemon = $derived(
		result
			? Array.from(
					new Set(
						[...result.p1.events, ...result.p2.events].map((e) => e.pokemon)
					)
				).sort()
			: []
	);

	function passives(events: LuckEvent[]): LuckEvent[] {
		return events.filter((e) => {
			if (hiddenCategories.has(categoryLabel(e.category))) return false;
			if (hiddenPokemon.has(e.pokemon)) return false;
			return true;
		});
	}

	function sortEvents(events: LuckEvent[]): LuckEvent[] {
		const sorted = [...events];
		switch (sortMode) {
			case 'turn':
				sorted.sort((a, b) => a.turn - b.turn);
				break;
			case 'score-desc':
				sorted.sort((a, b) => Math.abs(b.score) - Math.abs(a.score));
				break;
			case 'score-asc':
				sorted.sort((a, b) => Math.abs(a.score) - Math.abs(b.score));
				break;
			case 'beneficial':
				sorted.sort((a, b) => Number(b.is_beneficial) - Number(a.is_beneficial) || a.turn - b.turn);
				break;
			case 'detrimental':
				sorted.sort((a, b) => Number(a.is_beneficial) - Number(b.is_beneficial) || a.turn - b.turn);
				break;
		}
		return sorted;
	}

	function applyFilters(events: LuckEvent[]): LuckEvent[] {
		return sortEvents(passives(events));
	}

	function enabledEvents(events: LuckEvent[]): LuckEvent[] {
		return passives(events).filter((e) => !disabledEvents.has(eventKey(e)));
	}

	const filteredP1 = $derived(result ? applyFilters(result.p1.events) : []);
	const filteredP2 = $derived(result ? applyFilters(result.p2.events) : []);
	const enabledP1 = $derived(result ? enabledEvents(result.p1.events) : []);
	const enabledP2 = $derived(result ? enabledEvents(result.p2.events) : []);

	function resetFilterStateOnNewAnalysis() {
		hiddenCategories = new Set();
		hiddenPokemon = new Set();
		disabledEvents = new Set();
		sortMode = 'turn';
	}

	const scoreColor = (score: number) =>
		score > 0 ? '#38a169' : score < 0 ? '#e53e3e' : '#718096';

	const scoreLabel = (score: number) => (score > 0 ? `+${score}` : `${score}`);

	async function analyzeReplay() {
		if (!replayUrl) return;
		
		loading = true;
		error = null;
		result = null;
		resetFilterStateOnNewAnalysis();

		try {
			const apiUrl = PUBLIC_API_URL || 'http://localhost:8080';
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
	<title>BlisseyMeter</title>
</svelte:head>

<main class="container">
	<h1>BlisseyMeter</h1>
	
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
		<div class="filter-bar">
			<div class="filter-row">
				<label class="filter-field">
					<span class="filter-label">Sort</span>
					<select bind:value={sortMode}>
						<option value="turn">Turn (ascending)</option>
						<option value="score-desc">Impact (high → low)</option>
						<option value="score-asc">Impact (low → high)</option>
						<option value="beneficial">Beneficial first</option>
						<option value="detrimental">Detrimental first</option>
					</select>
				</label>

				<button class="reset-btn" onclick={resetFilters}>Reset filters</button>
			</div>

			{#if allPokemon.length > 0}
				<div class="filter-row chip-section">
					<span class="filter-label">Pokémon</span>
					<div class="chip-row">
						{#each allPokemon as p}
							<button
								class="chip poke-chip"
								class:active={!hiddenPokemon.has(p)}
								onclick={() => togglePokemon(p)}
							>
								<img
									src={getPokemonSpriteUrl(p)}
									alt=""
									class="chip-sprite"
									onerror={(e) => {
										const img = e.currentTarget as HTMLImageElement;
										const next =
											img.src === getPokemonSpriteUrl(p)
												? getPokemonStaticSpriteUrl(p)
												: POKEMON_FALLBACK_URL;
										if (img.src !== next) img.src = next;
									}}
								/>
								<span>{p}</span>
							</button>
						{/each}
					</div>
				</div>
			{/if}

			{#if allCategories.length > 0}
				<div class="filter-row chip-section">
					<span class="filter-label">Categories</span>
					<div class="chip-row">
						{#each allCategories as cat}
							<button
								class="chip"
								class:active={!hiddenCategories.has(cat)}
								onclick={() => toggleCategory(cat)}
							>
								{cat}
							</button>
						{/each}
					</div>
				</div>
			{/if}
		</div>

		<LuckGraph p1Events={enabledP1} p2Events={enabledP2} p1Name={result.p1.name} p2Name={result.p2.name} />

		<div class="results">
{@render playerColumn(result.p1.name, result.p1.avatar, filteredP1)}
		{@render playerColumn(result.p2.name, result.p2.avatar, filteredP2)}
		</div>
	{/if}
</main>

{#snippet playerColumn(name: string, avatar: string, events: LuckEvent[])}
	<section class="player-column">
		<header class="player-header">
			<img
				src={getAvatarUrl(avatar)}
				alt={name}
				class="player-avatar"
				onerror={(e) => {
					const img = e.currentTarget as HTMLImageElement;
					if (img.src !== AVATAR_FALLBACK_URL) img.src = AVATAR_FALLBACK_URL;
				}}
			/>
			<h2>{name || 'Player'}</h2>
			<span class="event-count">{events.length} event{events.length === 1 ? '' : 's'}</span>
		</header>

		{#if events.length === 0}
			<p class="empty">No luck events found.</p>
		{:else}
			<div class="event-list">
				{#each events as event}
					{@const disabled = disabledEvents.has(eventKey(event))}
					<button
						type="button"
						class="event {event.is_beneficial ? 'beneficial' : 'detrimental'}"
						class:disabled
						onclick={() => toggleEvent(event)}
						title={disabled ? 'Click to include in chart' : 'Click to exclude from chart'}
						aria-pressed={disabled}
					>
						<div class="event-marker"></div>
						<div class="event-body">
							<div class="event-top">
								<img
									src={getPokemonSpriteUrl(event.pokemon)}
									alt={event.pokemon}
									class="poke-sprite"
									onerror={(e) => {
										const img = e.currentTarget as HTMLImageElement;
										const next =
											img.src === getPokemonSpriteUrl(event.pokemon)
												? getPokemonStaticSpriteUrl(event.pokemon)
												: POKEMON_FALLBACK_URL;
										if (img.src !== next) img.src = next;
									}}
								/>
								<div class="event-meta">
									<span class="pokemon">{event.pokemon}</span>
									{#if event.source_move}
										<span class="move">{event.source_move}</span>
									{/if}
								</div>
								<span class="score-badge" style="color: {scoreColor(event.score)}">
									{scoreLabel(event.score)}
								</span>
							</div>
							<p class="description">{event.description}</p>
							<div class="event-footer">
								<span class="turn">Turn {event.turn}</span>
								<span class="category">{categoryLabel(event.category)}</span>
							</div>
						</div>
					</button>
				{/each}
			</div>
		{/if}
	</section>
{/snippet}

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

	.filter-bar {
		background: white;
		border-radius: 12px;
		padding: 1rem 1.25rem;
		margin-bottom: 1.5rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06), 0 4px 12px rgba(0, 0, 0, 0.04);
		display: flex;
		flex-direction: column;
		gap: 0.85rem;
	}

	.filter-row {
		display: flex;
		align-items: flex-end;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.filter-field {
		display: flex;
		flex-direction: column;
		gap: 0.3rem;
	}

	.filter-label {
		font-size: 0.7rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: #718096;
	}

	.filter-bar select {
		padding: 0.55rem 0.7rem;
		font-size: 0.9rem;
		border: 1px solid #cbd5e0;
		border-radius: 6px;
		background: white;
		color: #2d3748;
		outline: none;
		font-family: inherit;
	}

	.filter-bar select:focus {
		border-color: #4299e1;
		box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.2);
	}

	.reset-btn {
		padding: 0.55rem 1rem;
		font-size: 0.85rem;
		font-weight: 600;
		color: #4a5568;
		background: #edf2f7;
		border: 1px solid #cbd5e0;
		border-radius: 6px;
		cursor: pointer;
		transition: background 0.15s, color 0.15s;
	}

	.reset-btn:hover {
		background: #e2e8f0;
		color: #2d3748;
	}

	.chip-section {
		align-items: center;
	}

	.chip-row {
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
	}

	.poke-chip {
		display: inline-flex;
		align-items: center;
		gap: 0.4rem;
		padding-right: 0.85rem;
	}

	.chip-sprite {
		width: 28px;
		height: 28px;
		object-fit: contain;
		image-rendering: pixelated;
		flex-shrink: 0;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.4);
	}

	.chip {
		padding: 0.35rem 0.75rem;
		font-size: 0.8rem;
		font-weight: 600;
		color: #4a5568;
		background: #f7fafc;
		border: 1px solid #e2e8f0;
		border-radius: 999px;
		cursor: pointer;
		transition: all 0.15s ease;
		font-family: inherit;
	}

	.chip.active {
		background: #2b6cb0;
		color: white;
		border-color: #2b6cb0;
	}

	.chip:not(.active) {
		opacity: 0.55;
		text-decoration: line-through;
	}

	.chip:hover {
		opacity: 1;
	}

	.results {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
	}

	.player-column {
		background: white;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06), 0 4px 12px rgba(0, 0, 0, 0.04);
		overflow: hidden;
	}

	.player-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid #edf2f7;
		background: #f7fafc;
	}

	.player-avatar {
		width: 48px;
		height: 48px;
		object-fit: contain;
		image-rendering: pixelated;
		flex-shrink: 0;
	}

	.player-header h2 {
		margin: 0;
		padding: 0;
		flex: 1;
		border: none;
		font-size: 1.1rem;
		font-weight: 700;
		color: #2d3748;
	}

	.event-count {
		font-size: 0.75rem;
		font-weight: 600;
		color: #718096;
		text-transform: uppercase;
		letter-spacing: 0.06em;
	}

	.empty {
		padding: 2rem 1.25rem;
		text-align: center;
		color: #a0aec0;
		margin: 0;
	}

	.event-list {
		list-style: none;
		padding: 0.75rem;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 0.6rem;
	}

	.event {
		display: flex;
		width: 100%;
		border: none;
		padding: 0;
		margin: 0;
		text-align: left;
		font: inherit;
		color: inherit;
		border-radius: 8px;
		overflow: hidden;
		background: #f7fafc;
		cursor: pointer;
		transition: transform 0.12s ease, box-shadow 0.12s ease, opacity 0.15s ease;
	}

	.event:hover {
		transform: translateY(-1px);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}

	.event.disabled {
		opacity: 0.4;
		filter: grayscale(0.85);
	}

	.event.disabled .description {
		text-decoration: line-through;
	}

	.event.disabled:hover {
		opacity: 0.7;
		filter: grayscale(0.5);
	}

	.event-marker {
		flex-shrink: 0;
		width: 4px;
		align-self: stretch;
	}

	.event.beneficial .event-marker {
		background: linear-gradient(180deg, #48bb78, #38a169);
	}

	.event.detrimental .event-marker {
		background: linear-gradient(180deg, #f56565, #e53e3e);
	}

	.event-body {
		flex: 1;
		padding: 0.75rem 0.9rem;
		min-width: 0;
	}

	.event-top {
		display: flex;
		align-items: center;
		gap: 0.65rem;
	}

	.poke-sprite {
		width: 44px;
		height: 44px;
		object-fit: contain;
		image-rendering: pixelated;
		flex-shrink: 0;
	}

	.event-meta {
		display: flex;
		flex-direction: column;
		min-width: 0;
		flex: 1;
	}

	.pokemon {
		font-weight: 700;
		font-size: 0.95rem;
		color: #1a202c;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.move {
		font-size: 0.8rem;
		color: #718096;
		font-style: italic;
	}

	.score-badge {
		font-weight: 800;
		font-size: 1.05rem;
		font-variant-numeric: tabular-nums;
		flex-shrink: 0;
	}

	.description {
		margin: 0.55rem 0 0.5rem 0;
		font-size: 0.88rem;
		line-height: 1.45;
		color: #2d3748;
	}

	.event-footer {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.72rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-weight: 600;
		color: #718096;
	}

	.event-footer .turn {
		color: #4a5568;
	}

	.category {
		padding: 0.1rem 0.45rem;
		border-radius: 4px;
		background: #edf2f7;
	}

	@media (max-width: 768px) {
		.results {
			grid-template-columns: 1fr;
		}
	}
</style>