use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, asset::AssetMetaCheck};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
                // Wasm builds check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
        }))
        .add_systems(Startup, (setup))
        .add_systems(Update, (game_cycles))
        .run();
}

#[derive(Component)]
struct SunMoon;

// basic rendering initialization
fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    // camera/viewport
    commands.spawn(Camera2dBundle::default());
    //spawn sun moon
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
            material: materials.add(Color::srgb(255.0, 218.0, 124.0)),
            ..default()
        },
        // TODO: replace with sun moon sprites/textures
/*        SpriteBundle {
            texture: todo!(),
            sprite: todo!(),
            transform: todo!(),
            global_transform: todo!(),
            visibility: todo!(),
            inherited_visibility: todo!(),
            view_visibility: todo!(),
        },
*/
        SunMoon
    ));
}


// day night cycle & duration
fn game_cycles () {

    println!("time of day")
}