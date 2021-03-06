// http://adventofcode.com/2017/day/3

use std::collections::HashMap;


fn main() {
    println!("{}", solve(289326));
}


fn solve(target_value: u32) -> u32 {
    let mut cursor = Cursor::new();
    loop {
        let value = cursor.next();
        if value > target_value {
            return value;
        }
    }
}


type PointMatrix = HashMap<(i32, i32), u32>;


#[derive(Debug)]
struct Cursor {
    matrix: PointMatrix, // data structure holding the values computed by the cursor so far
    direction: Direction, // the direction the cursor is "pointed" (by extension which edge it's on)
    ring_index: u32, // position of cursor from beginning of current ring (zero indexed)
    edge_index: u32, // position of cursor from beginning of current edge (zero-indexed)
    edge_length: u32, // size of current edge
    point: Point, // (x, y) coords of cursor in overall matrix (the initial value is at (0, 0))
}

impl Cursor {

    //  index reference:

    //  37  36  35  34  33  32  31
    //  38  17  16  15  14  13  30
    //  39  18   5   4   3  12  29
    //  40  19   6   1   2  11  28
    //  41  20   7   8   9  10  27
    //  42  21  22  23  24  25  26
    //  43  44  45  46  47  48  49

    // value reference:

    // 147  142  133  122   59
    // 304    5    4    2   57
    // 330   10    1    1   54
    // 351   11   23   25   26
    // 362  747  806  880  931

    // initialize new cursor with starting conditions described in the question
    fn new() -> Cursor {
        let mut matrix = PointMatrix::new();
        matrix.insert((0, 0), 1); // odd case
        matrix.insert((1, 0), 1); // first space on a ring
        Cursor {
            matrix,
            direction: Direction::UP,
            ring_index: 0,
            edge_index: 0,
            edge_length: 2,
            point: Point { x: 1, y: 0 },
        }
    }

    // update state of cursor to represent the next state we get to when walking the spiral, and
    // return the value stored there
    fn next(&mut self) -> u32 {

        // EDGE_INDEX
        let (next_edge_index, is_new_edge) = if self.edge_index == self.edge_length - 1 {
            (0, true)
        } else {
            (self.edge_index + 1, false)
        };

        // RING_INDEX
        let (next_ring_index, is_new_ring) = if self.ring_index == (self.edge_length * 4) - 1 {
            (0, true)
        } else {
            (self.ring_index + 1, false)
        };

        // EDGE_LENGTH
        let next_edge_length = if is_new_ring {
            self.edge_length + 2
        } else {
            self.edge_length
        };

        // DIRECTION
        let is_time_to_turn = (is_new_edge && !is_new_ring) // a turn within an existing ring
                || (self.ring_index == 0 && self.direction == Direction::RIGHT); // a turn after starting a new ring
        let next_direction = if is_time_to_turn {
            *self.direction.turn()
        } else {
            self.direction
        };

        // POINT
        let (dx, dy) = next_direction.dxdy();
        let next_point = Point {
            x: self.point.x + dx,
            y: self.point.y + dy,
        };

        // VALUE
        let next_value = self.compute_value(&next_point);

        // update cursor's attributes to represent the next state
        self.matrix.insert((next_point.x, next_point.y), next_value);
        self.direction = next_direction;
        self.ring_index = next_ring_index;
        self.edge_index = next_edge_index;
        self.edge_length = next_edge_length;
        self.point = next_point;

        // println!("\tend: {:?}", self);


        // return the value at the next state
        next_value
    }

    fn compute_value(&self, point: &Point) -> u32 {
        match self.matrix.get(&(point.x, point.y)) {
            Some(value) => panic!("found value {} in cell expected to be blank", value),
            None => point.neighbors().iter().map(|n| {
                match self.matrix.get(&(n.x, n.y)) {
                    Some(value) => *value,
                    None => 0,
                }
            }).sum(),
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    RIGHT,
    UP,
    LEFT,
    DOWN,
}

impl Direction {
    // return the direction one quarter turn counter-clockwise from "self"
    fn turn(&self) -> &Direction {
        match *self {
            Direction::RIGHT => &Direction::UP,
            Direction::UP => &Direction::LEFT,
            Direction::LEFT => &Direction::DOWN,
            Direction::DOWN => &Direction::RIGHT,
        }
    }

    // return tuple of changes to x and y needed to shift one space in the "self" diretion
    fn dxdy(&self) -> (i32, i32) {
        match *self {
            Direction::RIGHT => (1, 0),
            Direction::UP => (0, 1),
            Direction::LEFT => (-1, 0),
            Direction::DOWN => (0, -1),
        }
    }
}


#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbors(&self) -> [Point; 8] {
        [
            Point {
                x: self.x + 1,
                y: self.y,
            },

            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },

            Point {
                x: self.x - 1,
                y: self.y,
            },

            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_at_index_1() {
        let mut cursor = Cursor::new();
        for _ in 0..5 {
            cursor.next();
        }
        assert_eq!(cursor.matrix.get(&(0, 0)), Some(1).as_ref());
    }

    #[test]
    fn value_at_index_2() {
        let mut cursor = Cursor::new();
        for _ in 0..5 {
            cursor.next();
        }
        assert_eq!(cursor.matrix.get(&(1, 0)), Some(1).as_ref());
    }

    #[test]
    fn value_at_index_4() {
        let mut cursor = Cursor::new();
        for _ in 0..5 {
            cursor.next();
        }
        assert_eq!(cursor.matrix.get(&(0, 1)), Some(4).as_ref());
    }

    #[test]
    fn value_at_index_5() {
        let mut cursor = Cursor::new();
        for _ in 0..5 {
            cursor.next();
        }
        assert_eq!(cursor.matrix.get(&(-1, 1)), Some(5).as_ref());
    }
}
