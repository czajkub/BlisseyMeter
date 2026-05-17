<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import Chart from 'chart.js/auto';

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

	let { result }: { result: AnalysisResult } = $props();

	let canvas: HTMLCanvasElement;
	let chart: Chart | null = null;

	// Whenever the result changes, update the chart
	$effect(() => {
		if (result && canvas) {
			updateChart();
		}
	});

	onDestroy(() => {
		if (chart) {
			chart.destroy();
		}
	});

	function updateChart() {
		// Calculate the max turn
		let maxTurn = 0;
		const allEvents = [...result.p1_luck_events, ...result.p2_luck_events];
		for (const event of allEvents) {
			if (event.turn > maxTurn) {
				maxTurn = event.turn;
			}
		}

		// Ensure we show at least up to turn 1 even if no events
		maxTurn = Math.max(1, maxTurn);

		const labels = Array.from({ length: maxTurn + 1 }, (_, i) => i.toString());

		const p1Data = calculateCumulativeLuck(result.p1_luck_events, maxTurn);
		const p2Data = calculateCumulativeLuck(result.p2_luck_events, maxTurn);
		
		// Net luck difference (Player 1 - Player 2)
		const netData = p1Data.map((p1Val, index) => p1Val - p2Data[index]);

		if (chart) {
			chart.data.labels = labels;
			chart.data.datasets[0].data = p1Data;
			chart.data.datasets[1].data = p2Data;
			chart.data.datasets[2].data = netData;
			chart.update();
		} else {
			chart = new Chart(canvas, {
				type: 'line',
				data: {
					labels,
					datasets: [
						{
							label: 'Player 1 Luck',
							data: p1Data,
							borderColor: 'rgba(54, 162, 235, 1)',
							backgroundColor: 'rgba(54, 162, 235, 0.1)',
							tension: 0.1,
							fill: true
						},
						{
							label: 'Player 2 Luck',
							data: p2Data,
							borderColor: 'rgba(255, 99, 132, 1)',
							backgroundColor: 'rgba(255, 99, 132, 0.1)',
							tension: 0.1,
							fill: true
						},
						{
							label: 'Net Luck (P1 - P2)',
							data: netData,
							borderColor: 'rgba(75, 192, 192, 1)',
							backgroundColor: 'transparent',
							borderWidth: 3,
							borderDash: [5, 5],
							tension: 0.1,
							fill: false
						}
					]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					scales: {
						x: {
							title: {
								display: true,
								text: 'Turn'
							}
						},
						y: {
							title: {
								display: true,
								text: 'Cumulative Luck Score'
							}
						}
					},
					plugins: {
						tooltip: {
							callbacks: {
								label: function(context) {
									const yValue = context.parsed.y !== null && context.parsed.y !== undefined ? context.parsed.y : 0;
									return `${context.dataset.label}: ${yValue.toFixed(2)}`;
								}
							}
						}
					}
				}
			});
		}
	}

	function calculateCumulativeLuck(events: LuckEvent[], maxTurn: number): number[] {
		const data: number[] = new Array(maxTurn + 1).fill(0);
		
		// Map scores per turn
		const scoresPerTurn = new Map<number, number>();
		for (const event of events) {
			const impact = event.is_beneficial ? event.score : -event.score;
			scoresPerTurn.set(event.turn, (scoresPerTurn.get(event.turn) || 0) + impact);
		}

		let cumulative = 0;
		for (let i = 0; i <= maxTurn; i++) {
			cumulative += scoresPerTurn.get(i) || 0;
			data[i] = cumulative;
		}

		return data;
	}
</script>

<div class="chart-container">
	<canvas bind:this={canvas}></canvas>
</div>

<style>
	.chart-container {
		position: relative;
		height: 400px;
		width: 100%;
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
		margin-bottom: 2rem;
		box-sizing: border-box;
	}
</style>