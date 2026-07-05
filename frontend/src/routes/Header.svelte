<script lang="ts">
	import { base } from '$app/paths';
	import { page } from '$app/state';
</script>

<header>
	<div class="brand">
		<a href="{base}/" class="brand-link">
			<svg class="logo" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
				<path d="M12 2 L20 6 V12 C20 16.5 16.5 20 12 22 C7.5 20 4 16.5 4 12 V6 Z" />
				<circle cx="12" cy="11" r="2.5" />
				<path d="M12 13.5 V17" />
			</svg>
			<span class="brand-name">BlisseyMeter</span>
		</a>
	</div>

	<nav>
		<ul>
			<li class:active={page.url.pathname === '/'}>
				<a href="{base}/">Analyze</a>
			</li>
			<li>
				<a
					href="https://github.com/czajkub/BlisseyMeter"
					target="_blank"
					rel="noopener noreferrer"
				>
					<svg viewBox="0 0 16 16" width="18" height="18" fill="currentColor" aria-hidden="true">
						<path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8Z" />
					</svg>
					<span class="nav-text">GitHub</span>
				</a>
			</li>
		</ul>
	</nav>
</header>

<style>
	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		box-sizing: border-box;
		padding: 0.75rem 1.5rem;
		background: rgba(255, 255, 255, 0.85);
		backdrop-filter: blur(10px);
		border-bottom: 1px solid rgba(0, 0, 0, 0.08);
		position: sticky;
		top: 0;
		z-index: 10;
	}

	.brand-link {
		display: inline-flex;
		align-items: center;
		gap: 0.6rem;
		color: var(--color-text);
		text-decoration: none;
	}

	.logo {
		width: 1.75rem;
		height: 1.75rem;
		color: var(--color-theme-1);
		flex-shrink: 0;
	}

	.brand-name {
		font-weight: 700;
		font-size: 1rem;
		letter-spacing: 0.01em;
		color: #1a202c;
	}

	nav ul {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	li a {
		display: inline-flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.5rem 0.85rem;
		font-size: 0.85rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		color: #4a5568;
		text-decoration: none;
		border-radius: 6px;
		transition: background-color 0.15s ease, color 0.15s ease;
	}

	li a:hover {
		background: rgba(0, 0, 0, 0.05);
		color: #1a202c;
	}

	li.active a {
		color: var(--color-theme-1);
		background: rgba(255, 62, 0, 0.08);
	}

	li.active a::before {
		content: '';
		display: inline-block;
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background: var(--color-theme-1);
		margin-right: 0.25rem;
	}

	@media (max-width: 480px) {
		.nav-text {
			position: absolute;
			width: 1px;
			height: 1px;
			overflow: hidden;
			clip: rect(0 0 0 0);
		}
		.brand-name {
			font-size: 0.9rem;
		}
	}
</style>