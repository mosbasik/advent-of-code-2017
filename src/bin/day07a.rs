// http://adventofcode.com/2017/day/7

#[macro_use]
extern crate nom;
extern crate petgraph;

use nom::{alpha, digit};
use std::collections::HashMap;
use std::io::{self, BufRead};
use petgraph::Direction;
use petgraph::graphmap::DiGraphMap;


fn main() {
    // get puzzle input from sdin
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    // initialize tree and weight map
    let mut tree: DiGraphMap<&str, ()> = DiGraphMap::new();
    let mut weights: HashMap<&str, i32> = HashMap::new();

    // populate tree and weight map using puzzle input
    for line in lines.iter() {

        // parse line of input into its sigificant values
        let (program, weight, children_opt) = line_parser(line.as_bytes()).to_result().unwrap();

        // insert programs's own data into tree and weight map
        if !tree.contains_node(program) {
            tree.add_node(program);
            weights.insert(program, weight);
        }

        // insert programs's children's data into structures (if applicable)
        match children_opt {
            None => (),
            Some(children) => {
                for child in children.iter() {
                    if !tree.contains_node(child) {
                        tree.add_node(child);
                    }
                    tree.add_edge(program, child, ());
                }
            }
        }
    }

    // find root of tree
    println!("{}", get_root(&tree));
}


fn get_root<'a>(tree: &'a DiGraphMap<&str, ()>) -> &'a str {
    get_root_recursive(tree, tree.nodes().next().unwrap())
}

fn get_root_recursive<'a>(tree: &'a DiGraphMap<&str, ()>, node: &'a str) -> &'a str {
    let mut parents: Vec<&str> = tree.neighbors_directed(node, Direction::Incoming).collect();
    match parents.len() {
        0 => node,
        1 => get_root_recursive(tree, parents.pop().unwrap()),
        _ => panic!("\"{}\" has multiple parents: {:?}", node, parents),
    }
}


named!(program_parser<&str>,
    map_res!(
        alpha,
        std::str::from_utf8
    )
);

named!(integer_parser<i32>,
    map_res!(
        map_res!(
            digit,
            std::str::from_utf8
        ),
        std::str::FromStr::from_str
    )
);

named!(weight_parser<i32>,
    delimited!(
        tag!("("),
        integer_parser,
        tag!(")")
    )
);

named!(arrow_parser,
    tag!("->")
);

named!(programs_vec_parser<Vec<&str>>, dbg!(
    separated_list_complete!(
        char!(','),
        ws!(program_parser)
    )
));

named!(line_parser<(&str, i32, Option<Vec<&str>>)>,
    do_parse!(
        program: ws!(program_parser) >>
        weight: ws!(weight_parser) >>
        children_opt: opt!(
            complete!(
                do_parse!(
                    ws!(arrow_parser) >>
                    children: ws!(programs_vec_parser) >>
                    (children)
                )
            )
        ) >>
        (program, weight, children_opt)
    )
);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_parser() {
        assert_eq!(program_parser(b"aaa").to_result().unwrap(), "aaa");
    }

    #[test]
    fn test_integer_parser() {
        assert_eq!(integer_parser(b"100").to_result().unwrap(), 100);
    }

    #[test]
    fn test_weight_parser() {
        assert_eq!(weight_parser(b"(100)").to_result().unwrap(), 100);
    }

    #[test]
    fn test_arrow_parser() {
        assert_eq!(arrow_parser(b"->").to_result().unwrap(), b"->");
    }

    #[test]
    fn test_programs_vec_parser() {
        assert_eq!(programs_vec_parser(b"aa, bb, cc").to_result().unwrap(), vec!["aa", "bb", "cc"]);
    }

    #[test]
    fn test_line_parser_without_children() {
        assert_eq!(line_parser(b"pbga (66)").to_result().unwrap(), ("pbga", 66, None));
    }

    #[test]
    fn test_line_parser_with_children() {
        let line = b"aaa (100) -> bbb, ccc, ddd";
        let (program, weight, children) = line_parser(line).to_result().unwrap();
        assert_eq!(program, "aaa");
        assert_eq!(weight, 100);
        assert_eq!(children, Some(vec!["bbb", "ccc", "ddd"]));
    }
}
