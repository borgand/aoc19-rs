use std::cmp::{max, min};

advent_of_code::solution!(3);

use advent_of_code::*;

#[derive(Debug)]
struct Wire {
    steps: Vec<(char, i64)>,
    segments: Vec<Line>,
}

// implement parser for Wire
impl Wire {
    fn parse(input: &str) -> Wire {
        let mut steps = Vec::new();
        let mut segments = Vec::new();
        let mut a = Point { x: 0, y: 0 };
        let mut b = Point { x: 0, y: 0 };
        for step in input.split(',') {
            let (sdir, sdist) = step.split_at(1);
            let dir = sdir.chars().next().unwrap();
            let dist = sdist.parse().unwrap();
            steps.push((dir, dist));
            match dir {
                'U' => b.y += dist,
                'D' => b.y -= dist,
                'L' => b.x -= dist,
                'R' => b.x += dist,
                _ => panic!("Unknown direction"),
            }
            segments.push(Line { start: a, end: b });
            a = b;
        }

        Wire { steps, segments }
    }
}

// implement a method to draw a map of two wires, starting from ORIGIN:Point
// and specified by a vector segments:Line consisting of start and end Point.
// The map is 2D array of chars with 'o' for Origin, 'X' for intersection, '.' for empty space
// and -,+ and | for the wires. Print to stdout.
fn draw_map(wires: Vec<Wire>, intersections: Vec<Point>) {
    let shift:WorldCoord = 50;
    let size: MapCoord = (shift*2) as MapCoord;
    let mut map = vec![vec!['.'; size]; size];
    for wire in wires.iter() {
        for line in wire.segments.iter() {
            let (ax, ay) = line.start.to_map_coords(shift);
            let (bx, by) = line.end.to_map_coords(shift);

            map[ay][ax] = '+';
            if ax == bx {
                for y in min(ay, by)..=max(ay, by) {
                    map[y][ax] = '|';
                }
            } else {
                for x in min(ax, bx)..=max(ax, bx) {
                    map[ay][x] = '-';
                }
            }
        }
    }
    for intersection in intersections.iter() {
        let (ix, iy) = intersection.to_map_coords(shift);
        map[iy][ix] = 'X';
    }
    map[shift as usize][shift as usize] = 'o';
    for row in map.iter().rev() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut wires: Vec<Wire> = input.lines().map(Wire::parse).collect();
    let mut intersections = Vec::new();
    let mut min_distance = u64::MAX;
    let mut closest_intersection = ORIGIN;
    for line in wires[0].segments.iter() {
        for other in wires[1].segments.iter() {
            if let Some(point) = line.intersect(other) {
                if point.x == 0 && point.y == 0 {
                    continue;
                }
                intersections.push(point);
                println!("Intersection: {:?}", point);
                let distance = point.manhattan_distance_origin();
                if distance  < min_distance{
                    min_distance = distance as u64;
                    closest_intersection = point;
                }
            }
        }
    }
    if (is_debug()){
        draw_map(wires, intersections);
        println!("Closest intersection: {:?}", closest_intersection);
    }
    Some(min_distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(159));
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(135));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
