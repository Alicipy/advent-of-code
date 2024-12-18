use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use collections::VecDeque;
use const_format::concatcp;
use std::collections;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

fn parse_input<R: BufRead>(reader: R) -> Result<Vec<(usize, usize)>> {
    let fields = reader
        .lines()
        .map(|line| line.unwrap().to_string())
        .map(|l| {
            let p = l.split_once(",").unwrap();
            let x = p.0;
            let y = p.1;
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    Ok(fields)
}

fn prepare_field(relevant_input_fields: Vec<(usize, usize)>) -> Vec<Vec<char>> {
    let field_size = if relevant_input_fields.len() < 50 {
        7
    } else {
        71
    };

    let mut field = vec![vec!['.'; field_size]; field_size];
    for f in relevant_input_fields {
        field[f.0][f.1] = '#';
    }
    field
}

const NEIGHBOUR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn search_shortest_path(field: Vec<Vec<char>>) -> Result<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    let mut visited = field
        .iter()
        .map(|f| f.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    loop {
        let (pos, way) = queue.pop_front().unwrap();

        if visited[pos.0][pos.1] {
            continue;
        }

        if pos.0 == field.len() - 1 && pos.1 == field[0].len() - 1 {
            return Ok(way);
        }

        visited[pos.0][pos.1] = true;

        if field[pos.0][pos.1] == '#' {
            continue;
        }

        for n in NEIGHBOUR.iter() {
            let nx = pos.0 as i32 + n.0;
            let ny = pos.1 as i32 + n.1;
            if nx < 0 || nx >= field.len() as i32 || ny < 0 || ny >= field[0].len() as i32 {
                continue;
            }
            queue.push_back(((nx as usize, ny as usize), way + 1));
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, steps: usize) -> Result<usize> {
        let all_input_fields = parse_input(reader)?;
        let relevant_input_fields = all_input_fields.into_iter().take(steps).collect::<Vec<_>>();

        let field = prepare_field(relevant_input_fields);

        search_shortest_path(field)
    }

    assert_eq!(22, part1(BufReader::new(TEST1.as_bytes()), 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1024)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
