use crate::solution::Solution;


pub trait Neighborhood {
    fn iter_neighbors<'a>(&self, solution: Solution<'a>) -> Box<dyn Iterator<Item = Solution<'a>> + 'a>;
}
