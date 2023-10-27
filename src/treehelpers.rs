use tree_sitter::{Node, Tree};

#[derive(Debug, Clone)]
pub struct TreeWrapper(pub Tree);
impl std::fmt::Display for TreeWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        pretty_display(f, self.0.root_node())?;
        Ok(())
    }
}

pub fn pretty_display(f: &mut std::fmt::Formatter<'_>, root: Node) -> std::fmt::Result {
    let mut stack = Vec::new();
    if !root.is_named() {
        return Ok(());
    }

    writeln!(f, "\nSyntax Tree: Child Count: {}", &root.child_count())?;

    stack.push((root, 0));
    while let Some((node, level)) = stack.pop() {
        let kind = node.kind();
        let start = node.start_position();
        let end = node.end_position();
        writeln!(
            f,
            "{}{} [{}, {}] - [{}, {}] ",
            " ".repeat(level * 2),
            kind,
            start.row,
            start.column,
            end.row,
            end.column
        )?;

        for i in (0..node.named_child_count()).rev() {
            let child = node.named_child(i).unwrap();
            stack.push((child, level + 1));
        }
    }

    Ok(())
}
