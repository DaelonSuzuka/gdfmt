use tree_sitter::Parser;
use tree_sitter_gdscript::language;

mod treehelpers;

use treehelpers::TreeWrapper;


fn main() {
    let mut parser = Parser::new();
    parser.set_language(language()).unwrap();

    let source_code = "var a = 'a'";
    let tree = parser.parse(source_code, None).unwrap();
    // let root_node = tree.root_node();

    tree.

    println!("{}", TreeWrapper(tree));
}
