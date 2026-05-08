# Bevy 2D — Concepts Learned

A reference of the concepts covered while building this project. Ordered roughly by introduction.

---

## 1. ECS — the mental model

Bevy is data-oriented, not OOP. Stop thinking "objects with methods."

- **Entity** — just an ID. No data, no behavior.
- **Component** — plain data attached to an entity (`Transform`, `Velocity`, `Health`).
- **System** — a function that queries entities by component shape and acts on them.
- **Resource** — global singleton data (`Score`, `SpawnTimer`).
- **Message** — one-frame mailbox for cross-system signals (`EnemyKilled`).

Ask "is this data per-thing or one-of?" → per-thing = component, one-of = resource.

---

## 2. The `App` and plugins

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins((MovementPlugin, PlayerPlugin, EnemyPlugin))
    .run();
```

- `DefaultPlugins` gives you windowing, input, rendering, assets, audio.
- A **plugin** is a struct implementing `Plugin::build(&self, app: &mut App)`. Inside `build`, register components, resources, systems for that feature.
- `main.rs` becomes a *table of contents*: each plugin is a self-contained slice (components + systems + resources for one feature).

---

## 3. Components

```rust
#[derive(Component)]
pub struct Player;                   // marker component (zero-sized tag)

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);       // tuple struct holding data

#[derive(Component)]
pub struct Health(pub i32);
```

- **Marker components** carry no data — used in `With<...>` filters to identify entity roles.
- **Tuple structs** (newtypes) wrap a single value. Access with `.0`. Why use them instead of raw types? Distinct types let multiple resources/components coexist (`Velocity(Vec2)` and `Acceleration(Vec2)` are different types).
- Components must be `pub` (and their fields `pub`) to be constructed from other modules.

### Required components

In Bevy 0.15+, declaring `Sprite` or `Text` automatically pulls in everything they need (`Transform`, `Visibility`, `Node`, etc.) — you don't have to spawn them all manually.

---

## 4. Resources

```rust
#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource)]
struct SpawnTimer(Timer);
```

- Registered with `app.init_resource::<T>()` (uses `Default`) or `app.insert_resource(T(...))` (explicit value).
- Accessed in systems by parameter type: `Res<T>` (read), `ResMut<T>` (write).
- The **newtype pattern** is mandatory for distinct types — Bevy keys resources by type.

---

## 5. Systems

A system is a regular `fn` whose parameters are queries, resources, or message readers/writers.

```rust
fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q: Query<&mut Transform, With<Player>>,
) { ... }
```

- A system never runs unless **registered**: `.add_systems(Schedule, system)`.
- The parameter list declares what data it reads/writes — the scheduler uses this to run systems in parallel safely.

---

## 6. Queries

The data slot says *what to access*; the filter slot says *which entities qualify*.

```rust
Query<(&Transform, &mut Velocity), With<Enemy>>
//     ^^^^^^^^^^^^^^^^^^^^^^^^^   ^^^^^^^^^^^^
//     data: read Transform,       filter: only Enemies
//     mutate Velocity
```

### Filters
- `With<T>` — entity has component `T`.
- `Without<T>` — entity does not have `T`.
- `Or<(With<A>, With<B>)>` — has either.
- Tuples combine with AND: `(With<A>, Without<B>)`.

### Special data items
- `Entity` in the data tuple gives you the entity ID, useful for despawn: `Query<(Entity, &Transform), With<Enemy>>`.
- `Option<&mut T>` matches entities whether or not they have `T` — returns `Some(...)` or `None`. Useful when iterating heterogeneous entity sets.

### Generic vs specific systems

- **Generic system** (no marker filter): `Query<(&Velocity, &mut Transform)>` — moves *anything* with a `Velocity`. Reuses across entity types.
- **Specific system** (marker filter): `Query<&mut Velocity, With<Player>>` — only acts on players.

Rule: filter by the components your system actually needs, not by markers, unless you specifically want to distinguish entity types.

---

## 7. Schedules and ordering

```rust
.add_systems(Startup, setup)            // runs once at launch
.add_systems(Update, my_system)         // runs every frame
.add_systems(OnEnter(GameState::X), s)  // runs on state transition
```

- **`Startup`** — once at launch.
- **`Update`** — every frame (tied to render rate).
- **`FixedUpdate`** — fixed timestep, for deterministic physics.
- **`OnEnter(state)` / `OnExit(state)`** — state-transition hooks.

### Ordering inside a schedule

```rust
.add_systems(Update, (a, b, c).chain())          // a → b → c
.add_systems(Update, b.before(c))                // b before c
.add_systems(Update, b.after(a))                 // b after a
.add_systems(Update, system.run_if(condition))   // gated execution
```

- Systems with no ordering and no data conflict run in parallel.
- `.chain()` forces sequential execution within a tuple.
- `.before(other_system_fn)` / `.after(other_system_fn)` create explicit ordering.

---

## 8. The frame

A **frame** = one iteration of the main loop:
1. Poll OS events.
2. Run all `Update` systems.
3. Render.
4. Wait for next vsync.

**FPS** = frames per second. At 60 FPS, each frame = ~16.67ms. At 144 FPS, ~6.94ms.

Your `Update` systems run exactly once per frame. Faster machine = more frames = more system runs per second — which is why you scale by `delta_secs()`.

---

## 9. Time and motion

```rust
let dt = time.delta_secs();    // f32 seconds since last frame
transform.translation.x += velocity.x * dt;
```

- `time.delta()` → `Duration` (used for `Timer::tick`).
- `time.delta_secs()` → `f32` seconds (used for motion math).
- `time.elapsed_secs()` → total time since app start.

**Always multiply per-frame motion by `dt`** — otherwise speed depends on framerate.

### Direct vs velocity-based movement

- **Direct (instant)**: `position += direction * speed * dt`. Stops the moment input releases. Arcade feel.
- **Velocity-based**: maintain a `Velocity` component, accumulate from input as acceleration, decay with friction, integrate to position. Smooth, gliding, supports knockback/momentum/gravity.

Velocity decoupled from input is the foundation for any "physicsy" feel. Once you have a `Velocity` component, every effect (wind, gravity, knockback, springs) is just `vel += force * dt`.

### AABB clamp + reaction

When clamping position at a wall, also zero the velocity component pointing into the wall:

```rust
if p.x > half_w {
    p.x = half_w;
    vel.x = vel.x.min(0.0);  // kill outward, keep inward
}
```

Without this, velocity accumulates against the wall and the entity feels "stuck."

---

## 10. `Commands` — runtime spawn/despawn

```rust
fn my_system(mut commands: Commands) {
    let id = commands.spawn((Sprite { ... }, Transform::default(), MyMarker)).id();
    commands.entity(id).despawn();
}
```

- `Commands` is **deferred**: changes apply at the next sync point, not immediately.
- You can't query an entity in the same system you spawned it — split into two systems.
- `commands.spawn(bundle).id()` returns the new `Entity` for later use.
- `commands.entity(id).despawn()` removes the entity and all its components.

The deferred design is what lets many systems run in parallel — they record intentions, the scheduler applies them safely.

---

## 11. `Timer` and time-gated events

```rust
#[derive(Resource)]
struct SpawnTimer(Timer);

// init: Timer::from_seconds(1.0, TimerMode::Repeating)

fn spawn_system(time: Res<Time>, mut t: ResMut<SpawnTimer>) {
    t.0.tick(time.delta());
    if t.0.just_finished() {
        // do the thing
    }
}
```

- `tick(duration)` — advance the timer. Must be called every frame; timers don't progress on their own.
- `just_finished()` — true on the single frame the timer crossed its threshold. Use for "fire once."
- `finished()` — true on every frame after completion. Use for "stay in this state."
- `TimerMode::Once` vs `TimerMode::Repeating` — the latter auto-resets after firing.

This pattern (resource holding a Timer + tick + check) is universal for cooldowns, animations, intervals, debuffs.

---

## 12. Messages (formerly Events) — Bevy 0.18 rename

```rust
#[derive(Message)]
pub struct EnemyKilled;

// register
app.add_message::<EnemyKilled>();

// send
fn collide(mut writer: MessageWriter<EnemyKilled>) {
    writer.write(EnemyKilled);
}

// receive
fn react(mut reader: MessageReader<EnemyKilled>) {
    for msg in reader.read() { ... }
}
```

The mental model: a per-frame mailbox per type. Sender doesn't know about receivers; receivers don't know about senders. Multiple receivers are fine.

**When to use:** as soon as two or more systems care about the same event (collision → score + sound + particles). One sender + one receiver = could be a direct call; two+ = use messages.

> Naming note: this concept is called "Events" in Bevy ≤0.17 and most existing tutorials. Bevy 0.18 renamed `Event` → `Message`, `EventWriter` → `MessageWriter`, etc. Same mechanics, different names.

---

## 13. States

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

// register
app.init_state::<GameState>();
```

Three things states give you:

1. **Read current**: `Res<State<GameState>>`.
2. **Request transition**: `mut next: ResMut<NextState<GameState>>` → `next.set(GameState::GameOver)`. Applies between schedules, not instantly.
3. **Hooks and gating**:
   - `OnEnter(state)` / `OnExit(state)` schedules run once on transition.
   - `system.run_if(in_state(state))` gates a system to only run while in that state.

**Idiomatic uses:** `MainMenu` / `Playing` / `Paused` / `GameOver`. Gate gameplay systems with `run_if(in_state(Playing))` so the world freezes during menus.

---

## 14. UI text

UI text uses several components per entity (component-per-aspect, ECS style):

```rust
commands.spawn((
    Text::new("Score: 0"),
    TextFont { font_size: 28.0, ..default() },
    TextColor(Color::WHITE),
    Node {
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        left: Val::Px(10.0),
        ..default()
    },
    ScoreText,  // marker for finding it later
));
```

- `Text` — the string content (`Text(pub String)`, accessed via `.0`).
- `TextFont` — font handle and size.
- `TextColor` — color.
- `Node` — layout. `PositionType::Absolute` + `top`/`left` for fixed pixel placement.

For world-space text (follows a camera), use `Text2d` instead.

To update text later, add a marker component, then in a system:

```rust
fn update_score_text(score: Res<Score>, mut text: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() { return; }
    let Ok(mut t) = text.single_mut() else { return };
    t.0 = format!("Score: {}", score.0);
}
```

---

## 15. Change detection

```rust
if !score.is_changed() { return; }       // resource-level
Query<&T, Changed<T>>                     // component-level filter
Query<&T, Added<T>>                       // only entities that just got T
```

Bevy tracks per-frame change ticks. Use change detection to avoid running expensive logic every frame when the data didn't actually move.

---

## 16. Collision math (AABB)

Two axis-aligned rectangles overlap iff their projections overlap on **both** axes.

Using **half-extents** (half-width, half-height) makes the formula symmetric:

```rust
let delta    = a.position - b.position;
let combined = a.half_extent + b.half_extent;
if delta.x.abs() < combined.x && delta.y.abs() < combined.y {
    // overlap
}
```

Why half-extents:
- `|center_a - center_b| < half_a + half_b` is the 1D overlap test in one line.
- Symmetric: same data shape works at any position.
- Generalizes: a circle's "hitbox" is just `radius`, test becomes `delta.length() < r_a + r_b`.

For nested-query collision (player vs enemy):

```rust
for (p, p_h) in &players {
    for (e_id, e, e_h) in &enemies {
        // O(n*m) — fine until thousands of entities,
        // then move to spatial partitioning.
    }
}
```

---

## 17. Module organization

This project organizes by **feature**, not by layer:

```
src/
├── main.rs         — plugin composition only
├── game_state.rs   — GameState, restart, game-over UI
├── movement.rs     — Velocity primitive + apply_velocity
├── player.rs       — Player markers, input, movement, Health
├── enemy.rs        — Enemy marker, spawner, despawn
├── collision.rs    — Hitbox, EnemyKilled, detection
└── score.rs        — Score resource, score UI
```

Feature-based wins:
- A new feature → one new file with one plugin.
- Plugins as removable units (comment out one line to disable).
- Co-located components, systems, resources, constants.

Three signs of a wrong split:
- A file keeps growing without a clear theme. → Split by sub-feature.
- Two files always change together. → They're probably one feature.
- You're searching across files to understand one behavior. → Co-locate.

Cross-feature dependencies (e.g. collision needs to know about player and enemy markers) are fine in small projects. Larger projects use shared trait-style markers (`Friendly` / `Hostile`) to decouple.

---

## 18. Common Bevy idioms

### Early-return on optional values

```rust
let Ok(window) = windows.single() else { return };
```

`single()` returns a `Result` because there could be 0 or many windows. The let-else pattern is standard.

### Generic move system

```rust
fn apply_velocity(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    for (v, mut t) in &mut q {
        t.translation.x += v.0.x * time.delta_secs();
        t.translation.y += v.0.y * time.delta_secs();
    }
}
```

No marker filter → moves anything with a `Velocity`. Add `Velocity` to enemies and they move for free.

### Spawn-on-timer

```rust
timer.0.tick(time.delta());
if !timer.0.just_finished() { return; }
// spawn here
```

Universal pattern for periodic events.

### Restart pattern

On game-over → input → reset state:
- Despawn all enemies.
- Reset player `Transform`, `Velocity`, `Health`.
- Reset score.
- Transition `NextState` back to `Playing`.

---

## 19. What's next

Concepts seen but not yet used in this project:
- **Hot-reload assets** (turn it on for fast iteration).
- **`FixedUpdate`** for deterministic physics.
- **Audio** (`AudioPlayer`).
- **Sprite atlases** (`TextureAtlas`) for animation.
- **Tilemaps** (`bevy_ecs_tilemap`).
- **Physics** (Avian2D for idiomatic Bevy, Rapier2D as alternative).
- **System sets** for grouping and ordering many systems together.
- **Observers** (`commands.observe(...)`) — reactive systems triggered by component/event activity.

The pattern stays the same: a feature is a plugin, a plugin owns its slice, systems query by component shape.
