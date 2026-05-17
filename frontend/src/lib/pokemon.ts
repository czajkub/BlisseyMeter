/**
 * Gets the Pokemon Showdown sprite URL for a given Pokemon species or nickname string.
 * It parses nicknames formatted like "Poetic Justice (Dragapult)" or "Solo (Reprise) (Terapagos-Terastal)"
 * and extracts/normalizes the Pokemon name to fetch the proper Pokemon Showdown sprite.
 */
export function getPokemonSpriteUrl(pokemonName: string): string {
	if (!pokemonName) {
		return 'https://play.pokemonshowdown.com/sprites/ani/unown.gif';
	}

	// 1. Extract species name inside parentheses if it is a nickname, e.g. "Nick (Pikachu)" -> "Pikachu"
	// Sometime there are multiple parentheses, we match the last one: e.g. "Solo (Reprise) (Terapagos-Terastal)" -> "Terapagos-Terastal"
	const match = pokemonName.match(/\(([^)]+)\)$/) || pokemonName.match(/\(([^)]+)\)/);
	const speciesName = match ? match[1] : pokemonName;

	// Normalize the name according to Pokémon Showdown's ID conversion rules:
	// Lowercase and remove all non-alphanumeric characters.
	const cleanId = speciesName.toLowerCase().trim().replace(/[^a-z0-9]/g, '');

	if (!cleanId) {
		return 'https://play.pokemonshowdown.com/sprites/ani/unown.gif';
	}

	// Pokémon Showdown's standard animated sprite path.
	return `https://play.pokemonshowdown.com/sprites/ani/${cleanId}.gif`;
}
