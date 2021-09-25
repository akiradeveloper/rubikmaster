//! Parse rotation sequence like (RUR')U'(R'FR)F'
//!
//! Syntax:
//! - Move -> R|L|U|D|F|B| ...
//! - Double -> 2 | ε
//! - Prime -> ' | ε
//! - Rep -> Double Prime
//! - Command -> Move Rep
//! - Group -> ( Command+ ) Rep
//! - Elem -> Command | Group
//! - Seq -> Elem+

use crate::{Command, Elem, Move};

use nom::branch::alt;
use nom::character::complete::{char, one_of};
use nom::combinator::{all_consuming, map};
use nom::multi::{many0, many1, many_m_n};
use nom::sequence::{delimited, pair};
use nom::IResult;

fn parse_move(i: &str) -> IResult<&str, Move> {
    map(one_of("RLUDFBrludfbMESxyz"), |c| match c {
        'R' => Move::R,
        'L' => Move::L,
        'U' => Move::U,
        'D' => Move::D,
        'F' => Move::F,
        'B' => Move::B,
        'r' => Move::r,
        'l' => Move::l,
        'u' => Move::u,
        'd' => Move::d,
        'f' => Move::f,
        'b' => Move::b,
        'M' => Move::M,
        'E' => Move::E,
        'S' => Move::S,
        'x' => Move::x,
        'y' => Move::y,
        'z' => Move::z,
        _ => unreachable!(),
    })(i)
}
fn parse_double(i: &str) -> IResult<&str, bool> {
    map(many_m_n(0, 1, char('2')), |v| v.len() > 0)(i)
}
fn parse_prime(i: &str) -> IResult<&str, bool> {
    map(many_m_n(0, 1, char('\'')), |v| v.len() > 0)(i)
}
struct Rep(i8);
fn parse_rep(i: &str) -> IResult<&str, Rep> {
    map(pair(parse_double, parse_prime), |(double, prime)| {
        let mut rep = 1;
        if double {
            rep = 2;
        }
        if prime {
            rep *= -1;
        }
        Rep(rep)
    })(i)
}
fn parse_command(i: &str) -> IResult<&str, Command> {
    let f = pair(parse_move, parse_rep);
    map(f, |(mov, rep)| Command(mov, rep.0))(i)
}
fn parse_group(i: &str) -> IResult<&str, (Vec<Command>, i8)> {
    let p1 = char('(');
    let p2 = many1(parse_command);
    let p3 = char(')');
    let f = delimited(p1, p2, p3);
    let f = pair(f, parse_rep);
    map(f, |(xs, rep)| (xs, rep.0))(i)
}
fn parse_elem(i: &str) -> IResult<&str, Elem> {
    let p1 = map(parse_command, |x| Elem::One(x));
    let p2 = map(parse_group, |(xs, rep)| Elem::Group(xs, rep));
    alt((p1, p2))(i)
}

pub fn parse(i: &str) -> IResult<&str, Vec<Elem>> {
    let p = many0(parse_elem);
    all_consuming(p)(i)
}
#[test]
fn test_parse() {
    use Elem::*;
    use Move::*;
    assert_eq!(parse("").unwrap().1, vec![]);
    assert_eq!(parse("R").unwrap().1, vec![One(Command(R, 1))]);
    assert_eq!(parse("R2'").unwrap().1, vec![One(Command(R, -2))]);
    assert!(parse("R'2").is_err());
    assert_eq!(
        parse("(RR')'").unwrap().1,
        vec![Group(vec![Command(R, 1), Command(R, -1)], -1)]
    );
    assert!(parse("RUR'U'").is_ok());
    assert!(parse("R2D(R'U2R)D'(R'U2R')").is_ok());
    assert!(parse("(RU')(RU)2(RU')R'U'R2").is_ok());
    assert!(parse("(R2'U)(RUR')(U'R'U')(R'UR')").is_ok());
    assert!(parse("RNA").is_err());
}
