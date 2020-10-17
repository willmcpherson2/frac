use quicksilver::Timer;
use std::mem;
use std::time::Duration;

const CELL_SIZE_I: usize = 3;
const CELL_SIZE_F: f32 = 3.0;
const GRID_SIZE_I: usize = 81;
const GRID_SIZE_F: f32 = 81.0;

pub struct State {
    timer: Timer,
    cell: Vec<Vec<bool>>,
    grid: Vec<Vec<bool>>,
    new_grid: Vec<Vec<bool>>,
    stage: Stage,
    strategy: Strategy,
    running: bool,
    step: bool,
}

enum Stage {
    Cell,
    Grid,
}

enum Strategy {
    Assign,
    Or,
}

impl State {
    pub fn new() -> Self {
        State {
            timer: Timer::with_duration(Duration::from_millis(1000)),
            cell: vec![vec![false; CELL_SIZE_I]; CELL_SIZE_I],
            grid: vec![vec![false; GRID_SIZE_I]; GRID_SIZE_I],
            new_grid: vec![vec![false; GRID_SIZE_I]; GRID_SIZE_I],
            stage: Stage::Cell,
            running: true,
            step: false,
            strategy: Strategy::Or,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        match self.stage {
            Stage::Cell => self.cell[x][y],
            Stage::Grid => self.grid[x][y],
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut bool {
        match self.stage {
            Stage::Cell => &mut self.cell[x][y],
            Stage::Grid => &mut self.grid[x][y],
        }
    }

    pub fn size_i(&self) -> usize {
        match self.stage {
            Stage::Cell => CELL_SIZE_I,
            Stage::Grid => GRID_SIZE_I,
        }
    }

    pub fn size_f(&self) -> f32 {
        match self.stage {
            Stage::Cell => CELL_SIZE_F,
            Stage::Grid => GRID_SIZE_F,
        }
    }

    pub fn next_stage(&mut self) {
        match self.stage {
            Stage::Cell => self.stage = Stage::Grid,
            Stage::Grid => self.stage = Stage::Cell,
        }
    }

    pub fn toggle_running(&mut self) {
        self.running = !self.running
    }

    pub fn step_mut(&mut self) -> &mut bool {
        &mut self.step
    }

    pub fn clear(&mut self) {
        fn clear_grid(vec: &mut Vec<Vec<bool>>) {
            for row in vec {
                for e in row {
                    *e = false;
                }
            }
        }
        clear_grid(&mut self.grid);
        clear_grid(&mut self.new_grid);
    }

    pub fn next_strategy(&mut self) {
        match self.strategy {
            Strategy::Assign => self.strategy = Strategy::Or,
            Strategy::Or => self.strategy = Strategy::Assign,
        }
    }

    pub fn update(&mut self) {
        fn translate_cell_grid(grid: usize, cell: usize) -> usize {
            if grid == 0 && cell == 0 {
                GRID_SIZE_I - 1
            } else {
                (grid + cell - 1) % GRID_SIZE_I
            }
        }

        if !self.step && (!self.timer.tick() || !self.running) {
            return;
        }

        for grid_x in 0..GRID_SIZE_I {
            for grid_y in 0..GRID_SIZE_I {
                if !self.grid[grid_x][grid_y] {
                    continue;
                }

                for cell_x in 0..CELL_SIZE_I {
                    for cell_y in 0..CELL_SIZE_I {
                        let x = translate_cell_grid(grid_x, cell_x);
                        let y = translate_cell_grid(grid_y, cell_y);
                        if let Strategy::Assign = self.strategy {
                            self.new_grid[x][y] = self.cell[cell_x][cell_y];
                        } else {
                            self.new_grid[x][y] |= self.cell[cell_x][cell_y];
                        }
                    }
                }
            }
        }

        mem::swap(&mut self.grid, &mut self.new_grid);

        *self.step_mut() = false;
    }
}
