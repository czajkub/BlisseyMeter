#### p+n - player: nick
#### pokemon - species, gender (if applicable)


# main lines

switch:         p+n | pokemon | hp
move:           p+n | move    | target (p+n)
faint:          p+n
detailschange:  p+n | pokemon



# sublines

-damage:    hit pokemon (p+n) | new hp
-heal: targeted pokemon (p+n) | new hp

-crit:            target of move (p+n)
-resisted:        target of move (p+n)
-supereffective:  target of move (p+n)
-immune:          target of move (p+n)

-unboost: target | stat | amount(1-6)
-boost:   target | stat | amount(1-6)

-enditem:  p+n | item | [from] | [of] (referencing [from])

-activate: p+n | ability (only?)

-terastallize: p+n | type
-mega:         p+n | pokemon | item



# info
start: null
upkeep: null
t: timestamp
gen: number (1-9)
gametype: singles / doubles(?)
tier: nazwa
rule: (rulename): desc
turn: number

win: nick
j: nick
c: nick | message

player:    `p1`/`p2` | nick | avatar
teamsize:  `p1`/`p2` | number (1-6)
poke:      `p1`/`p2` | pokemon 
