use advent_of_code::day_12::*;
use std::collections::{HashSet, VecDeque};
fn main() {

    // PART 1

    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut buf_queue: Vec<(usize, usize)> = Vec::new();
    let mut tracker: HashSet<(usize, usize)> = HashSet::new();
    let mut map = ElevationMap::new("inputs/12.inputs.txt");
    let mut counter: u32 = 1;
    
    let (start_idx, end_idx) = get_start_end(&map);
    map.0[start_idx.0][start_idx.1] = ElevationMap::as_int('a');
    map.0[end_idx.0][end_idx.1] = ElevationMap::as_int('z');

    push(&mut queue, &mut tracker, start_idx);

    println!("{start_idx:?}, {end_idx:?}");
    'main: loop {
        for coordinate in queue.iter() {
            for neighbor in neighbors(&map, &coordinate) {
                if let Some(neighbor) = neighbor {
                    if neighbor == end_idx {
                        println!("Hello neighbor");
                        break 'main;
                    } else if !tracker.contains(&neighbor) {
                        push(&mut buf_queue, &mut tracker, neighbor);
                    }
                }
            }
        }
        queue = buf_queue.clone();
        buf_queue = Vec::new();
        counter += 1;
    }
    println!("{counter}");

    // PART 2: flip start and end, define end by elevation.
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut buf_queue: Vec<(usize, usize)> = Vec::new();
    let mut tracker: HashSet<(usize, usize)> = HashSet::new();
    let mut map = ElevationMap::new("inputs/12.inputs.txt");
    let mut counter: u32 = 1;
    
    let (start_idx, end_idx) = get_start_end(&map);
    map.0[start_idx.0][start_idx.1] = ElevationMap::as_int('a');
    map.0[end_idx.0][end_idx.1] = ElevationMap::as_int('z');

    push(&mut queue, &mut tracker, end_idx);

    println!("{end_idx:?}");
    'main: loop {
        for coordinate in queue.iter() {
            for neighbor in rev_neighbors(&map, &coordinate) {
                if let Some(neighbor) = neighbor {
                    if map.0[neighbor.0][neighbor.1] == ElevationMap::as_int('a') {
                        println!("Hello neighbor");
                        break 'main;
                    } else if !tracker.contains(&neighbor) {
                        push(&mut buf_queue, &mut tracker, neighbor);
                    }
                }
            }
        }
        queue = buf_queue.clone();
        buf_queue = Vec::new();
        counter += 1;
    }
    println!("{counter}");

}

fn push(queue: &mut Vec<(usize, usize)>, tracker: &mut HashSet<(usize, usize)>, coordinate: (usize, usize)) {
    queue.push(coordinate);
    tracker.insert(coordinate);
}