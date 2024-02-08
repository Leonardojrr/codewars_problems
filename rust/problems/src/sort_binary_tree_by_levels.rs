// kata: https://www.codewars.com/kata/52bef5e3588c56132c0003bc/train/rust

mod preloaded;
use preloaded::Node;

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut numbers = vec![root.value];
    let mut next_levels = vec![&root.left, &root.right];

    loop {
        if next_levels.is_empty() {
            break;
        }

        let mut new_next_levels: Vec<&Option<Box<Node>>> = vec![];

        for node in &next_levels {
            match node {
                Some(node) => {
                    numbers.push(node.value);
                    new_next_levels.append(&mut vec![&node.left, &node.right])
                }

                None => {}
            }
        }

        next_levels = new_next_levels;
    }

    return numbers;
}
