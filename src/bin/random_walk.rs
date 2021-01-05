use rand::Rng;

const MODULO: i32 = 32768;
const MAX_PATH: usize = 12;
const MAX_ITERATIONS: usize = 10_000_000;

#[derive(Debug)]
enum Node {
    Number(i32),
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn next_direction(location: (usize, usize)) -> Direction {
    use Direction::*;

    let direction = match rand::thread_rng().gen_range(0, 4) {
        0 => North,
        1 => South,
        2 => East,
        3 => West,
        _ => panic!("shouldn't happen"),
    };

    match (&direction, location) {
        (North, (0, _)) => next_direction(location),
        (South, (3, _)) => next_direction(location),
        (South, (2, 0)) => next_direction(location),
        (West, (_, 0)) => next_direction(location),
        (West, (3, 1)) => next_direction(location),
        (East, (_, 3)) => next_direction(location),
        _ => direction,
    }
}

fn next_location(location: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (location.0 - 1, location.1),
        Direction::South => (location.0 + 1, location.1),
        Direction::East => (location.0, location.1 + 1),
        Direction::West => (location.0, location.1 - 1),
    }
}

fn eval(map: &Vec<Vec<Node>>, path: &Vec<Direction>) -> i32 {
    if path.len() % 2 == 0 {
        let mut nodes: Vec<&Node> = vec![];
        let mut location: (usize, usize) = (3, 0);

        for direction in path {
            location = next_location(location, direction);
            nodes.push(&map[location.0][location.1]);
        }

        let mut iterator = nodes.iter();
        let mut total: i32 = 22;

        while let Some(op) = iterator.next() {
            if let Some(node) = iterator.next() {
                total = match (op, node) {
                    (&Node::Add, &Node::Number(rhs)) => total + rhs,
                    (&Node::Sub, &Node::Number(rhs)) => total - rhs,
                    (&Node::Mul, &Node::Number(rhs)) => total * rhs,
                    _ => panic!("shouldn't see a non-op node here"),
                }
            } else {
                break;
            }
        }

        total
    } else {
        1
    }
}

fn main() -> std::io::Result<()> {
    use Node::*;
    let map: Vec<Vec<Node>> = vec![
        vec![Mul, Number(8), Sub, Number(1)],
        vec![Number(4), Mul, Number(11), Mul],
        vec![Add, Number(4), Sub, Number(18)],
        vec![Number(22), Sub, Number(9), Mul],
    ];
    let mut iterations: usize = 0;

    loop {
        if iterations >= MAX_ITERATIONS {
            break;
        }

        let mut location: (usize, usize) = (3, 0);
        let mut path: Vec<Direction> = vec![];
        let mut found: bool = false;

        loop {
            if path.len() >= MAX_PATH {
                break;
            }

            let direction = next_direction(location);
            location = next_location(location, &direction);
            path.push(direction);

            let total = eval(&map, &path);

            if total <= 0 || total >= MODULO {
                break;
            }

            if location == (0, 3) {
                if total == 30 {
                    println!("{:?} => {}", path, total);
                    found = true;
                }
                break;
            }
        }

        if found {
            break;
        }
        iterations += 1;
    }

    Ok(())
}
