use super::forest::{Forest, Tree};
use std::sync::Arc;
use std::thread;


#[derive(Default, Clone)]
struct VisibilityBitMap(Vec<Vec<bool>>);
impl VisibilityBitMap {
    fn with_capactiy(rows: usize) -> Self {
        Self(Vec::with_capacity(rows))
    }
}

fn calc_top(forest: Arc<Forest>) -> VisibilityBitMap {
    let forest = forest;
    let (rows, columns) = (forest.0.len(), forest.0[0].len());

    let mut bitmap = VisibilityBitMap(vec![vec![false; columns]; rows]);

    let mut max_height: i32;
    for i in 0..rows {
        max_height = -1;

        for j in 0..columns {
            let tree_height = forest.0[i][j].height;
            
            if tree_height > max_height {
                max_height = tree_height;
                bitmap.0[i][j] = true;
            }
        }
    }
    bitmap
}

fn calc_left(forest: Arc<Forest>) -> VisibilityBitMap {
    let forest = forest;
    let (rows, columns) = (forest.0.len(), forest.0[0].len());

    let mut bitmap = VisibilityBitMap(vec![vec![false; columns]; rows]);

    let mut max_height: i32;
    for j in 0..columns {
        max_height = -1;
        for i in 0..rows {
            let tree_height = forest.0[i][j].height;
            if tree_height > max_height {
                max_height = tree_height;
                bitmap.0[i][j] = true;
            }
        }
    }
    bitmap
}

fn calc_bottom(forest: Arc<Forest>) -> VisibilityBitMap {
    let forest = forest;
    let (rows, columns) = (forest.0.len(), forest.0[0].len());

    let mut bitmap = VisibilityBitMap(vec![vec![false; columns]; rows]);

    let mut max_height: i32;
    for i in 0..rows {
        max_height = -1;
        for j in (0..columns).rev() {
            let tree_height = forest.0[i][j].height;
            if tree_height > max_height {
                max_height = tree_height;
                bitmap.0[i][j] = true;
            }
        }
    }
    bitmap
}

fn calc_right(forest: Arc<Forest>) -> VisibilityBitMap {
    let forest = forest;
    let (rows, columns) = (forest.0.len(), forest.0[0].len());

    let mut bitmap = VisibilityBitMap(vec![vec![false; columns]; rows]);

    let mut max_height: i32;
    for j in 0..columns {
        max_height = -1;
        for i in (0..rows).rev() {
            let tree_height = forest.0[i][j].height;
            if tree_height > max_height {
                max_height = tree_height;
                bitmap.0[i][j] = true;
            }
        }
    }
    bitmap
}

#[inline]
pub fn calc_visibility(forest: &Arc<Forest>) -> u32 {

    
    let arc_forest = Arc::clone(forest);
    let from_top = thread::spawn(move ||
        calc_top(arc_forest));

    let arc_forest = Arc::clone(forest);
    let from_bottom = thread::spawn(move ||
        calc_bottom(arc_forest));

    let arc_forest = Arc::clone(forest);
    let from_left = thread::spawn(move ||
        calc_left(arc_forest));

    let arc_forest = Arc::clone(forest);
    let from_right = thread::spawn(move ||
        calc_right(arc_forest));

    let top_map = from_top.join().unwrap();
    let bottom_map = from_bottom.join().unwrap();
    let left_map = from_left.join().unwrap();
    let right_map = from_right.join().unwrap();
    
    let mut total_visibilty: u32 = 0;
     
    for i in 0..top_map.0.len() {
        for j in 0..top_map.0[0].len() {
            if top_map.0[i][j] || bottom_map.0[i][j] || left_map.0[i][j] || right_map.0[i][j] {
                total_visibilty += 1;
            }
        }
    }

    total_visibilty
}

#[inline]
pub fn calc_desirability(forest: &Forest) {
    
}

#[cfg(test)]
mod threaded_8 {
    use super::*;
    
    #[test]
    fn ex() {
        let forest = crate::day_8_algorithms::new_forest();
        assert_eq!(1543,super::calc_visibility(&Arc::new(forest)));
    }
}