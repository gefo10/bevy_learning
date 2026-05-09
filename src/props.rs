use crate::game_state::GameState;
use crate::player::KineticPlayer;
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashSet;

const CHUNK_SIZE: f32 = 400.0;
const PROPS_PER_CHUNK: usize = 12;
const SPAWN_RADIUS: i32 = 3;
const DESPAWN_RADIUS: i32 = 5;

#[derive(Component)]
struct Prop {
    chunk: (i32, i32),
}

#[derive(Resource, Default)]
struct SpawnedChunks(HashSet<(i32, i32)>);

pub struct PropsPlugin;

impl Plugin for PropsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnedChunks>()
            .add_systems(Update, manage_chunks.run_if(in_state(GameState::Playing)));
    }
}

fn world_to_chunk(x: f32, z: f32) -> (i32, i32) {
    ((x / CHUNK_SIZE).floor() as i32, (z / CHUNK_SIZE).floor() as i32)
}

fn manage_chunks(
    mut commands: Commands,
    mut spawned: ResMut<SpawnedChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_q: Query<&Transform, With<KineticPlayer>>,
    props: Query<(Entity, &Prop)>,
) {
    let Ok(player) = player_q.single() else { return };
    let (cx, cz) = world_to_chunk(player.translation.x, player.translation.z);

    // Despawn props in chunks that are too far, remove from spawned set
    let mut chunks_to_remove: HashSet<(i32, i32)> = HashSet::new();
    for (entity, prop) in &props {
        if (prop.chunk.0 - cx).abs() > DESPAWN_RADIUS
            || (prop.chunk.1 - cz).abs() > DESPAWN_RADIUS
        {
            commands.entity(entity).despawn();
            chunks_to_remove.insert(prop.chunk);
        }
    }
    for chunk in chunks_to_remove {
        spawned.0.remove(&chunk);
    }

    // Spawn props for nearby chunks not yet spawned
    for dx in -SPAWN_RADIUS..=SPAWN_RADIUS {
        for dz in -SPAWN_RADIUS..=SPAWN_RADIUS {
            let chunk = (cx + dx, cz + dz);
            if spawned.0.contains(&chunk) {
                continue;
            }
            spawned.0.insert(chunk);
            spawn_chunk_props(&mut commands, &mut meshes, &mut materials, chunk);
        }
    }
}

fn spawn_chunk_props(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    chunk: (i32, i32),
) {
    let mut rng = rand::rng();
    let ox = chunk.0 as f32 * CHUNK_SIZE;
    let oz = chunk.1 as f32 * CHUNK_SIZE;

    for _ in 0..PROPS_PER_CHUNK {
        let x = ox + rng.random_range(0.0..CHUNK_SIZE);
        let z = oz + rng.random_range(0.0..CHUNK_SIZE);

        // Keep spawn area clear near world origin
        if chunk == (0, 0) && Vec2::new(x, z).length() < 160.0 {
            continue;
        }

        match rng.random_range(0u8..3) {
            0 => spawn_rock(commands, meshes, materials, Vec3::new(x, 0.0, z), &mut rng, chunk),
            1 => spawn_stump(commands, meshes, materials, Vec3::new(x, 0.0, z), &mut rng, chunk),
            _ => spawn_bush(commands, meshes, materials, Vec3::new(x, 0.0, z), &mut rng, chunk),
        }
    }
}

fn spawn_rock(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rng: &mut impl Rng,
    chunk: (i32, i32),
) {
    let w = rng.random_range(20.0_f32..55.0);
    let h = rng.random_range(10.0_f32..28.0);
    let d = rng.random_range(15.0_f32..45.0);
    let grey = rng.random_range(0.28_f32..0.58);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(w, h, d))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(grey, grey, grey + 0.04),
            ..default()
        })),
        Transform::from_translation(pos + Vec3::Y * h * 0.5).with_rotation(
            Quat::from_rotation_y(rng.random_range(0.0_f32..std::f32::consts::TAU)),
        ),
        Prop { chunk },
    ));
}

fn spawn_stump(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rng: &mut impl Rng,
    chunk: (i32, i32),
) {
    let r = rng.random_range(7.0_f32..15.0);
    let h = rng.random_range(25.0_f32..60.0);
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(r, h))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.34, 0.21, 0.11),
            ..default()
        })),
        Transform::from_translation(pos + Vec3::Y * h * 0.5),
        Prop { chunk },
    ));
}

fn spawn_bush(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
    rng: &mut impl Rng,
    chunk: (i32, i32),
) {
    let r = rng.random_range(12.0_f32..30.0);
    let green = rng.random_range(0.28_f32..0.55);
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(r))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, green, 0.08),
            ..default()
        })),
        Transform::from_translation(pos + Vec3::Y * r * 0.65),
        Prop { chunk },
    ));
}
