# BlisseyMeter

> How lucky were you, really?

BlisseyMeter analyzes [Pokemon Showdown](https://pokemonshowdown.com) replays and measures the
luck involved in each player's game — crits, misses, secondary effects, flinches, status rolls,
and turns them into per-event scores so you can see at a glance whether a win was earned or handed
out by RNG.

- **Frontend**: <https://czajkub.pl>
- **API**: <https://api.czajkub.pl>

---

## Tech stack

### Backend (`backend/`)

- **Rust** with **axum** as the HTTP framework.
- Deployed as an **AWS Lambda** function behind **API Gateway**.
- For local development the same binary falls back to a plain tokio `TcpListener` on
  `127.0.0.1:$PORT` (default `8080`) — no Lambda runtime required.
- Local dev: run with `cargo run`

### Frontend (`frontend/`)

- Build with **SvelteKit**.
- Deployed on **Cloudflare Pages**
- Pokemon and trainer sprites are loaded directly from the Pokemon Showdown CDN
  (`play.pokemonshowdown.com/sprites/...`) — no sprite assets are bundled.
- tbh the frontend is vibe coded, full disclosure - the backend is not though :P

Frontend workspaces are managed with `pnpm`
---

## How Showdown replay logs work

A Showdown replay at `https://replay.pokemonshowdown.com/<format>-<id>` is aHTML
page, but the pure logs are available at the same URL with
`.log` appended:

```
https://replay.pokemonshowdown.com/gen9ou-1234567890.log
```

The `.log` file is a plain-text stream of `|`-delimited lines. Each line
has a type marker in its first field, then type-specific fields:

```
|turn|2
|
|t:|1768157977
|move|p1a: New Person SOM|U-turn|p2a: Pecharunt
|-resisted|p2a: Pecharunt
|-damage|p2a: Pecharunt|99/100
|
|t:|1768158003
|switch|p1a: Touch The Sky|Bombirdier, F|100/100|[from] U-turn
|move|p2a: Pecharunt|Parting Shot|p1a: Touch The Sky
|-unboost|p1a: Touch The Sky|atk|1
|-unboost|p1a: Touch The Sky|spa|1
|
|t:|1768158016
|switch|p2a: Swellow|Swellow, M|100/100|[from] Parting Shot
|
|upkeep
```

Lines fall into three groups:

| Group | Examples | Role |
|---|---|---|
| **Main lines** | `move`, `switch`, `faint`, `cant`, `-curestatus`, `detailschange` | Something happened to a Pokemon. |
| **Sub lines** | `-damage`, `-heal`, `-crit`, `-miss`, `-status`, `-boost`, `-activate`, `-terastallize`, etc | A modifier attached to the most recent main line (the cause/effect detail behind a `move`). |
| **Info lines** | `turn`, `poke`, `player` | Metadata about the game state: whose team is what, what the turn counter is, who the players are. |

BlisseyMeter fetches the `.log`, splits it on newlines, parses each line into a typed `Line` enum,
attaches sub-lines to the parent main line, and then dispatches main/info lines to handlers. Each
handler may emit zero or more `LuckEvent`s tagged with a category, a score, and the player they
belong to.

---

## Luck events

Every event has:

- `turn` — when it happened
- `pokemon` — which Pokemon it happened to (species parsed from the nickname, e.g.
  `"Poetic Justice (Dragapult)"` -> `Dragapult`)
- `category` — one of the categories below
- `score` — signed float (see *Scoring* below)
- `description` — human-readable summary
- `source_move` — the move that triggered it, if any
- `is_beneficial` — did this help (`true`) or hurt (`false`) the player it's attributed to

### Currently detected

| Category | Trigger | Score | Beneficial? |
|---|---|---|---|
| **CriticalHit** | `-crit` subline on a move | `+1.00` (fixed) | yes |
| **AccuracyMiss** | `-miss` subline on a move | `−1.5 * accuracy / 100` (so a  75% accurate move miss -> −1.125, the 1.5 is a weight) | no (counts against the player who missed) |
| **SecondaryEffect** — *didn't fire* | a move with a secondary-effect chance (e.g. Liquidation's 20% defense drop) is used and the proc doesn't happen | `−0.5 * chance / 100` | no |
| **SecondaryEffect** — *fired* | same move and the proc does happen | `+0.5 * (100 − chance) / 100` | yes |
| **SecondaryEffect** — *flinch didn't fire* | attacker used a flinch move (e.g. Air Slash 30%) and the target still moved | `−0.5 * flinch_chance / 100` | no (against the attacker) |
| **SecondaryEffect** — *flinch did fire* | opponent's flinch move successfully prevents the target's move | `+0.35` (fixed) | yes (for the attacker) |
| **Flinch** | `cant ... flinch` — the victim is forced to skip their move | `−0.15` (fixed) | no (against the victim) |
| **StatusTurn** — *moved through paralysis* | a paralyzed Pokemon gets to move anyway (the 25% full-para skip didn't happen) | `+0.25` (fixed) | yes |
| **StatusTurn** — *fully paralyzed* | `cant ... par` — the 25% skip triggered | `−0.75` (fixed) | no |
| **StatusTurn** — *woke up* | `-curestatus slp` after N sleep turns | `+(1/3)(2 − N)`, i.e. woke after 1 turn -> +0.333, after 2 -> 0.000, after 3 -> −0.333 | sign depends |

### Planned / reserved but not yet emitted

The `LuckCategory` enum already declares these variants, but no handler produces them yet:

- **DamageRoll** — low/high damage variance rolls (the 85%–100% multiplier).
- **AbilityProc** — random on-activation abilities (Static, Flame Body, Effect Spore, Cute Charm,
  Poison Point, etc) — the `-activate` sub-lines are already parsed, no scoring yet.
- **MultiHit** — RNG hit counts on Bullet Seed, Rock Blast, Triple Kick, etc
- **Other(String)** — catch-all for events not yet categorized.

Other areas untouched so far: item-triggered luck (Focus Sash, Sturdy), probably something else as well,
cant think off the top of my head
---

## Scoring

The conceptual scale runs from **−1.0** to **+1.0** per event:

- **+1.0** — really lucky and beneficial for user
- **0.0** — neutral / the most-likely outcome
- **−1.0** — really really really unlucky

In practice, a few categories overshoot the bounds today in either direction:

- miss has weight of `1.5`, so it can go from `-1.5` to `1.5`
- Secondary effects and flinches live in `−0.5 .. +0.5` because the `SECONDARY_EFFECT_WEIGHT = 0.5` multiplier bounds them.
- Status events live in `−0.75 - +0.25` (because of 25% para luck chance).

Weights are centralized in `backend/src/constants/luck_weights.rs` and easy to tune. There's no
global clamp and no per-game normalization — the sum across a player's events is the raw net luck.

The frontend ties this together visually:

- Events are shown as cards in two per-player columns, color-coded green (beneficial) or red
  (detrimental), each with a `+N` / `−N` badge.
- A cumulative luck graph plots each player's running total across the game (a dashed emerald line
  shows the net differential).
- Sort (by turn, impact magnitude, or direction), per-category and per-Pokemon filters, and a
  click-to-exclude toggle make it easy to slice the analysis. Disabled events drop out of the chart;
  the running total recomputes live.

---

## Running locally

### Backend

```bash
cd backend
cp .env.example .env   # PORT=8080, FRONTEND_URL=http://localhost:5173
cargo run
# POST http://127.0.0.1:8080/analyze with body = a replay .log URL
# example curl below
curl -X POST localhost:8080/analyze   -H "Content-Type: text/plain"   -d "https://replay.pokemonshowdown.com/gen9natdexdraft-2516579178.log" | jq

```

### Frontend

```bash
pnpm install  # from repo root (pnpm workspaces)
pnpm dev   # http://localhost:5173
```

The frontend reads its API base from `PUBLIC_API_URL` (`$env/static/public`), falling back to
`http://localhost:8080`. Override with a `frontend/.env`:

```
PUBLIC_API_URL=http://127.0.0.1:8080
```

---

## Credits

- Sprites & trainer avatars: [Pokemon Showdown](https://pokemonshowdown.com) (`play.pokemonshowdown.com/sprites/`).
- Move data: a bundled `move_details.csv` snapshot of Showdown's move database (accuracy, secondary
  effect chances, flinch flags). iirc its from pokemondb
