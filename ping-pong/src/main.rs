use bevy::prelude::*;

fn main() {
    // initialize the Bevy game engine
    App::build()
        .add_resource(Msaa::default())
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(ball_movement_system.system())
        .add_system(paddle_movement_system.system())
        .add_system(collision_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // load the ping pong ball image
    let ball_texture_handle = asset_server.load("assets/ball.png").unwrap();

    // create a material for the ball using the ball texture
    let ball_material = materials.add(ColorMaterial {
        albedo: ball_texture_handle,
        ..ColorMaterial::default()
    });

    // create the ball entity
    commands
        .spawn(SpriteComponents {
            material: ball_material,
            translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
            scale: Scale(0.1),
            ..Default::default()
        })
        .with(Ball {
            velocity: Vec2::new(1.0, 1.0),
        });

    // create the left and right paddles
    let paddle_width = 0.2;
    let paddle_height = 1.0;

    let left_paddle_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    commands
        .spawn(SpriteComponents {
            material: left_paddle_material,
            translation: Translation(Vec3::new(-1.0, 0.0, 0.0)),
            scale: Scale(Vec3::new(paddle_width, paddle_height, 1.0)),
            ..Default::default()
        })
        .with(Paddle {
            side: Side::Left,
            height: paddle_height,
        });

    let right_paddle_material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    commands
        .spawn(SpriteComponents {
            material: right_paddle_material,
            translation: Translation(Vec3::new(1.0, 0.0, 0.0)),
            scale: Scale(Vec3::new(paddle_width, paddle_height, 1.0)),
            ..Default::default()
        })
        .with(Paddle {
            side: Side::Right,
            height: paddle_height,
        });
}

fn ball_movement_system(time: Res<Time>, mut ball_query: Query<(&Ball, &mut Translation)>) {
    // update the position of the ball based on its velocity
    for (ball, mut translation) in &mut ball_query.iter() {
        let dt = time.delta_seconds;
        translation

