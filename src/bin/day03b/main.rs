// http://adventofcode.com/2017/day/3

use std::collections::HashMap;

fn main() {
    println!("{}", solve(289326));
}

fn solve(target_value: u32) -> u32 {
    let mut matrix = HashMap::new();

    // TODO initialize point (0, 0) in hashmap to hold value 1

    let cursor = Cursor::new(&mut matrix);

    loop {
        // TODO
        // get the next cursor
        // check cursor.point.value()
        // if it's greater than target_value return it
    }
}


#[derive(Debug)]
struct Cursor<'a> {
    matrix: &'a mut HashMap<u32, HashMap<u32, u32>>,
    direction: Direction,  // the direction the cursor's "pointed" (and by extension which edge it's on)
    edge_index: u32,  // position of cursor from beginning of current edge (edges are zero-indexed)
    edge_length: u32,  // size of current edge
    point: Point,  // (x, y) coordinates of cursor in overall matrix
}

impl Cursor {
    // initialize new cursor with starting conditions described in the question
    fn new(matrix: &mut HashMap<u32, HashMap<u32, u32>>) -> Cursor {
        Cursor {
            matrix,
            direction: Direction::RIGHT,
            edge_index: 0,
            edge_length: 1,
            point: Point { x: 0, y: 0 },
        }
    }

    // return cursor representing the next state we get to when walking the spiral
    fn next(self) -> Cursor {

        //  reference:

        //  37  36  35  34  33  32  31
        //  38  17  16  15  14  13  30
        //  39  18   5   4   3  12  29
        //  40  19   6   1   2  11  28
        //  41  20   7   8   9  10  27
        //  42  21  22  23  24  25  26
        //  43  44  45  46  47  48  49

        let is_time_to_turn = self.edge_index == self.edge_length - 1  // we're in the edge's last spot
                              && self.direction != Direction::RIGHT;  // and we're NOT in the last edge of the ring

        // DIRECTION
        let next_direction = match is_time_to_turn {
            true => self.direction.turn(),
            false => self.direction,
        };

        // FIXME these conditions as written do not correctly set is_new_edge and is_new_ring, but
        // such conditions do exist; I just need to think about them a little more
        let is_new_edge = self.direction != next_direction;
        let is_new_ring = (self.direction, next_direction) == (Direction::RIGHT, Direction::UP);

        // EDGE_INDEX
        let next_edge_index = match is_new_edge {
            true => 0,
            false => self.edge_index + 1,
        };

        // EDGE_LENGTH
        let next_edge_length = match is_new_ring {
            true => self.edge_length + 2,
            false => self.edge_length,
        };

        // POINT
        let (dx, dy) = match next_direction {
            Direction::RIGHT => ( 1,  0),
            Direction::UP =>    ( 0,  1),
            Direction::LEFT =>  (-1,  0),
            Direction::DOWN =>  ( 0, -1),
        };
        let next_point = Point {
            x: self.point.x + dx,
            y: self.point.y + dy,
        };

        let next_value = next_point.value();

        // TODO save next_value to the hashmap (need to look up syntax)

        // return next cursor
        Cursor {
            matrix: self.matrix,
            direction: next_direction,
            edge_index: next_edge_index,
            edge_length: next_edge_length,
            point: next_point,
        }
    }
}


#[derive(PartialEq, Debug)]
enum Direction {
    RIGHT,
    UP,
    LEFT,
    DOWN,
}

impl Direction {
    fn turn(&self) -> Direction {
        match *self {
            Direction::RIGHT => Direction::UP,
            Direction::UP => Direction::LEFT,
            Direction::LEFT => Direction::DOWN,
            Direction::DOWN => Direction::RIGHT,
        }
    }
}


#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {

    fn neighbors(&self) -> [Point; 8] {[
        Point { x: self.x + 1, y: self.y },

        Point { x: self.x + 1, y: self.y + 1 },
        Point { x: self.x,     y: self.y + 1 },
        Point { x: self.x - 1, y: self.y + 1 },

        Point { x: self.x - 1, y: self.y },

        Point { x: self.x - 1, y: self.y - 1 },
        Point { x: self.x,     y: self.y - 1 },
        Point { x: self.x + 1, y: self.y - 1 },
    ]}


    // TODO finish implementing this function
    fn value(&self) -> u32 {
        match *self {
            Point { x: 0, y: 0 } => 1,
            _ => {
                for neighbor in self.neighbors.iter() {
                    // more logic in here
                }
                666  // FIXME dummy return value
            },
        }
    }
}


#[test]
fn value_at_index_1() {
    assert!(value_at_index(1) == 1);
}

#[test]
fn value_at_index_2() {
    assert!(value_at_index(2) == 1);
}

#[test]
fn value_at_index_4() {
    assert!(value_at_index(4) == 4);
}

#[test]
fn value_at_index_5() {
    assert!(value_at_index(5) == 5);
}
