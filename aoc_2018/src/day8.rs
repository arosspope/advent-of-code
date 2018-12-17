// Day 8: Memory Maneuver

#[derive(Default, Debug, PartialEq)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
    length: usize, // Total count of numbers in this node. For root, this is the total count of the tree
}

impl Node {
    fn construct(stream: &[usize]) -> Node {
        let (num_children, num_metadata) = (stream[0], stream[1]);
        let mut node = Node {
            length: 2,
            ..Node::default()
        };

        for _ in 0..num_children {
            let child = Node::construct(&stream[node.length..]);
            node.length += child.length;
            node.children.push(child);
        }

        for _ in 0..num_metadata {
            let meta = stream[node.length];
            node.metadata.push(meta);
            node.length += 1;
        }

        node
    }

    fn sum_metadata(&self) -> usize {
        let sum: usize = self.children.iter().map(|c| c.sum_metadata()).sum(); // Sum of children
        sum + self.metadata.iter().sum::<usize>() // Sum of self
    }

    fn sum_complex(&self) -> usize {
        if self.children.is_empty() {
            return self.metadata.iter().sum::<usize>();
        }

        // Get all children that can be indexed by the metadata (accounting for indexing starting at 1)
        let sum_iter = self
            .metadata
            .iter()
            .filter(|&&m| m < self.children.len() + 1 && m != 0)
            .map(|&m| &self.children[m - 1]);

        sum_iter.map(|c| c.sum_complex()).sum()
    }
}

#[aoc_generator(day8)]
pub fn input_tree(input: &str) -> Node {
    let stream: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    Node::construct(&stream)
}

#[aoc(day8, part1)]
pub fn part1(root: &Node) -> usize {
    root.sum_metadata()
}

#[aoc(day8, part2)]
pub fn part2(root: &Node) -> usize {
    root.sum_complex()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_tree(TEST_STR)), 138);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_tree(TEST_STR)), 66);
    }
}
