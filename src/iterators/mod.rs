use crate::grammar::Grammar;
use crate::valid_word::ValidWord;

mod breadth_first;
mod depth_first;

#[derive(Debug)]
pub enum ValidWordIteratorStrategy {
    BreadthFirst,
    DepthFirst { recursion_limit: usize },
}

pub struct ValidWordIterator<'grammar>(ValidWordIteratorInner<'grammar>);

enum ValidWordIteratorInner<'grammar> {
    BreadthFirst(breadth_first::BreadthFirstValidWordIterator<'grammar>),
    DepthFirst(depth_first::DepthFirstValidWordIterator<'grammar>),
}

impl<'grammar> ValidWordIterator<'grammar> {
    pub(crate) fn new(grammar: &'grammar Grammar, strategy: ValidWordIteratorStrategy) -> Self {
        Self(match strategy {
            ValidWordIteratorStrategy::BreadthFirst => ValidWordIteratorInner::BreadthFirst(
                breadth_first::BreadthFirstValidWordIterator::new(grammar),
            ),
            ValidWordIteratorStrategy::DepthFirst { recursion_limit } => {
                ValidWordIteratorInner::DepthFirst(depth_first::DepthFirstValidWordIterator::new(
                    grammar,
                    recursion_limit,
                ))
            }
        })
    }
}

impl<'grammar> Iterator for ValidWordIterator<'grammar> {
    type Item = ValidWord<'grammar>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            ValidWordIteratorInner::BreadthFirst(iter) => iter.next(),
            ValidWordIteratorInner::DepthFirst(iter) => iter.next(),
        }
    }
}
