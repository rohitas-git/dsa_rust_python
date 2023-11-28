/* ------------------------------ Rat in a Maze ----------------------------- */

// Consider a rat placed at (0, 0) in a square matrix of order N * N.
// It has to reach the destination at (N – 1, N – 1).
// Find all possible paths that the rat can take to reach from source to destination.
// The directions in which the rat can move are ‘U'(up), ‘D'(down), ‘L’ (left), ‘R’ (right).

// Value 0 at a cell in the matrix represents that it is blocked and the rat cannot move to it
// while value 1 at a cell in the matrix represents that rat can travel through it.

// Note: In a path, no cell can be visited more than one time.

/* ------------------------------------ x ----------------------------------- */

use Value::*;

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Blocked,
    Passable,
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

    fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    fn up(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    fn left(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }
}

struct Maze {
    maze: Vec<Vec<Value>>,
    curr_position: Position,
    final_position: Position,
}

impl Maze {
    fn get_value(&self) -> Value {
        let x = self.curr_position.x;
        let y = self.curr_position.y;

        self.maze[x][y].clone()
    }

    fn new(maze: Vec<Vec<Value>>) -> Self {
        let n = maze.len();
        Maze {
            maze,
            curr_position: Position::new(0, 0),
            final_position: Position::new(n - 1, n - 1),
        }
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
        }
    }

    fn passable_position(&self, position: &Position) -> bool {
        match self.maze[position.x][position.y] {
            Blocked => false,
            Passable => true,
        }
    }

    fn down(&mut self) -> Option<Position> {
        let new_pos = &self.curr_position.down();
        if self.in_bound(new_pos) && self.passable_position(new_pos) {
            self.curr_position = new_pos.clone();
            return Some(new_pos.clone());
        }
        None
    }

    fn up(&mut self) -> Option<Position> {
        let new_pos = &self.curr_position.up();
        if self.in_bound(new_pos) && self.passable_position(new_pos) {
            self.curr_position = new_pos.clone();
            return Some(new_pos.clone());
        }
        None
    }

    fn right(&mut self) -> Option<Position> {
        let new_pos = &self.curr_position.right();
        if self.in_bound(new_pos) && self.passable_position(new_pos) {
            self.curr_position = new_pos.clone();
            return Some(new_pos.clone());
        }
        None
    }

    fn left(&mut self) -> Option<Position> {
        let new_pos = &self.curr_position.left();
        if self.in_bound(new_pos) && self.passable_position(new_pos) {
            self.curr_position = new_pos.clone();
            return Some(new_pos.clone());
        }
        None
    }
}

fn backtrack(path: &mut Vec<Position>) -> Option<Position> {
    path.pop()
}

fn all_paths(maze: Vec<Vec<Value>>) {
    let n = maze.len();
    let mut all_paths: Vec<Vec<Position>> = Vec::new();
    let mut curr_path = Vec::new();
    let mut maze = Maze::new(maze);

    maze.get_paths(&mut curr_path, &mut all_paths);

    dbg!(all_paths);
}

impl Maze {
    fn get_paths(&mut self, curr_path: &mut Vec<Position>, all_paths: &mut Vec<Vec<Position>>) {
        if self.curr_position == self.final_position {
            all_paths.push(curr_path.clone());
            return;
        }

        if let Some(position) = self.down(){
            curr_path.push(position);
            self.get_paths(curr_path, all_paths);
            
            backtrack(curr_path);
        };


        if let Some(position) = self.up(){
            curr_path.push(position);
            self.get_paths(curr_path, all_paths);
            
            backtrack(curr_path);
        };


        if let Some(position) = self.right(){
            curr_path.push(position);
            self.get_paths(curr_path, all_paths);
            
            backtrack(curr_path);
        };

        if let Some(position) = self.left(){
            curr_path.push(position);
            self.get_paths(curr_path, all_paths);
            
            backtrack(curr_path);
        };
    }
}


#[cfg(test)]
mod test_super {
    use std::vec;

    use super::*;

    #[test]
    fn test_finding_paths() {
        let maze = vec![vec![]];
    }
}