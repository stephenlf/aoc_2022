/// Key topics covered
/// !!! Multithreading ??? And/OR AsyNC??? Also, measuring performance.
/// * Benchmarking. https://nnethercote.github.io/perf-book/title-page.html
///     - Criterion: https://bheisler.github.io/criterion.rs/book/getting_started.html
/// * Nested vectors
/// - derive clone

use std::{path::PathBuf, fs::File, io::{BufReader, Lines, BufRead}};

fn lines_from_file(path: PathBuf) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
    
}

#[derive(Default, Debug)]
struct Forest(Vec<Vec<Tree>>);         // Vec<Rows> where Rows<Vec<u32>>

impl Forest {
    fn calc_visibility(&mut self) {
        let (rows, columns) = (self.0.len(), self.0[0].len());
        let mut max_height: i32 = -1;

        // Left to right
        for i in 0..rows {
            max_height = -1;
            for j in 0..columns {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visible = true;
                }
            }
        }
    
        // Top to bottom
        for j in 0..columns {
            max_height = -1;
            for i in 0..rows {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visible = true;
                }
            }
        }
    
        for i in 0..rows {
            max_height = -1;
            for j in (0..columns).rev() {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visible = true;
                }
            }
        }
    
        // Top to bottom
        for j in 0..columns {
            max_height = -1;
            for i in (0..rows).rev() {
                let tree_height = self.0[i][j].height;
                if tree_height > max_height {
                    max_height = tree_height;
                    self.0[i][j].visible = true;
                }
            }
        }
    }

    fn size(&self) -> u32 {
        self.0.iter().flatten()
            .map(|tree| {
                match tree.visible {
                    true => 1,
                    false => 0,
                }
            }).sum::<u32>()
    }

    fn calc_desirabilities(&mut self) {
        let (rows, columns) = (self.0.len(), self.0[0].len());
        
        for i in 0..rows {
            for j in 0..columns {
                self.calc_desirability(i, j);
            }
        }
    }

    fn calc_desirability(&mut self, row: usize, column: usize) {
        let (max_height, max_width) = (self.0.len(), self.0[0].len());
        // Start at tree, go up
        
        let mut top_view: u32 = 0;
        for i in (0..row).rev() {
            let height = self.0[i][column].height;
            top_view += 1;
            if height >= self.0[row][column].height {
                break
            }
        }
        
        let mut bottom_view: u32 = 0;
        for i in row+1..max_height {
            let height = self.0[i][column].height;
            bottom_view += 1;
            if height >= self.0[row][column].height {
                break
            }
        }
        
        let mut left_view: u32 = 0;
        for j in (0..column).rev() {
            let height = self.0[row][j].height;
            left_view += 1;
            if height >= self.0[row][column].height {
                break
            }
        }
        
        let mut right_view: u32 = 0;
        for j in column+1..max_width {
            let height = self.0[row][j].height;
            right_view += 1;
            if height >= self.0[row][column].height {
                break
            }
        }

        self.0[row][column].desirable = top_view * bottom_view * left_view * right_view;
    }

    fn max_desirability(&self) -> u32 {
        self.0.iter().flatten().map(|t| t.desirable).max().unwrap()
    }
}

#[derive(Clone, Default, Debug)]
struct Tree {
    height: i32,
    visible: bool,
    desirable: u32,
}
// my_forest[row][column]

impl Tree {
    fn new(height: i32) -> Self {
        Tree {
            height,
            visible: false,
            desirable: 1,
        }
    }


}

fn forest_from_lines(lines: Lines<BufReader<File>>) -> Forest {
    let mut forest = Forest::default();
    for line in lines.map(|f| f.unwrap()) {
        forest.0.push(Vec::new());
        for c in line.chars() {
            forest.0
            .last_mut()
            .unwrap()
            .push(Tree::new(c.to_digit(10).unwrap().try_into().unwrap()));
        }
    }
    forest
}
// Optimization: check visibility-from-left as forest is initialized

fn main() {
    let lines = lines_from_file(PathBuf::from("inputs/8.inputs.txt"));
    let mut forest = forest_from_lines(lines);

    forest.calc_visibility();   
    println!("{}",forest.size());

    forest.calc_desirabilities();

    println!("{}",forest.max_desirability());

    // Optimization: run async [may require different storage type]
}

#[cfg(test)] 
mod tests_8 {
    use super::*;

    #[test]
    fn test_calc_des() {
        let t0 = Tree {
            height: 0,
            visible: false,
            desirable: 0,
        };
        let t1 = Tree {
            height: 1,
            visible: false,
            desirable: 0,
        };
        let mut forest = Forest(
            vec![
                vec![t0.clone(); 5],
                vec![t0.clone(); 5],
                vec![t0.clone(),t0.clone(),t1,t0.clone(),t0.clone()],
                vec![t0.clone(); 5],
                vec![t0.clone(); 5],
            ]
        );
        forest.calc_desirabilities();
        assert_eq!(forest.0[2][2].desirable, 16);
        assert_eq!(forest.0[2][2].desirable, 16);
    }
}