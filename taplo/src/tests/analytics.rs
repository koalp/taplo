use crate::{analytics::NodeRef, syntax::SyntaxKind::*, util::coords::Mapper};
use lsp_types::Position;
use std::fs;

fn cargo_toml() -> String {
    fs::read_to_string("../test-data/analytics/_cargo.toml").unwrap()
}

#[test]
fn query_author() {
    let src = cargo_toml();
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let start = mapper.offset(Position::new(2, 12)).unwrap();
    let middle = mapper.offset(Position::new(2, 16)).unwrap();
    let end = mapper.offset(Position::new(2, 45)).unwrap();

    let start = dom.query_position(start);
    let middle = dom.query_position(middle);
    let end = dom.query_position(end);

    assert!(start.after.syntax.is_kind(STRING));
    assert!(start.after.syntax.text.unwrap() == r#""tamasf97 <tamasf97@outlook.com>""#);
    assert!(middle.after.syntax.is_kind(STRING));
    assert!(middle.after.syntax.text.unwrap() == r#""tamasf97 <tamasf97@outlook.com>""#);
    assert!(end.before.as_ref().unwrap().syntax.is_kind(STRING));
    assert!(end.before.unwrap().syntax.text.unwrap() == r#""tamasf97 <tamasf97@outlook.com>""#);
    assert!(!end.after.syntax.is_kind(STRING));
    assert!(end.after.syntax.text.as_ref().unwrap() != r#""tamasf97 <tamasf97@outlook.com>""#);
}

#[test]
fn query_package_field() {
    let src = cargo_toml();
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(6, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());

    let first_query_node = pos.after.nodes.last().copied().unwrap();

    let is_table = match first_query_node {
        NodeRef::Table(_) => true,
        _ => false,
    };

    assert!(is_table);

    let pos = mapper.offset(Position::new(7, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(!pos.is_completable());

    let before_node = pos.before.unwrap().nodes.last().copied().unwrap();
    let after_node = pos.after.nodes.last().copied().unwrap();

    assert!(before_node == first_query_node);
    assert!(before_node != after_node);
}

#[test]
fn query_lib_table() {
    let src = cargo_toml();
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(7, 5)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());

    let first_query_node = pos.after.nodes.last().copied().unwrap();

    let is_table = match first_query_node {
        NodeRef::Table(_) => true,
        _ => false,
    };

    assert!(is_table);

    let before_node = pos.before.unwrap().nodes.last().copied().unwrap();

    let is_key = match before_node {
        NodeRef::Key(_) => true,
        _ => false,
    };

    assert!(is_key);
}

#[test]
fn query_table_header() {
    let src = cargo_toml();
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(49, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(!pos.is_completable());

    let pos = mapper.offset(Position::new(49, 3)).unwrap();
    let pos = dom.query_position(pos);
    assert!(!pos.is_completable());

    let pos = mapper.offset(Position::new(49, 2)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());

    assert!(pos.after.syntax.expected_kind.unwrap() == KEY);
}

#[test]
fn query_incomplete_key() {
    let src = cargo_toml();
    let mapper = Mapper::new(&src);

    let dom = crate::parser::parse(&src).into_dom();

    let pos = mapper.offset(Position::new(51, 1)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.after.syntax.expected_kind.unwrap() == KEY);

    let key = pos.after.syntax.text.unwrap();
    assert!(key == "asd.bsd");

    let pos = mapper.offset(Position::new(51, 8)).unwrap();
    let pos = dom.query_position(pos);
    assert!(pos.is_completable());
    assert!(pos.before.as_ref().unwrap().syntax.expected_kind.unwrap() == KEY);
    assert!(pos.before.unwrap().syntax.text.unwrap() == key);
}
