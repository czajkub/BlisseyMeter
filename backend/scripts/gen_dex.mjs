#!/usr/bin/env node
// Generates the bundled Pokemon data files under src/constants/data/.
//
// Usage:
//   node backend/scripts/gen_dex.mjs
//
// Sources (Pokemon Showdown; MIT licensed - see src/constants/data/SOURCE.md):
//   https://play.pokemonshowdown.com/data/pokedex.json
//   https://play.pokemonshowdown.com/data/moves.json
//   https://play.pokemonshowdown.com/data/typechart.js   (CommonJS)
//   https://play.pokemonshowdown.com/data/abilities.js   (CommonJS)
//   https://play.pokemonshowdown.com/data/items.js       (CommonJS)
//
// Natures never change and are not exposed as a data file upstream, so they are
// hard-coded below.

import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import vm from "node:vm";

const OUTPUT_DIR = join(
  dirname(fileURLToPath(import.meta.url)),
  "..",
  "src",
  "constants",
  "data",
);

const SOURCES = {
  pokedex: "https://play.pokemonshowdown.com/data/pokedex.json",
  moves: "https://play.pokemonshowdown.com/data/moves.json",
  typechart: "https://play.pokemonshowdown.com/data/typechart.js",
  abilities: "https://play.pokemonshowdown.com/data/abilities.js",
  items: "https://play.pokemonshowdown.com/data/items.js",
};

async function fetchText(url) {
  const res = await fetch(url);
  if (!res.ok) throw new Error(`GET ${url} -> ${res.status} ${res.statusText}`);
  return res.text();
}

async function fetchJson(url) {
  return JSON.parse(await fetchText(url));
}

// The upstream .js files are self-contained CommonJS (`exports.BattleX = {...}`)
// with no require() calls, so a throwaway sandbox is enough to evaluate them.
function evalCommonJs(code) {
  const sandbox = { exports: {} };
  vm.runInNewContext(code, sandbox);
  return sandbox.exports;
}

const cap = (s) => s.charAt(0).toUpperCase() + s.slice(1);

// Recursively sort object keys so regenerated files produce stable diffs.
function sortKeys(value) {
  if (Array.isArray(value)) return value.map(sortKeys);
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, sortKeys(value[key])]),
    );
  }
  return value;
}

// ---------------------------------------------------------------------------
// Species
// ---------------------------------------------------------------------------

function trimSpecies(entry) {
  // Cosmetic-only formes (Vivillon patterns, Alcremie swirls, ...) carry no
  // stats and never appear in battle; fall back to baseSpecies at runtime.
  if (!entry.baseStats) return null;

  return {
    name: entry.name,
    num: entry.num,
    types: entry.types,
    baseStats: {
      hp: entry.baseStats.hp,
      atk: entry.baseStats.atk,
      def: entry.baseStats.def,
      spa: entry.baseStats.spa,
      spd: entry.baseStats.spd,
      spe: entry.baseStats.spe,
    },
    abilities: entry.abilities,
    weightkg: entry.weightkg,
    baseSpecies: entry.baseSpecies ?? null,
    forme: entry.forme ?? null,
    isNonstandard: entry.isNonstandard ?? null,
  };
}

function buildSpecies(pokedex) {
  const out = {};
  for (const [id, entry] of Object.entries(pokedex)) {
    const trimmed = trimSpecies(entry);
    if (trimmed) out[id] = trimmed;
  }
  return out;
}

// ---------------------------------------------------------------------------
// Moves
// ---------------------------------------------------------------------------

const EFFECT_KEYS = [
  "chance",
  "status",
  "volatileStatus",
  "boosts",
  "self",
  "sideCondition",
  "slotCondition",
  "terrain",
  "weather",
  "pseudoWeather",
  "forceSwitch",
  "target",
];

function trimEffect(effect) {
  const out = {};
  for (const key of EFFECT_KEYS) {
    if (effect[key] !== undefined) out[key] = effect[key];
  }
  return out;
}

// `secondary` and `secondaries` are unified into a single array. This is the
// authoritative flinch/secondary source (e.g. Air Slash carries
// {chance: 30, volatileStatus: "flinch"}; Low Kick carries none).
function collectSecondaries(move) {
  const out = [];
  if (move.secondary) out.push(trimEffect(move.secondary));
  if (Array.isArray(move.secondaries)) {
    for (const effect of move.secondaries) out.push(trimEffect(effect));
  }
  return out;
}

function trimMove(move) {
  return {
    name: move.name,
    type: move.type,
    category: move.category,
    basePower: move.basePower,
    // `true` means the move skips the accuracy/evasion check entirely.
    accuracy: move.accuracy === true ? null : move.accuracy,
    pp: move.pp,
    priority: move.priority,
    target: move.target,
    flags: move.flags ? Object.keys(move.flags).filter((k) => move.flags[k]) : [],
    critRatio: move.critRatio ?? 1,
    willCrit: move.willCrit ?? false,
    multihit: move.multihit ?? null,
    multiaccuracy: move.multiaccuracy ?? false,
    // Variable base power (Low Kick, Gyro Ball, Heavy Slam, ...) is computed
    // in Rust; the static basePower above is only a placeholder.
    variablePower: move.basePowerCallback === true,
    secondaries: collectSecondaries(move),
    boosts: move.boosts ?? null,
    self: move.self ?? move.selfBoost ?? null,
    drain: move.drain ?? null,
    recoil: move.recoil ?? null,
    heal: move.heal ?? null,
    selfSwitch: move.selfSwitch ?? false,
    ignoreAbility: move.ignoreAbility ?? false,
    breaksProtect: move.breaksProtect ?? false,
    stallingMove: move.stallingMove ?? false,
    status: move.status ?? null,
    volatileStatus: move.volatileStatus ?? null,
    sideCondition: move.sideCondition ?? null,
    weather: move.weather ?? null,
    terrain: move.terrain ?? null,
    pseudoWeather: move.pseudoWeather ?? null,
    damage: move.damage ?? null,
    ohko: move.ohko ?? false,
  };
}

function buildMoves(moves) {
  const out = {};
  for (const [id, move] of Object.entries(moves)) out[id] = trimMove(move);
  return out;
}

// ---------------------------------------------------------------------------
// Type chart
// ---------------------------------------------------------------------------

// Showdown stores effectiveness on the *defender* as damage-taken codes.
const DAMAGE_TAKEN_MULTIPLIER = { 0: 1, 1: 2, 2: 0.5, 3: 0 };

// `damageTaken` also carries non-type pseudo-keys (`prankster`, `Brn`, `Par`,
// `Trapped`, `Powder`, ...) for ability/status interactions. Those are handled
// by code in Rust, so only real types are kept here.
const TYPES = new Set([
  "Bug",
  "Dark",
  "Dragon",
  "Electric",
  "Fairy",
  "Fighting",
  "Fire",
  "Flying",
  "Ghost",
  "Grass",
  "Ground",
  "Ice",
  "Normal",
  "Poison",
  "Psychic",
  "Rock",
  "Steel",
  "Stellar",
  "Water",
]);

function buildTypeChart(chart) {
  const out = {};
  for (const [defenderId, data] of Object.entries(chart)) {
    const defender = cap(defenderId);
    if (!TYPES.has(defender)) continue;
    for (const [attackerId, code] of Object.entries(data.damageTaken ?? {})) {
      const attacker = cap(attackerId);
      if (!TYPES.has(attacker)) continue;
      const multiplier = DAMAGE_TAKEN_MULTIPLIER[code];
      if (multiplier === undefined || multiplier === 1) continue;
      out[attacker] ??= {};
      out[attacker][defender] = multiplier;
    }
  }
  return out;
}

// ---------------------------------------------------------------------------
// Abilities / items
// ---------------------------------------------------------------------------

function trimNamed(entry) {
  return {
    name: entry.name,
    num: entry.num ?? 0,
    isNonstandard: entry.isNonstandard ?? null,
  };
}

function buildNamed(table) {
  const out = {};
  for (const [id, entry] of Object.entries(table)) out[id] = trimNamed(entry);
  return out;
}

// ---------------------------------------------------------------------------
// Natures (static; not available as JSON upstream)
// ---------------------------------------------------------------------------

const NATURES = [
  ["adamant", "Adamant", "atk", "spa"],
  ["bashful", "Bashful", null, null],
  ["bold", "Bold", "def", "atk"],
  ["brave", "Brave", "atk", "spe"],
  ["calm", "Calm", "spd", "atk"],
  ["careful", "Careful", "spd", "spa"],
  ["docile", "Docile", null, null],
  ["gentle", "Gentle", "spd", "def"],
  ["hardy", "Hardy", null, null],
  ["hasty", "Hasty", "spe", "def"],
  ["impish", "Impish", "def", "spa"],
  ["jolly", "Jolly", "spe", "spa"],
  ["lax", "Lax", "def", "spd"],
  ["lonely", "Lonely", "atk", "def"],
  ["mild", "Mild", "spa", "def"],
  ["modest", "Modest", "spa", "atk"],
  ["naive", "Naive", "spe", "spd"],
  ["naughty", "Naughty", "atk", "spd"],
  ["quiet", "Quiet", "spa", "spe"],
  ["quirky", "Quirky", null, null],
  ["rash", "Rash", "spa", "spd"],
  ["relaxed", "Relaxed", "def", "spe"],
  ["sassy", "Sassy", "spd", "spe"],
  ["serious", "Serious", null, null],
  ["timid", "Timid", "spe", "atk"],
];

function buildNatures() {
  const out = {};
  for (const [id, name, plus, minus] of NATURES) out[id] = { name, plus, minus };
  return out;
}

// ---------------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------------

async function writeDataset(name, data) {
  const path = join(OUTPUT_DIR, `${name}.json`);
  await writeFile(path, `${JSON.stringify(sortKeys(data), null, 2)}\n`, "utf8");
  return path;
}

async function writeSourceDoc() {
  const lines = [
    "# Bundled Pokemon data",
    "",
    "Auto-generated by `backend/scripts/gen_dex.mjs` - do not edit by hand.",
    "",
    `Snapshot generated: ${new Date().toISOString().slice(0, 10)}`,
    "",
    "| File | Source |",
    "|---|---|",
    ...Object.entries({
      species: SOURCES.pokedex,
      moves: SOURCES.moves,
      typechart: SOURCES.typechart,
      abilities: SOURCES.abilities,
      items: SOURCES.items,
      natures: "(hard-coded in gen_dex.mjs)",
    }).map(([file, source]) => `| ${file}.json | ${source} |`),
    "",
    "Data originates from the [Pokemon Showdown](https://github.com/smogon/pokemon-showdown)",
    "project, which is licensed under the MIT License. Regenerate with:",
    "",
    "```bash",
    "node backend/scripts/gen_dex.mjs",
    "```",
    "",
  ];
  const path = join(OUTPUT_DIR, "SOURCE.md");
  await writeFile(path, lines.join("\n"), "utf8");
  return path;
}

async function main() {
  console.log("Fetching Showdown data...");
  const [pokedex, moves, typechartCode, abilitiesCode, itemsCode] =
    await Promise.all([
      fetchJson(SOURCES.pokedex),
      fetchJson(SOURCES.moves),
      fetchText(SOURCES.typechart),
      fetchText(SOURCES.abilities),
      fetchText(SOURCES.items),
    ]);

  const typechart = evalCommonJs(typechartCode).BattleTypeChart;
  const abilities = evalCommonJs(abilitiesCode).BattleAbilities;
  const items = evalCommonJs(itemsCode).BattleItems;
  if (!typechart || !abilities || !items) {
    throw new Error("Failed to evaluate one of the Showdown CommonJS data files");
  }

  const datasets = {
    species: buildSpecies(pokedex),
    moves: buildMoves(moves),
    typechart: buildTypeChart(typechart),
    abilities: buildNamed(abilities),
    items: buildNamed(items),
    natures: buildNatures(),
  };

  await mkdir(OUTPUT_DIR, { recursive: true });

  for (const [name, data] of Object.entries(datasets)) {
    await writeDataset(name, data);
    console.log(`  ${name}.json: ${Object.keys(data).length} entries`);
  }
  await writeSourceDoc();
  console.log(`Wrote data to ${join(OUTPUT_DIR, "*.json")}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
