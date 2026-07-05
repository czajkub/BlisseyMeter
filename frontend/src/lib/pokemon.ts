/**
 * Gets the Pokemon Showdown sprite URL for a given Pokemon species or nickname string.
 * It parses nicknames formatted like "Poetic Justice (Dragapult)" or "Solo (Reprise) (Terapagos-Terastal)"
 * and extracts/normalizes the Pokemon name to fetch the proper Pokemon Showdown sprite.
 *
 * Fallback chain: animated gif -> static PNG -> unown.
 */

const FALLBACK = 'unown';

function toSpriteId(name: string): string {
	// Pokémon Showdown sprite filenames use hyphens for multi-word species and forms
	// (e.g. mr-mime, ho-oh, mega-charizard-x, porygon-z). Lowercase, collapse spaces
	// to hyphens, drop apostrophes/periods/commas, leave hyphens intact.
	return name
		.toLowerCase()
		.trim()
		.replace(/['.,]/g, '')
		.replace(/\s+/g, '-')
		.replace(/[^a-z0-9-]/g, '')
		.replace(/-+/g, '-');
}

function extractSpecies(pokemonName: string): string {
	if (!pokemonName) return FALLBACK;
	// Match the last parenthesized group if it's a nickname like "Poetic Justice (Dragapult)"
	// or "Solo (Reprise) (Terapagos-Terastal)".
	const match = pokemonName.match(/\(([^)]+)\)$/) || pokemonName.match(/\(([^)]+)\)/);
	return match ? match[1] : pokemonName;
}

function spriteId(pokemonName: string): string {
	return toSpriteId(extractSpecies(pokemonName)) || FALLBACK;
}

export function getPokemonSpriteUrl(pokemonName: string): string {
	return `https://play.pokemonshowdown.com/sprites/ani/${spriteId(pokemonName)}.gif`;
}

export function getPokemonStaticSpriteUrl(pokemonName: string): string {
	return `https://play.pokemonshowdown.com/sprites/ani/${spriteId(pokemonName)}.png`;
}

export const POKEMON_FALLBACK_URL = `https://play.pokemonshowdown.com/sprites/ani/${FALLBACK}.gif`;

const AVATAR_FALLBACK = 'unknown';

export function getAvatarUrl(avatar: string): string {
	const id = (avatar || '').toLowerCase().trim() || AVATAR_FALLBACK;
	return `https://play.pokemonshowdown.com/sprites/trainers/${id}.png`;
}

export const AVATAR_FALLBACK_URL = `https://play.pokemonshowdown.com/sprites/trainers/${AVATAR_FALLBACK}.png`;