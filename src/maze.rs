use bevy::prelude::*;

enum Walls {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Cell {
    walls: [bool; 4],
}

impl Cell {
    fn new() -> Self {
        Cell {
            walls: [true, false, false, false],
        }
    }
}

#[derive(Component)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
    stack: Vec<(usize, usize)>,
    visited_cells: usize,
    path_width: usize,
}

impl Grid {
    fn new(width: usize, height: usize, path_width: usize) -> Self {
        Grid {
            width,
            height,
            grid: vec![vec![Cell::new(); width]; height],
            stack: vec![],
            visited_cells: 0,
            path_width,
        }
    }
}

fn create_grid(mut commands: Commands) {
    let mut grid = Grid::new(4, 4, 3);
    commands.spawn(grid);
}

fn display_grid(
    grid: Query<&Grid>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let grid = grid.single();

    for (row_i, row) in grid.grid.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                transform: Transform::from_xyz(row_i as f32 * 1.1, 0.0, col_i as f32 * 1.1),
                ..default()
            });
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
