use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

#[derive(Clone)]
struct Cell {
    visited: bool,
    walls: [bool; 2], // North, East
}

impl Cell {
    fn new() -> Self {
        Cell {
            visited: false,
            walls: [true, true], // North, East
        }
    }
}

#[derive(Component)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
    stack: Vec<(usize, usize)>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            grid: vec![vec![Cell::new(); width]; height],
            stack: Vec::new(),
        }
    }

    fn check_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (d_row, d_col) in &directions {
            let neighbor_row = row as isize + d_row;
            let neighbor_col = col as isize + d_col;
            if neighbor_row >= 0 && neighbor_col >= 0 {
                let neighbor_row = neighbor_row as usize;
                let neighbor_col = neighbor_col as usize;

                if neighbor_row < self.height && neighbor_col < self.width {
                    if !self.grid[neighbor_row][neighbor_col].visited {
                        result.push((neighbor_row, neighbor_col));
                    }
                }
            }
        }
        result
    }
}

fn create_grid(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let mut grid = Grid::new(10, 10);
    let current_cell = (0, 0);

    grid.grid[current_cell.0][current_cell.1].visited = true;
    grid.stack.push(current_cell);

    while !grid.stack.is_empty() {
        let current_cell = grid.stack.pop().unwrap();

        let neighbors = grid.check_neighbors(current_cell.0, current_cell.1);
        if !neighbors.is_empty() {
            grid.stack.push(current_cell);
            let neighbor = neighbors[rng.gen_range(0..neighbors.len())];

            match (
                neighbor.0 as isize - current_cell.0 as isize,
                neighbor.1 as isize - current_cell.1 as isize,
            ) {
                (-1, 0) => grid.grid[neighbor.0][neighbor.1].walls[0] = false, // South
                (1, 0) => grid.grid[current_cell.0][current_cell.1].walls[0] = false, // North
                (0, -1) => grid.grid[neighbor.0][neighbor.1].walls[1] = false, // West
                (0, 1) => grid.grid[current_cell.0][current_cell.1].walls[1] = false, // East
                _ => (),
            }

            grid.grid[neighbor.0][neighbor.1].visited = true;
            grid.stack.push(neighbor);
        }
    }

    commands.spawn(grid);
}

fn display_grid(
    grid: Query<&Grid>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid = grid.single();
    let room_size = 2.0;
    let wall_width = 0.1;
    let wall_height = 1.5;
    let wall_depth = room_size + wall_width * 2.0;

    // Floor
    let maze_width = (room_size + wall_width) * grid.width as f32 + wall_width;
    let maze_height = (room_size + wall_width) * grid.height as f32 + wall_width;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                maze_width,
                wall_width,
                maze_height,
            ))),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(
                maze_width / 2.0 - wall_width,
                -wall_width / 2.0,
                maze_height / 2.0 - wall_width,
            ),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(maze_width / 2.0, wall_width / 2.0, maze_height / 2.0),
    ));

    // South
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                maze_width,
                wall_height,
                wall_width,
            ))),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(
                maze_width / 2.0 - wall_width,
                wall_height / 2.0,
                -wall_width / 2.0,
            ),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(maze_width / 2.0, wall_height / 2.0, wall_width / 2.0),
    ));

    // West
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                wall_width,
                wall_height,
                maze_height,
            ))),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(
                -wall_width / 2.0,
                wall_height / 2.0,
                maze_height / 2.0 - wall_width,
            ),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(wall_width / 2.0, wall_height / 2.0, maze_height / 2.0),
    ));

    // Finish room sensor collider
    commands.spawn((
        Sensor,
        TransformBundle::from(Transform::from_xyz(
            (grid.width - 1) as f32 * (room_size + wall_width) + room_size / 2.0,
            wall_height / 2.0,
            (grid.height - 1) as f32 * (room_size + wall_width) + room_size / 2.0,
        )),
        Collider::cuboid(room_size / 2.0, wall_height / 2.0, room_size / 2.0),
    ));

    for (row_i, row) in grid.grid.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            // Room
            // commands.spawn(PbrBundle {
            //     mesh: meshes.add(Mesh::from(shape::Cube { size: room_size })),
            //     material: materials.add(if cell.visited {
            //         Color::rgb(1.0, 1.0, 1.0).into()
            //     } else {
            //         Color::rgb(0.0, 0.0, 0.0).into()
            //     }),
            //     transform: Transform::from_xyz(
            //         col_i as f32 * (room_size + wall_width) + room_size / 2.0,
            //         wall_height / 2.0,
            //         row_i as f32 * (room_size + wall_width) + room_size / 2.0,
            //     ),
            //     ..default()
            // });
            // North
            if cell.walls[1] {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(
                            wall_width,
                            wall_height,
                            wall_depth,
                        ))),
                        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                        transform: Transform::from_xyz(
                            col_i as f32 * (room_size + wall_width) + room_size + wall_width / 2.0,
                            wall_height / 2.0,
                            row_i as f32 * (room_size + wall_width) + (room_size / 2.0),
                        ),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(wall_width / 2.0, wall_height / 2.0, wall_depth / 2.0),
                ));
            }
            // East
            if cell.walls[0] {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(
                            wall_depth,
                            wall_height,
                            wall_width,
                        ))),
                        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                        transform: Transform::from_xyz(
                            col_i as f32 * (room_size + wall_width) + (room_size / 2.0),
                            wall_height / 2.0,
                            row_i as f32 * (room_size + wall_width) + room_size + wall_width / 2.0,
                        ),
                        ..default()
                    },
                    RigidBody::Fixed,
                    Collider::cuboid(wall_depth / 2.0, wall_height / 2.0, wall_width / 2.0),
                ));
            }
        }
    }
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, create_grid)
            .add_systems(Startup, display_grid);
    }
}
