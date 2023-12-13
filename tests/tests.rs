use libaheui::hangul::*;
use libaheui::instruction::*;
use libaheui::component::*;
use libaheui::AheuiState;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[test]
fn test_hangul_creation() {
    let han = '한';
    let han_struct = HangulSyllable {
        onset: HangulOnset::Hieut,
        vowel: HangulVowel::A,
        coda: HangulCoda::Nieun,
    };

    let han_constructed = HangulSyllable::from_char(han);

    assert!(han_constructed.is_some());
    assert_eq!(han_constructed.unwrap(), han_struct);
}

#[test]
fn test_invalid_hangul() {
    let fake = 'k';
    let han_constructed = HangulSyllable::from_char(fake);

    assert_eq!(han_constructed, None);
}

#[test]
fn test_basic_operator() {
    let mah = '맣';
    let deol = '덜';
    let beugs = '씋';

    let mah_operator = AheuiInstruction::from_char(mah);
    let deol_operator = AheuiInstruction::from_char(deol);
    let beugs_operator = AheuiInstruction::from_char(beugs);
    // println!("{:?}", mah_operator);
    // println!("{:?}", deol_operator);
    // println!("{:?}", beugs_operator);
}

#[test]
fn test_construct_program() {
    let mut f = File::open("tests/hello.ah").unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer);

    let program = AheuiProgram::from_str(&buffer);
    println!("{:#?}", program);
}

#[test]
fn test_single_op_program() {
    let basic_program = "발희";

    let mut program = AheuiState::init(&basic_program);
    println!("Initial state of program:\nRunning: {:?}\nPosition: {:?}\nDirection: {:?}\nStorage: {:#?}", !program.terminated, program.position, program.direction, program.storages[0]);

    program.step().unwrap();
    println!("After first step:\nRunning: {:?}\nPosition: {:?}\nDirection: {:?}\nStorage: {:#?}", !program.terminated, program.position, program.direction, program.storages[0]);

    program.step().unwrap();
    println!("After second step:\nRunning: {:?}\nPosition: {:?}\nDirection: {:?}\nStorage: {:#?}", !program.terminated, program.position, program.direction, program.storages[0]);
}

#[test]
fn test_pa() {
    let pa = "발반파희";

    let mut program = AheuiState::init(&pa);
    println!("Program is {:#?}", program.program);
    println!("Current position is {:?}\nCharacter: {}\nStorage: {:?}", program.position, program.program.get_instruction(&program.position).unwrap().character, program.storages[program.storage_index]);
}

#[test]
fn test_hello_world() {
    let mut f = File::open("tests/hello.ah").unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer);

    //let buffer = "밯밦밝다다맣희";

    let mut stdin = io::stdin();
    let mut program = AheuiState::init(&buffer);
    //println!("Program is {:#?}", program.program);
    while !program.terminated {
        //println!("Current position is {:?}\nCharacter: {}\nStorage: {:?}", program.position, program.program.get_instruction(&program.position).unwrap().character, program.storages[program.storage_index]);
        //let _ = stdin.read(&mut [0u8]).unwrap();
        program.step().unwrap();
    }
}
