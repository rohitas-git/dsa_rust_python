/* ------------------------------ Rat in a Maze ----------------------------- */

// Consider a rat placed at (0, 0) in a square matrix of order N * N.
// It has to reach the destination at (N – 1, N – 1).
// Find all possible paths that the rat can take to reach from source to destination.
// The directions in which the rat can move are ‘U'(up), ‘D'(down), ‘L’ (left), ‘R’ (right).

// Value 0 at a cell in the matrix represents that it is blocked and the rat cannot move to it
// while value 1 at a cell in the matrix represents that rat can travel through it.

// Note: In a path, no cell can be visited more than one time.

/* ------------------------------- Complexity ------------------------------- */
// Time: O(4^(m*n) 
// because on every cell we need to try 4 different directions.

// Space Complexity: O(m*n) 
// Maximum Depth of the recursion tree(auxiliary space).

/* ------------------------------------ x ----------------------------------- */

use Value::*;

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Blocked,
    Passable,
    Visited,
}

fn get_val(x: u32) -> Value {
    match x {
        1 => Passable,
        _ => Blocked,
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn down(&self) -> Option<Self> {
        if let Some(y) = self.y.checked_add(1) {
            return Some(Self::new(self.x, y));
        }
        None
    }

    fn up(&self) -> Option<Self> {
        if let Some(y) = self.y.checked_sub(1) {
            return Some(Self::new(self.x, y));
        }
        None
    }

    fn right(&self) -> Option<Self> {
        if let Some(x) = self.x.checked_add(1) {
            return Some(Self::new(x, self.y));
        }
        None
    }

    fn left(&self) -> Option<Self> {
        if let Some(x) = self.x.checked_sub(1) {
            return Some(Self::new(x, self.y));
        }
        None
    }
}

#[derive(Debug, Clone)]
struct Maze {
    maze: Vec<Vec<Value>>,
    curr_position: Position,
    final_position: Position,
}

impl Maze {
    fn get_value(&self) -> Value {
        let x = self.curr_position.x;
        let y = self.curr_position.y;

        self.maze[y][x].clone()
    }

    fn get_value_mut(&mut self) -> &mut Value {
        let x = self.curr_position.x;
        let y = self.curr_position.y;

        &mut self.maze[y][x]
    }

    fn value_at(&mut self, position: &Position) -> &mut Value {
        let x = position.x;
        let y = position.y;

        &mut self.maze[y][x]
    }

    fn set_visited(&mut self, position: &Position) -> bool {
        let val = self.value_at(position);
        *val = Visited;
        true
    }

    fn set_passable(&mut self, position: &Position) -> bool {
        let val = self.value_at(position);
        *val = Passable;
        true
    }

    fn new(maze: Vec<Vec<u32>>) -> Self {
        let n = maze.len();
        let maze = maze
            .iter()
            .map(|row| row.iter().map(|val| get_val(val.clone())).collect())
            .collect();
        Maze {
            maze,
            curr_position: Position::new(0, 0),
            final_position: Position::new(n - 1, n - 1),
        }
    }

    fn print_position(&self) {
        println!("Here: ({},{})", self.curr_position.x, self.curr_position.y);
    }

    fn in_bound(&self, position: &Position) -> bool {
        let n = self.maze.len();
        let x = position.x;
        let y = position.y;

        y < n && x < n
    }

    fn passable_current(&self) -> bool {
        match self.get_value() {
            Blocked => false,
            Passable => true,
            Visited => false,
        }
    }

    fn passable_position(&self, position: &Position) -> bool {
        match self.maze[position.y][position.x] {
            Blocked => false,
            Passable => true,
            Visited => false,
        }
    }

    fn down(&mut self) -> Option<Position> {
        let prev_pos = self.curr_position.clone();
        let new_pos = &self.curr_position.down()?;
        if self.in_bound(new_pos) && self.passable_position(new_pos) {
            self.curr_position = new_pos.clone();
            return Some(prev_pos.clone());
        }
        None
    }

    fn up(&mut self) -> Option<Position> {
        let prev_pos = self.curr_position.clone();
        let new_pos = &self.curr_position.up()?;
        if self.in_bound(new_pos) && self.passable_position(new_pos) {
            self.curr_position = new_pos.clone();
            return Some(prev_pos.clone());
        }
        None
    }

    fn right(&mut self) -> Option<Position> {
        let prev_pos = self.curr_position.clone();
        let new_pos = &self.curr_position.right()?;
        if self.in_bound(new_pos) && self.passable_position(new_pos) {
            self.curr_position = new_pos.clone();
            return Some(prev_pos.clone());
        }
        None
    }

    fn left(&mut self) -> Option<Position> {
        let prev_pos = self.curr_position.clone();
        let new_pos = &self.curr_position.left()?;
        if self.in_bound(new_pos) && self.passable_position(new_pos) {
            self.curr_position = new_pos.clone();
            return Some(prev_pos.clone());
        }
        None
    }
}

fn backtrack(path: &mut String) -> Option<char> {
    path.pop()
}

fn all_paths(maze: Vec<Vec<u32>>) {
    let mut all_paths: Vec<String> = Vec::new();
    let mut curr_path = String::new();
    let mut maze = Maze::new(maze);

    maze.get_paths(&mut curr_path, 0, &mut all_paths);

    dbg!(all_paths);
}

impl Maze {
    fn get_paths(&mut self, curr_path: &mut String, _index: u32, all_paths: &mut Vec<String>) {
        if self.curr_position == self.final_position {
            all_paths.push(curr_path.clone());
            return;
        }

        println!("{}: Right {}", _index, curr_path);
        if let Some(position) = self.right() {
            self.set_visited(&position);
            curr_path.push('R');
            self.get_paths(curr_path, _index + 1, all_paths);

            backtrack(curr_path);
            self.set_passable(&position);
            self.left();
        };

        println!("{}: Down {}", _index, curr_path);
        if let Some(position) = self.down() {
            self.set_visited(&position);
            curr_path.push('D');
            self.get_paths(curr_path, _index + 1, all_paths);

            backtrack(curr_path);
            self.set_passable(&position);
            self.up();
        };

        println!("{}: Up {}", _index, curr_path);
        if let Some(position) = self.up() {
            self.set_visited(&position);
            curr_path.push('U');
            self.get_paths(curr_path, _index + 1, all_paths);

            backtrack(curr_path);
            self.set_passable(&position);
            self.down();
        };

        println!("{}: Left {}", _index, curr_path);
        if let Some(position) = self.left() {
            self.set_visited(&position);
            curr_path.push('L');
            self.get_paths(curr_path, _index + 1, all_paths);

            backtrack(curr_path);
            self.set_passable(&position);
            self.right();
        };
    }
}

#[cfg(test)]
mod test_super {
    use std::vec;

    use super::*;

    #[test]
    fn test_finding_paths() {
        let maze = vec![vec![1u32, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        all_paths(maze);

        let maze = vec![
            vec![1u32, 0, 0, 0],
            vec![1, 1, 0, 1],
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 1],
        ];
        all_paths(maze);
    }
}
