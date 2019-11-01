use std::io::{stdout, Write};

use crossterm::{cursor::MoveTo, queue, style::Colorize, style::PrintStyledContent, Result};
use rand::{
    self,
    distributions::{IndependentSample, Range},
};

use super::snake::Snake;
use super::types::Position;

/// A food.
struct Food {
    /// The food position.
    position: Position,
}

impl Food {
    /// Creates a new food with the given `position`.
    fn new(position: Position) -> Self {
        Food { position }
    }

    /// Draws the food.
    fn draw(&self) -> Result<()> {
        queue!(
            stdout(),
            MoveTo(self.position.x, self.position.y),
            PrintStyledContent("❖".green())
        )
    }
}

/// A world map.
pub struct Map {
    /// The map width.
    width: u16,
    /// The map height.
    height: u16,
    /// Food.
    food: Option<Food>,
}

impl Map {
    /// Crates a new map with the given `width` & `height`.
    pub fn new(width: u16, height: u16) -> Self {
        Map {
            width,
            height,
            food: None,
        }
    }

    /// Draws the map border.
    pub fn draw_border(&self) -> Result<()> {
        for y in 0..self.height {
            queue!(
                stdout(),
                MoveTo(0, y),
                PrintStyledContent("█".magenta()),
                MoveTo(self.width - 1, y),
                PrintStyledContent("█".magenta())
            )?;
        }
        for x in 0..self.width {
            queue!(
                stdout(),
                MoveTo(x, 0),
                PrintStyledContent("█".magenta()),
                MoveTo(x, self.height - 1),
                PrintStyledContent("█".magenta())
            )?;
        }
        Ok(())
    }

    /// Check if the given `position` is out of bounds.
    ///
    /// Every map has a border and out of bounds means that the position
    /// is inside the border.
    pub fn is_position_out_of_bounds(&self, position: Position) -> bool {
        position.x == 0
            || position.y == 0
            || position.x >= self.width - 1
            || position.y >= self.height - 1
    }

    /// Returns food position.
    ///
    /// # Panics
    ///
    /// It's forbidden to call this function before calling the `spawn_food()` function.
    /// Considered as a programmer error and will panic.
    pub fn food_position(&self) -> Position {
        self.food.as_ref().unwrap().position
    }

    /// Spawns a new food and draws it.
    ///
    /// The `snake` argument is used to check that the food position doesn't collide
    /// with any snake fragment.
    pub fn spawn_food(&mut self, snake: &Snake) -> Result<()> {
        let free_area_width = self.width - 2;
        let free_area_height = self.height - 2;
        let free_area_position_count = free_area_width * free_area_height;

        // Naive implementation, but enough for an example
        let position = loop {
            let index = Range::new(0, free_area_position_count).ind_sample(&mut rand::thread_rng());
            let x = index % free_area_width + 1;
            let y = index / free_area_width + 1;
            let position = (x, y).into();

            if !snake.fragment_exists_at_position(position) {
                break position;
            }
        };

        let food = Food::new(position);
        food.draw()?;
        self.food = Some(food);
        return Ok(());
    }
}
