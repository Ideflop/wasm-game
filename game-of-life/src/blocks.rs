use bevy::prelude::*;
use rand::{Rng,thread_rng};

const BLOCK_SIZE: Vec2 = Vec2::new(15.,15.);
const GAB_BETWEEN_BLOCK: f32 = 3.;

const LEFT_WALL: f32 = -550.;
const RIGHT_WALL: f32 = 250.;
const BOTTOM_WALL: f32 = -290.;
const TOP_WALL: f32 = 290.;

const BLOCK_COLOR_WHITE: Color = Color::rgb(1., 1., 1.);
const BLOCK_COLOR_BLACK: Color = Color::rgb(0., 0., 0.);

const TIME_STEP: f32 = 4.0;

pub struct BlocksPlugin;

#[derive(Resource)]
struct ChangeColorTimer(Timer);

#[derive(Bundle)]
struct Blocks {
    sprite_bundle : SpriteBundle,
    universe : Universe,
}

#[repr(u8)]
#[derive(Clone, Copy, Component)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Component)]
struct Width(u32);

#[derive(Component)]
struct Height(u32);

#[derive(Component)]
struct Number(u32);

#[derive(Component, Clone)]
struct Cells(Vec<Cell>);

#[derive(Component)]
pub struct Universe {
    width: Width,
    height: Height,
    number: Number,
    cells: Cells,
}

impl Universe {

    fn get_index(&self, row : u32, column : u32) -> usize {
        (row * self.width.0 + column) as usize
    }

    fn count_neighbor(&self, row : u32, column : u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height.0 - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width.0 - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_column == 0 {
                    continue
                }

                let neighbor_row = (row + delta_row) % self.height.0;
                let neighbor_column = (column + delta_column) % self.width.0;
                let idx = self.get_index(neighbor_row, neighbor_column);
                count += self.cells.0[idx] as u8;
            }
        }
        count
    }

    pub fn update(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height.0 {
            for col in 0..self.width.0 {
                let idx = self.get_index(row, col);
                let cell = self.cells.0[idx];
                let live_neighbors = self.count_neighbor(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next.0[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    //fn change_colors(
    //    &mut self,
    //    time: Res<Time>, mut timer: ResMut<ChangeColorTimer>,
    //    mut query_sprite: Query<(&mut Universe, &mut Sprite)>,
    //    ) {
    //    if timer.0.tick(time.delta()).just_finished() {
    //        self.update();

    //        // Create a query to find the sprite with the same universe number as self.
    //        let query_same_universe = query_sprite
    //            .filter(|(u, _)| u.number.0 == self.number.0);

    //        // Iterate over the query and change the color of the sprite
    //        for (_, mut sprite) in query_same_universe.iter_mut() {
    //            sprite.color = BLOCK_COLOR_BLACK;
    //        }
    //    }

    //fn change_colors(
    //    &mut self,
    //    time: Res<Time>, mut timer: ResMut<ChangeColorTimer>, 
    //    mut query_sprite: Query<(&mut Universe, &mut Sprite)>,
    //    ) {
    //    if timer.0.tick(time.delta()).just_finished() {
    //        self.update();
    //        let mut number = 0;
    //        for line in self.cells.0.as_slice().chunks(self.width.0 as usize) {
    //            for &cell in line {
    //                // add here
    //            }
    //        }
    //    }
    //     

    //            //for mut sprite in query_sprite.iter_mut() {
    //            //    let color = sprite.color.r();
    //            //    if !(color == 0.5) {
    //            //        if color == 1. {
    //            //            sprite.color.set_r(1.0);
    //            //            sprite.color.set_g(1.0);
    //            //            sprite.color.set_b(1.0);
    //            //            i += 1;
    //            //        } else if color == 0. {
    //            //            sprite.color.set_r(0.0);
    //            //            sprite.color.set_g(0.0);
    //            //            sprite.color.set_b(0.0);
    //            //            i += 1;
    //            //        }
    //            //    }
    //            //}
    //}
}

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChangeColorTimer(Timer::from_seconds(TIME_STEP, TimerMode::Repeating)))
            .add_startup_system(Block::spawn_blocks);
            //.add_system(function); // add the function here
    }
}

pub struct Block;

impl Block {

    fn rand_color() -> Color {
        let rng = thread_rng().gen_range(0..2);
        let color = match rng {
            0 => BLOCK_COLOR_WHITE,
            _ => BLOCK_COLOR_BLACK,
            
        };
        color
    }

    fn spawn_blocks(mut commands : Commands) {
        
        // compute how many blocks can be fit
        let total_block_width =  (RIGHT_WALL -  LEFT_WALL) - 2. * GAB_BETWEEN_BLOCK ;
        let total_block_height = (TOP_WALL - BOTTOM_WALL) - 2. * GAB_BETWEEN_BLOCK ;

        let x_columms = ((total_block_width / (BLOCK_SIZE.x + GAB_BETWEEN_BLOCK) ).floor() - 1.) as usize;
        let x_rows = (total_block_height / (BLOCK_SIZE.y + GAB_BETWEEN_BLOCK) ).floor() as usize;

        //spawn blocks
        let mut number = 0;
        for row in 0..x_rows {
            for col in 0..x_columms {
                let brick_position = Vec2::new(
                    col as f32 * (BLOCK_SIZE.x + GAB_BETWEEN_BLOCK) + LEFT_WALL + 22.,
                    row as f32 * (BLOCK_SIZE.y + GAB_BETWEEN_BLOCK) + BOTTOM_WALL + 20.,
                    );

                let rand_color = Block::rand_color();
                let cell_dead_live =  if rand_color == BLOCK_COLOR_WHITE {
                    Cell::Dead
                } else {
                    Cell::Alive
                };

                commands.spawn(Blocks {
                    sprite_bundle :  SpriteBundle {
                        sprite: Sprite {
                            color: rand_color,
                            ..default()
                        },
                        transform: Transform {
                            translation: brick_position.extend(0.),
                            scale: Vec3::new(BLOCK_SIZE.x, BLOCK_SIZE.y, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    universe : Universe {
                        width: Width(x_columms as u32),
                        height: Height(x_rows as u32),
                        number: Number(number),
                        cells: Cells(vec![cell_dead_live; x_columms * x_rows]),
                    }
                });
                number += 1;
            }
        }
    }
}
