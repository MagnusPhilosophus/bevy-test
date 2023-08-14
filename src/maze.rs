use bevy::prelude::*;
use rand::Rng;

#[derive(Copy, Clone)]
enum Walls {
    North,
    South,
    West,
    East,
}

#[derive(Clone)]
struct Cell {
    visited: bool,
    walls: [bool; 4],
}

impl Cell {
    fn new() -> Self {
        Cell {
            visited: false,
            walls: [true, true, true, true],
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

    fn check_neighbors(&self, row: usize, col: usize) -> Vec<((usize, usize), (Walls, Walls))> {
        let mut result = Vec::new();
        let directions = [
            (-1, 0, Walls::South, Walls::North),
            (1, 0, Walls::North, Walls::South),
            (0, -1, Walls::West, Walls::East),
            (0, 1, Walls::East, Walls::West),
        ];

        for (d_row, d_col, wall_current, wall_neighbor) in &directions {
            let neighbor_row = row as isize + d_row;
            let neighbor_col = col as isize + d_col;
            if neighbor_row >= 0 && neighbor_col >= 0 {
                let neighbor_row = neighbor_row as usize;
                let neighbor_col = neighbor_col as usize;

                if neighbor_row < self.height && neighbor_col < self.width {
                    if !self.grid[neighbor_row][neighbor_col].visited {
                        result.push((
                            (neighbor_row, neighbor_col),
                            (wall_current.clone(), wall_neighbor.clone()),
                        ));
                    }
                }
            }
        }
        result
    }
}

fn create_grid(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let mut grid = Grid::new(16, 16);
    let current_cell = (0, 0);

    grid.grid[current_cell.0][current_cell.1].visited = true;
    grid.stack.push(current_cell);

    while !grid.stack.is_empty() {
        let current_cell = grid.stack.pop().unwrap();

        let neighbors = grid.check_neighbors(current_cell.0, current_cell.1);
        if !neighbors.is_empty() {
            grid.stack.push(current_cell);
            let (neighbor, wall) = neighbors[rng.gen_range(0..neighbors.len())];

            grid.grid[current_cell.0][current_cell.1].walls[wall.0 as usize] = false;
            grid.grid[neighbor.0][neighbor.1].walls[wall.1 as usize] = false;

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
    // X
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(10.0, 0.0, 0.0),
        ..default()
    });
    // Y
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
    // Z
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    });

    for (row_i, row) in grid.grid.iter().enumerate() {
        // Room
        // for (col_i, cell) in row.iter().enumerate() {
        //     commands.spawn(PbrBundle {
        //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        //         material: materials.add(if cell.visited {
        //             Color::rgb(1.0, 1.0, 1.0).into()
        //         } else {
        //             Color::rgb(0.0, 0.0, 0.0).into()
        //         }),
        //         transform: Transform::from_xyz(
        //             row_i as f32 * 1.1 + 0.5,
        //             0.0,
        //             col_i as f32 * 1.1 + 0.5,
        //         ),
        //         ..default()
        //     });
        // }
        // North
        for (col_i, cell) in row.iter().enumerate() {
            if cell.walls[Walls::North as usize] {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 1.1, 1.0))),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_xyz(
                        row_i as f32 * 1.1 + 1.05,
                        0.0,
                        col_i as f32 * 1.1 + 0.5,
                    ),
                    ..default()
                });
            }
        }
        // South
        for (col_i, cell) in row.iter().enumerate() {
            if cell.walls[Walls::South as usize] {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.1, 1.1, 1.0))),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_xyz(
                        row_i as f32 * 1.1 - 0.05,
                        0.0,
                        col_i as f32 * 1.1 + 0.5,
                    ),
                    ..default()
                });
            }
        }
        // West
        for (col_i, cell) in row.iter().enumerate() {
            if cell.walls[Walls::West as usize] {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.1, 0.1))),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_xyz(
                        row_i as f32 * 1.1 + 0.5,
                        0.0,
                        col_i as f32 * 1.1 - 0.05,
                    ),
                    ..default()
                });
            }
        }
        // East
        for (col_i, cell) in row.iter().enumerate() {
            if cell.walls[Walls::East as usize] {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.1, 0.1))),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_xyz(
                        row_i as f32 * 1.1 + 0.5,
                        0.0,
                        col_i as f32 * 1.1 + 1.05,
                    ),
                    ..default()
                });
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
