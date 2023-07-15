use super::forest::{Forest, Tree};

#[inline]
pub fn calc_visibility(forest: &mut Forest) -> u32 {
    let (rows, columns) = (forest.0.len(), forest.0[0].len());
    let mut max_height: i32 = -1;

    // Left to right
    for i in 0..rows {
        max_height = -1;
        for j in 0..columns {
            let tree_height = forest.0[i][j].height;
            if tree_height > max_height {
                max_height = tree_height;
                forest.0[i][j].visible = true;
            }
        }
    }

    // Top to bottom
    for j in 0..columns {
        max_height = -1;
        for i in 0..rows {
            let tree_height = forest.0[i][j].height;
            if tree_height > max_height {
                max_height = tree_height;
                forest.0[i][j].visible = true;
            }
        }
    }

    for i in 0..rows {
        max_height = -1;
        for j in (0..columns).rev() {
            let tree_height = forest.0[i][j].height;
            if tree_height > max_height {
                max_height = tree_height;
                forest.0[i][j].visible = true;
            }
        }
    }

    // Top to bottom
    for j in 0..columns {
        max_height = -1;
        for i in (0..rows).rev() {
            let tree_height = forest.0[i][j].height;
            if tree_height > max_height {
                max_height = tree_height;
                forest.0[i][j].visible = true;
            }
        }
    }

    forest.size()
}

#[inline]
pub fn calc_desirability(forest: &mut Forest) -> u32 {
    let (rows, columns) = (forest.0.len(), forest.0[0].len());
    
    for i in 0..rows {
        for j in 0..columns {
            desirability(forest, i, j);
        }
    }

    forest.max_desirability()
}

fn desirability(forest: &mut Forest, row: usize, column: usize) {
    let (max_height, max_width) = (forest.0.len(), forest.0[0].len());
    // Start at tree, go up
    
    let mut top_view: u32 = 0;
    for i in (0..row).rev() {
        let height = forest.0[i][column].height;
        top_view += 1;
        if height >= forest.0[row][column].height {
            break
        }
    }
    
    let mut bottom_view: u32 = 0;
    for i in row+1..max_height {
        let height = forest.0[i][column].height;
        bottom_view += 1;
        if height >= forest.0[row][column].height {
            break
        }
    }
    
    let mut left_view: u32 = 0;
    for j in (0..column).rev() {
        let height = forest.0[row][j].height;
        left_view += 1;
        if height >= forest.0[row][column].height {
            break
        }
    }
    
    let mut right_view: u32 = 0;
    for j in column+1..max_width {
        let height = forest.0[row][j].height;
        right_view += 1;
        if height >= forest.0[row][column].height {
            break
        }
    }

    forest.0[row][column].desirable = top_view * bottom_view * left_view * right_view;
}
