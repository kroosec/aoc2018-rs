use std::fs::read_to_string;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct Node {
    children: Vec<usize>, // Children positions in the tree
    metadata: Vec<usize>,
}

type LicenseTree = Vec<Node>;

fn insert_nodes<'a>(tree: &mut Vec<Node>, entries: &'a [usize]) -> &'a [usize] {
    assert!(entries.len() >= 2);

    let num_children = entries[0];
    let metadata = entries[1];

    let mut children = Vec::new();
    let mut remaining = &entries[2..];
    for _ in 0..num_children {
        remaining = insert_nodes(tree, remaining);
        // Child node was inserted at the end
        children.push(tree.len() - 1);
    }

    tree.push(Node {
        children,
        metadata: remaining[0..metadata].to_vec(),
    });

    &remaining[metadata..]
}

fn get_license_tree(input: &str) -> LicenseTree {
    let split = input
        .trim_end()
        .split(' ')
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    // Recursively insert nodes in tree, from the slice. The recursive function would tell its
    // parent how many splits it consumed, so that the parent knows where the next child is.
    let mut tree = Vec::new();
    let remaining = insert_nodes(&mut tree, &split);
    assert!(remaining.is_empty());

    tree
}

fn part1(tree: &LicenseTree) -> Result<()> {
    let sum: usize = tree
        .iter()
        .map(|node| node.metadata.iter().sum::<usize>())
        .sum();

    println!("Part1 {}", sum);

    Ok(())
}

fn calculate_value(tree: &LicenseTree, pos: usize) -> Result<usize> {
    let node = tree.get(pos).ok_or("Erroneous position ?")?;
    if node.children.is_empty() {
        return Ok(node.metadata.iter().sum::<usize>());
    }

    let mut sum = 0;
    for child in &node.metadata {
        if *child <= node.children.len() {
            sum += calculate_value(tree, node.children[*child - 1])?;
        }
    }

    Ok(sum)
}

fn part2(tree: &LicenseTree) -> Result<()> {
    let root_value = calculate_value(tree, tree.len() - 1)?;

    assert_eq!(33422, root_value);
    println!("Part2 {}", root_value);

    Ok(())
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;

    let tree = get_license_tree(&input);
    part1(&tree)?;
    part2(&tree)?;

    Ok(())
}
