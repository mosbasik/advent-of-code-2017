// http://adventofcode.com/2017/day/3

fn main() {
    println!("{}", solve(289326));
}

pub fn solve(index: u32) -> u32 {
    let access_port = Point::from_index(1);
    let other_location = Point::from_index(index);
    access_port.manhattan(other_location)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_index(index: u32) -> Point {
        let (edge_id, edge_start_index, edge_start_point) = Point::get_edge_info(index);

        let (dx, dy) = match edge_id {
            0 => ( 0,  1),
            1 => (-1,  0),
            2 => ( 0, -1),
            3 => ( 1,  0),
            _ => panic!("unexpected edge_id found"),
        };

        let mut curr_index = edge_start_index;
        let mut curr_x = edge_start_point.x;
        let mut curr_y = edge_start_point.y;

        while curr_index != index {
            curr_index += 1;
            curr_x += dx;
            curr_y += dy;
        }

        Point {
            x: curr_x,
            y: curr_y,
        }
    }

    fn get_ring_info(index: u32) -> (u32, Point) {
        // the bottom right corner (end) of each ring is the square of an odd number [1, 9, 25, 49, ...]
        // so find out which odd sqare the ring containing our index ends on
        let root = (index as f64).sqrt();
        let root_ceil = root.ceil() as i32;

        let ring_end_index = match root_ceil % 2 {
            1 => root_ceil.pow(2) as u32,
            0 => (root_ceil + 1).pow(2) as u32,
            _ => panic!(),
        };

        let ring_end_point = Point {
            x: root_ceil / 2,
            y: -root_ceil / 2,
        };

        (ring_end_index, ring_end_point)
    }

    fn get_edge_info(index: u32) -> (u32, u32, Point) {

        //  reference:

        //  37  36  35  34  33  32  31
        //  38  17  16  15  14  13  30
        //  39  18   5   4   3  12  29
        //  40  19   6   1   2  11  28
        //  41  20   7   8   9  10  27
        //  42  21  22  23  24  25  26
        //  43  44  45  46  47  48  49

        let (ring_end_index, ring_end_point) = Point::get_ring_info(index);

        match ring_end_index {
            1 => (0, 1, Point { x: 0, y: 0 }),
            _ => {
                let root = (ring_end_index as f64).sqrt() as u32;
                let edge_length = root - 1;
                let ring_start_index = ring_end_index - (4 * edge_length) + 1;
                let edge_id = (index - ring_start_index) / edge_length;
                let edge_start_index = ring_start_index + (edge_id * edge_length);
                let edge_start_point = match edge_id {
                    0 => Point {
                        x: ring_end_point.x,
                        y: ring_end_point.y + 1,
                    },
                    1 => Point {
                        x: ring_end_point.x - 1,
                        y: ring_end_point.y + (edge_length as i32),
                    },
                    2 => Point {
                        x: ring_end_point.x - (edge_length as i32),
                        y: ring_end_point.y + (edge_length as i32) - 1,
                    },
                    3 => Point {
                        x: ring_end_point.x - (edge_length as i32) + 1,
                        y: ring_end_point.y,
                    },
                    _ => panic!("unexpected edge_id found"),
                };
                (edge_id, edge_start_index, edge_start_point)
            }
        }
    }

    fn manhattan(&self, other: Point) -> u32 {
        let dx = (self.x - other.x).abs() as u32;
        let dy = (self.y - other.y).abs() as u32;
        dx + dy
    }
}

#[test]
fn ring_info_1() {
    let (ring_end, point) = Point::get_ring_info(1);
    assert!(ring_end == 1);
    assert!(point.x == 0);
    assert!(point.y == 0);
}
#[test]
fn ring_info_5() {
    let (ring_end, point) = Point::get_ring_info(5);
    assert!(ring_end == 9);
    assert!(point.x == 1);
    assert!(point.y == -1);
}
#[test]
fn ring_info_21() {
    let (ring_end, point) = Point::get_ring_info(21);
    assert!(ring_end == 25);
    assert!(point.x == 2);
    assert!(point.y == -2);
}
#[test]
fn ring_info_37() {
    let (ring_end, point) = Point::get_ring_info(37);
    assert!(ring_end == 49);
    assert!(point.x == 3);
    assert!(point.y == -3);
}
#[test]
fn ring_info_2() {
    let (ring_end, point) = Point::get_ring_info(2);
    assert!(ring_end == 9);
    assert!(point.x == 1);
    assert!(point.y == -1);
}
#[test]
fn ring_info_10() {
    let (ring_end, point) = Point::get_ring_info(10);
    assert!(ring_end == 25);
    assert!(point.x == 2);
    assert!(point.y == -2);
}
#[test]
fn ring_info_30() {
    let (ring_end, point) = Point::get_ring_info(30);
    assert!(ring_end == 49);
    assert!(point.x == 3);
    assert!(point.y == -3);
}

#[test]
fn solve_1() {
    assert!(solve(1) == 0);
}
#[test]
fn solve_12() {
    assert!(solve(12) == 3);
}
#[test]
fn solve_23() {
    assert!(solve(23) == 2);
}
#[test]
fn solve_1024() {
    assert!(solve(1024) == 31);
}
