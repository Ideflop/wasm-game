use bevy::prelude::*;

const BLOCK_SIZE: Vec2 = Vec2::new(15.,15.);
const GAB_BETWEEN_BLOCK: f32 = 3.;

const LEFT_WALL: f32 = -550.;
const RIGHT_WALL: f32 = 250.;
const BOTTOM_WALL: f32 = -290.;
const TOP_WALL: f32 = 290.;
const WALL_THICKNESS: f32 = 5.;

const BLOCK_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const BACKGROUND_COLOR: Color = Color::rgb(1., 1., 1.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Blocks;

#[derive(Bundle)]
struct Wall {
    sprite_bundle : SpriteBundle,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {

    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL,0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL,0.),
            WallLocation::Bottom => Vec2::new(0.,BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0.,TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {

        let arena_width = RIGHT_WALL -  LEFT_WALL + TOP_WALL;
        let arena_height = TOP_WALL - BOTTOM_WALL;
        
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(arena_width + WALL_THICKNESS , WALL_THICKNESS)
            }
        }

    }
}

impl Wall {
    fn new(location: WallLocation) -> Wall {
        Wall {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0,),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Top));
    commands.spawn(Wall::new(WallLocation::Bottom));


    // compute how many blocks can be fit
    let total_block_width =  (RIGHT_WALL -  LEFT_WALL) - 2. * GAB_BETWEEN_BLOCK ;
    let total_block_height = (TOP_WALL - BOTTOM_WALL) - 2. * GAB_BETWEEN_BLOCK ;

    let x_columms = ((total_block_width / (BLOCK_SIZE.x + GAB_BETWEEN_BLOCK) ).floor() - 1.) as usize;
    let x_rows = (total_block_height / (BLOCK_SIZE.y + GAB_BETWEEN_BLOCK) ).floor() as usize;

    for row in 0..x_rows {
        for col in 0..x_columms {
            let brick_position = Vec2::new(
                col as f32 * (BLOCK_SIZE.x + GAB_BETWEEN_BLOCK) + LEFT_WALL + 22.,
                row as f32 * (BLOCK_SIZE.y + GAB_BETWEEN_BLOCK) + BOTTOM_WALL + 20.,
                );

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: BLOCK_COLOR, 
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.),
                        scale: Vec3::new(BLOCK_SIZE.x, BLOCK_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Blocks,
            ));
        }
    }
}

