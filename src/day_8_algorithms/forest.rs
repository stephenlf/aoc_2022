#[derive(Default, Debug)]
pub struct Forest(pub Vec<Vec<Tree>>);         // Vec<Rows> where Rows<Vec<u32>>

impl Forest {
    pub fn size(&self) -> u32 {
        self.0.iter().flatten()
            .map(|tree| {
                match tree.visible {
                    true => 1,
                    false => 0,
                }
            }).sum::<u32>()
    }

    pub fn max_desirability(&self) -> u32 {
        self.0.iter().flatten().map(|t| t.desirable).max().unwrap()
    }
}

#[derive(Clone, Default, Debug)]
pub struct Tree {
    pub height: i32,
    pub visible: bool,
    pub desirable: u32,
}
// my_forest[row][column]

impl Tree {
    pub fn new(height: i32) -> Self {
        Tree {
            height,
            visible: false,
            desirable: 1,
        }
    }


}

