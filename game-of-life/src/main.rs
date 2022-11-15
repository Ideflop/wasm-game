use bevy::prelude::*;

const BLOCK_SIZE: Vec2 = Vec2::new(15.,15.);
//const GAB_BETWEEN_BLOCK: f32 = 1.;

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

    let brick_position= Vec2::new(-540.,-280.);

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

