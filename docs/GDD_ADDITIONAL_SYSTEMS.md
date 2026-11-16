# Additional gameplay systems backlog

This backlog records the additional systems requested for Gaucho. It does not claim that these systems are implemented or that the server matches the original Conquer Mobile. Each item requires server logic, persistence, client/protocol integration, tests, and live-client verification before it can be marked complete.

## Priority and dependencies

### P0 — Correctness and authoritative gameplay

- [ ] **Battle Power (BP):** Define a versioned, data-driven BP formula from verified game data. Include level, rebirth, equipment, upgrades, titles/ranks, and progression systems only where their contribution is confirmed. Display BP consistently in attributes and player cards. Add tests for each term and backward-compatible recalculation for existing heroes.
- [ ] **BP combat differential:** Establish the exact damage/defense modifier curve from authorized reference behavior. Apply it in one shared combat calculation path for melee, magic, and monster attacks; cap extreme differences and test symmetry, boundaries, and zero/negative inputs. Do not invent a formula and call it original parity.
- [ ] **Monster aggro:** Represent aggressive/passive behavior explicitly in world data. Passive monsters only retaliate after a valid hit; aggressive monsters acquire the nearest eligible living player in configured vision. Retain a leash and return-home behavior after pursuit ends. Test target selection, death, map change, and leash boundaries.
- [ ] **Server-side movement/pathfinding:** Validate player movement and jumps against map bounds and movement limits. Implement navigation across known portals as server-approved steps; client auto-pathing is separate and needs client support. A quest marker alone is not proof of pathfinding.

### P1 — Progression, inventory, and economy

- [ ] **Perfection / Chi-Perfection:** Model per-equipment star level, materials, costs, caps, and stat bonuses. Validate upgrades atomically; preserve item identity and safely persist. Add visual/client packet support and tests for success, failure, max level, and concurrent requests.
- [ ] **Sub-classes:** Persist unlocked/selected specializations and progression. Define effects using data, validate eligibility, and avoid stacking or rebirth exploits. Client UI and packets are required.
- [ ] **CP / Bound CP mall:** Add separate integer-safe wallets and an authoritative catalog. Purchases must validate price, stock/eligibility, bag capacity, and currency, then commit wallet and item changes atomically with an audit trail. Payment integration is out of scope until explicitly designed.
- [ ] **Level-up packs and online rewards:** Configure one-time level milestones and online reward intervals in data. Persist claims and timestamps to prevent reconnect/restart duplication; test skipped levels, repeated claims, and clock behavior.
- [ ] **Achievements and permanent titles:** Track server-verified progress/events, completion, points, and reward claims. Persist achievements and title ownership; never trust a client-supplied kill/win count.

### P2 — World, competition, and social play

- [ ] **Random rare spawns / treasure thieves:** Add configurable spawn schedules/chances, map-safe spawn locations, unique encounter IDs, cleanup/respawn, eligibility, and one-time rewards. Test concurrent last-hit/reward races and server restart recovery.
- [ ] **Friends and enemy list:** Persist friend/enemy relationships and expose online/map presence only according to privacy and map rules. Record a valid PvP killer on death, handle disconnects and name changes, and never permit tracking of hidden or inaccessible players.
- [ ] **Auto-pathfinding:** The server should provide validated route/portal data and safe interaction targets; actual continuous movement automation belongs to the mobile client. Enforce movement validation server-side regardless of client automation.

## Acceptance criteria common to all items

1. Data is validated at load; malformed or missing optional values fail safely and are logged.
2. State-changing requests are authorized, bounded, and idempotent where retries are possible.
3. Persistence survives reconnect and restart without duplicate rewards or lost currency/items.
4. Unit/integration tests cover success, invalid input, boundary values, and repeated requests.
5. The supported mobile client is tested end-to-end before a feature is marked complete.
6. Official-game behavior is claimed only after comparison with authorized reference data/client behavior; otherwise document it as Gaucho-specific.

## Current status

All entries above are planned/unverified from the source inspected for this backlog. The existing server has a basic BP estimate, basic monster AI/aggro behavior, friend records, titles, mounts, guilds, and shop/trade foundations, but those are not equivalent to the full systems described here. The project still requires build/test execution and client integration testing.
