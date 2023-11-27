use crate::neighborhood::neighborhood::Neighborhood;
use crate::solution::Solution;

pub struct OneFlip;

impl Neighborhood for OneFlip {
    fn iter_neighbors<'a>(&self, solution: Solution<'a>) -> Box<dyn Iterator<Item = Solution<'a>> + 'a> {
        Box::new(OneFlipIter::<'a>::new(solution))
    }
}

struct OneFlipIter<'a> {
    original: Solution<'a>,
    row: usize,
    col: usize,
}

impl<'a> OneFlipIter<'a> {
    fn new(original: Solution<'a>) -> Self {
        Self {
            original,
            row: 0,
            col: 1,
        }
    }
}

impl<'a> Iterator for OneFlipIter<'a> {
    type Item = Solution<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.original.edges.len() - 1 {
            return None;
        }

        let mut solution = self.original.clone();

        solution.flip_edge(self.row, self.col);

        if self.col == self.original.edges.len() - 1 {
            self.row += 1;
            self.col = self.row + 1;
        } else {
            self.col += 1;
        }

        Some(solution)
    }
}