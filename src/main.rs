use bevy::{ecs::query, math::DVec3, prelude::*, sprite::MaterialMesh2dBundle};
mod sol;
use sol::{default_system, CelestialBody, SolarSystem};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Systems to run at startup
        .add_systems(Startup, setup)
        // .add_systems(Startup, create_system)
        .add_systems(Startup, big_bang)
        // Systems to run every frame
        .add_systems(Update, update_positions)
        .add_systems(Update, render_bodies)
        // .add_systems(Update, calculate_collisions)
        .run();

    // let mut world = World::new();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //     transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
    //     ..default()
    // });
}

// fn create_system(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let system = sol::SolarSystem::new(Some(default_system()));
//     commands.spawn(system);
//     info!("BANG");
// }

fn big_bang(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // mut query: Query<&mut SolarSystem>,
) {
    let system = sol::SolarSystem::new(Some(default_system()));

    info!("BANG {:?}", system);
    // realisation that maybe the SolarSystem might not need to exist and can be merged with this system code
    // let system = query.single_mut();
    // if query = query.single_mut() {
    //     info!("BANG");
    //     info!("System: {:?}", system);
    // }
    system.objects.iter().for_each(|body| {
        // info!("BANG-Body: {:?}", body);

        // ISSUE IS HERE
        // Body clones at runtime are not the same when accessed via the system object
        commands.spawn(body);
    });

    commands.spawn(system);
}

fn render_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // query: Query<(&CelestialBody, &Transform)>,
    query: Query<(&CelestialBody)>,
) {
    // info!("BODY QUERY");
    for body in &query {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(body.radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(body.pos.x, body.pos.y, 0.)),
            ..default()
        });
    }
}

fn update_positions(time: Res<Time>, mut query: Query<(&mut CelestialBody, &mut Transform)>) {

    let bodies = query.iter().map(|(body, _)| body).collect::<Vec<&CelestialBody>>();

    SolarSystem::calculate_new_positions(&mut bodies);
    // if
    for  (body, transform) in query.iter_mut() {
        
        // info!("Transform: {:?}", transform);
        // transform.translation.x += time.delta_seconds() * 2.0;
        // transform.translation.y = time.seconds_since_startup() as f32 * 100.0;
    }
}

