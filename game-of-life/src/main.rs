use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, Inspectable, WorldInspectorPlugin};

mod blocks;

use blocks::BlocksPlugin;

#[derive(Resource, Inspectable, Default)]
struct Data {
    should_render: bool,
    text: String,
    #[inspectable(min = 42.0, max = 100.0)]
    size: f32,
}

const LEFT_WALL: f32 = -550.;
const RIGHT_WALL: f32 = 250.;
const BOTTOM_WALL: f32 = -290.;
const TOP_WALL: f32 = 290.;
const WALL_THICKNESS: f32 = 5.;

const WALL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_plugin(BlocksPlugin)
        .run()

}


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

    //spwan walls
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Top));
    commands.spawn(Wall::new(WallLocation::Bottom));

}

