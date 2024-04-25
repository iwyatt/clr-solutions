use assert_cmd::Command;
use std::fs::{self, File};
use std::io::prelude::*;
use clap::Parser;
use headr::Args;

const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";
const TWO: &str = "./tests/inputs/two.txt";
const THREE: &str = "./tests/inputs/three.txt";
const TWELVE: &str = "./tests/inputs/twelve.txt";

// --------------------------------------------------
#[test]
fn test_matching_output_one() {
    let headr_output = Command::cargo_bin(PRG).unwrap().args(["/home/ike/projects/clr-solutions/headr/tests/inputs/one.txt"]).unwrap();
    let head_output = Command::new("head").arg("/home/ike/projects/clr-solutions/headr/tests/inputs/one.txt").unwrap();
    assert_eq!(headr_output, head_output);
}