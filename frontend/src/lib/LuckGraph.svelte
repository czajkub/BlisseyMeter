<script lang="ts">
	import { onDestroy } from 'svelte';
	import Chart from 'chart.js/auto';

	type LuckEvent = {
		turn: number;
		score: number;
		is_beneficial: boolean;
	};

	let {
		p1Events,
		p2Events,
		p1Name = 'Player 1',
		p2Name = 'Player 2'
	}: {
		p1Events: LuckEvent[];
		p2Events: LuckEvent[];
		p1Name?: string;
		p2Name?: string;
	} = $props();

	let canvas: HTMLCanvasElement;
	let chart: Chart | null = null;

	$effect(() => {
		void p1Events;
		void p2Events;
		void p1Name;
		void p2Name;
		if (canvas) updateChart();
	});

	onDestroy(() => {
		chart?.destroy();
	});

	const COLORS = {
		p1: { line: '#3b82f6', fill: 'rgba(59, 130, 246, 0.12)' },
		p2: { line: '#ef4444', fill: 'rgba(239, 68, 68, 0.12)' },
		net: '#10b981'
	};

	function makeGradient(ctx: CanvasRenderingContext2D, color: string): CanvasGradient {
		const gradient = ctx.createLinearGradient(0, 0, 0, 340);
		gradient.addColorStop(0, color.replace('0.12', '0.28'));
		gradient.addColorStop(1, color.replace('0.12', '0'));
		return gradient;
	}

	function buildData() {
		let maxTurn = 0;
		for (const e of [...p1Events, ...p2Events]) {
			if (e.turn > maxTurn) maxTurn = e.turn;
		}
		maxTurn = Math.max(1, maxTurn);

		const labels = Array.from({ length: maxTurn + 1 }, (_, i) => i.toString());
		const p1Data = cumulative(p1Events, maxTurn);
		const p2Data = cumulative(p2Events, maxTurn);
		const netData = p1Data.map((v, i) => v - p2Data[i]);
		return { labels, p1Data, p2Data, netData, maxTurn };
	}

	function eventMarkers(events: LuckEvent[], color: string, maxTurn: number) {
		return events.map((e) => ({
			type: 'point' as const,
			data: { x: e.turn, y: cumulativeAt(events, e.turn) },
			properties: { r: 4, borderColor: color, backgroundColor: color }
		}));
	}

	function cumulativeAt(events: LuckEvent[], turn: number): number {
		let total = 0;
		for (const e of events) {
			if (e.turn <= turn) total += e.is_beneficial ? e.score : -e.score;
		}
		return total;
	}

	function updateChart() {
		const { labels, p1Data, p2Data, netData, maxTurn } = buildData();
		const ctx = canvas.getContext('2d');

		if (chart) {
			chart.data.labels = labels;
			chart.data.datasets[0].data = p1Data;
			chart.data.datasets[0].label = `${p1Name} Luck`;
			chart.data.datasets[1].data = p2Data;
			chart.data.datasets[1].label = `${p2Name} Luck`;
			chart.data.datasets[2].data = netData;
			chart.update();
		} else if (ctx) {
			chart = new Chart(canvas, {
				type: 'line',
				data: {
					labels,
					datasets: [
						{
							label: `${p1Name} Luck`,
							data: p1Data,
							borderColor: COLORS.p1.line,
							backgroundColor: makeGradient(ctx, COLORS.p1.fill),
							tension: 0.3,
							fill: true,
							pointRadius: 0,
							pointHoverRadius: 6,
							pointHoverBackgroundColor: COLORS.p1.line,
							pointHoverBorderColor: '#fff',
							pointHoverBorderWidth: 2,
							borderWidth: 2.5
						},
						{
							label: `${p2Name} Luck`,
							data: p2Data,
							borderColor: COLORS.p2.line,
							backgroundColor: makeGradient(ctx, COLORS.p2.fill),
							tension: 0.3,
							fill: true,
							pointRadius: 0,
							pointHoverRadius: 6,
							pointHoverBackgroundColor: COLORS.p2.line,
							pointHoverBorderColor: '#fff',
							pointHoverBorderWidth: 2,
							borderWidth: 2.5
						},
						{
							label: 'Net Luck',
							data: netData,
							borderColor: COLORS.net,
							backgroundColor: 'transparent',
							borderWidth: 2,
							borderDash: [6, 4],
							tension: 0.3,
							fill: false,
							pointRadius: 0,
							pointHoverRadius: 5,
							pointHoverBackgroundColor: COLORS.net,
							pointHoverBorderColor: '#fff',
							pointHoverBorderWidth: 2
						}
					]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					interaction: { mode: 'index', intersect: false },
					animation: { duration: 600, easing: 'easeOutQuart' },
					scales: {
						x: {
							title: {
								display: true,
								text: 'Turn',
								color: '#4a5568',
								font: { size: 12, weight: 'bold' as const }
							},
							grid: { display: false },
							border: { display: false },
							ticks: { color: '#a0aec0', font: { size: 11 } }
						},
						y: {
							title: {
								display: true,
								text: 'Cumulative Luck Score',
								color: '#4a5568',
								font: { size: 12, weight: 'bold' as const }
							},
							grid: { color: 'rgba(0, 0, 0, 0.05)' },
							border: { display: false },
							ticks: { color: '#a0aec0', font: { size: 11 } }
						}
					},
					plugins: {
						legend: {
							position: 'top',
							align: 'end',
							labels: {
								usePointStyle: true,
								pointStyle: 'circle',
								padding: 18,
								color: '#2d3748',
								font: { size: 12, weight: 'bold' as const }
							}
						},
						tooltip: {
							backgroundColor: 'rgba(26, 32, 44, 0.92)',
							titleColor: '#fff',
							bodyColor: '#e2e8f0',
							borderColor: 'rgba(255, 255, 255, 0.1)',
							borderWidth: 1,
							padding: 12,
							cornerRadius: 8,
							displayColors: true,
							boxPadding: 6,
							titleFont: { size: 13, weight: 'bold' as const },
							bodyFont: { size: 12 },
							callbacks: {
								title: (items) => `Turn ${items[0].label}`,
								label: (ctx) => {
									const y = ctx.parsed.y ?? 0;
									const sign = y > 0 ? '+' : '';
									return `  ${ctx.dataset.label}: ${sign}${y.toFixed(2)}`;
								}
							}
						}
					}
				}
			});
		}
	}

	function cumulative(events: LuckEvent[], maxTurn: number): number[] {
		const data: number[] = new Array(maxTurn + 1).fill(0);
		const perTurn = new Map<number, number>();
		for (const e of events) {
			const impact = e.is_beneficial ? e.score : -e.score;
			perTurn.set(e.turn, (perTurn.get(e.turn) ?? 0) + impact);
		}
		let cumulative = 0;
		for (let i = 0; i <= maxTurn; i++) {
			cumulative += perTurn.get(i) ?? 0;
			data[i] = cumulative;
		}
		return data;
	}
</script>

<div class="chart-card">
	<div class="chart-header">
		<h3>Cumulative Luck Over Time</h3>
		<p class="chart-sub">Net luck difference (dashed) = {p1Name} − {p2Name}</p>
	</div>
	<div class="chart-container">
		<canvas bind:this={canvas}></canvas>
	</div>
</div>

<style>
	.chart-card {
		background: white;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06), 0 4px 12px rgba(0, 0, 0, 0.04);
		margin-bottom: 1.5rem;
		overflow: hidden;
	}

	.chart-header {
		padding: 1.25rem 1.5rem 0.5rem 1.5rem;
	}

	.chart-header h3 {
		margin: 0 0 0.25rem 0;
		font-size: 1rem;
		font-weight: 700;
		color: #1a202c;
	}

	.chart-sub {
		margin: 0;
		font-size: 0.8rem;
		color: #a0aec0;
	}

	.chart-container {
		position: relative;
		height: 380px;
		width: 100%;
		padding: 0.5rem 1.5rem 1.5rem 1.5rem;
		box-sizing: border-box;
	}
</style>