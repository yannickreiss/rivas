use core::panic;
use std::str::SplitWhitespace;

// Instruction
// contain all instruction values
struct Instruction {
    opcode: String,
    rd: String,
    rs1: String,
    rs2: String,
    funct3: String,
    funct7: String,
    immediate: String,
}

pub fn translate(input: &str) -> String {
    let rv_: String;

    // Iterate over lines in input
    let instruction: String = String::from(str::replace(input, ",", " "));
    println!("Instruction: {}", instruction);
    let mut token = instruction.split_whitespace();

    // split instruction and call functions for different types
    let opcode: &str = token.next().unwrap();
    match opcode.to_uppercase().as_str() {
        "ADD" | "SUB" | "SLL" | "SLT" | "SLTU" | "XOR" | "SRL" | "SRA" | "OR" | "AND" => {
            rv_ = rtype(&instruction)
        } // R-Type
        "JALR" | "ADDI" | "SLTI" | "SLTIU" | "XORI" | "ORI" | "ANDI" | "SLLI" | "SRLI" | "SRAI" => {
            rv_ = itype(&instruction)
        } // I-Type
        "SB" | "SH" | "SW" => rv_ = stype(&instruction), // S-Type
        "BEQ" | "BNE" | "BLT" | "BGE" | "BLTU" | "BGEU" => rv_ = btype(&instruction), // B-Type
        "LUI" | "AUIPC" => rv_ = utype(&instruction),    // U-Type
        "JAL" => rv_ = jtype(&instruction),
        _ => panic!("Unknown opcode: {}!", opcode),
    };

    rv_
}

fn into_bin(register: &str) -> String {
    let reg: String;
    reg = if register.starts_with('x') == true {
        register[1..].to_string()
    } else {
        register.to_string()
    };

    let mut reg_id = reg.parse::<u32>().expect("Parsing not successfull!");

    let mut reg_bin: String = String::from("");
    while reg_id > 0 {
        if reg_id % 2 == 1 {
            reg_bin = String::from("1") + &reg_bin.to_string();
        } else {
            reg_bin = String::from("0") + &reg_bin.to_string();
        }
        reg_id /= 2;
    }
    while reg_bin.len() < 5 {
        reg_bin = String::from("0") + reg_bin.as_str();
    }
    reg_bin
}

fn rtype(instruction: &str) -> String {
    let mut operands: SplitWhitespace = instruction.split_whitespace();
    let opcode: &str = operands.next().unwrap();
    let funct3: &str = match opcode.to_uppercase().as_str() {
        "ADD" | "SUB" => "000",
        "SLL" => "001",
        "SLT" => "010",
        "SLTU" => "011",
        "XOR" => "100",
        "SRL" => "101",
        "SRA" => "101",
        "OR" => "110",
        "AND" => "111",
        _ => panic!("Unrecognized opcode: {}!", opcode),
    };
    let funct7: &str = match opcode.to_uppercase().as_str() {
        "SUB" | "SRA" => "0100000",
        _ => "0000000",
    };
    let rd: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("3 missing operands in instruction {}!", opcode),
    };
    let rs1: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("2 missing operands in instruction {}!", opcode),
    };
    let rs2: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("Missing operand in instruction {}!", opcode),
    };

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }

    String::from(funct7) + &rs2 + &rs1 + funct3 + &rd + "0110011"
}

fn itype(instruction: &str) -> String {
    let mut operands: SplitWhitespace = instruction.split_whitespace();

    let opcode: &str = operands.next().unwrap();

    let funct3: &str = match opcode.to_uppercase().as_str() {
        "JALR" | "LB" | "ADDI" => "000",
        "LH" => "001",
        "LW" | "SLTI" => "010",
        "SLTIU" => "011",
        "LBU" | "XORI" => "100",
        "LHU" => "101",
        "ORI" => "110",
        "ANDI" => "111",
        _ => panic!("Unrecognized opcode: {}!", opcode),
    };

    let rd: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("3 missing operands in instruction {}!", opcode),
    };

    let rs1: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("2 missing operands in instruction {}!", opcode),
    };

    let mut immediate: String = into_bin(operands.next().unwrap());
    while immediate.len() < 12 {
        immediate = String::from("0") + immediate.as_str();
    }

    let opcode_bin: &str = match opcode.to_uppercase().as_str() {
        "JALR" => "1100111",
        "LB" | "LH" | "LW" | "LBU" | "LHU" => "0000011",
        "ADDI" | "SLTI" | "SLTIU" | "XORI" | "ORI" | "ANDI" => "0010011",
        _ => panic!("Uncecognized opcode: {}!", opcode),
    };

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }

    immediate + &rs1 + funct3 + &rd + opcode_bin
}

// Still unfinished. DO NOT USE UNTIL Tested!
fn stype(instruction: &str) -> String {
    panic!("S-Types not yet tested!!!");
    let mut operands: SplitWhitespace = instruction.split_whitespace();

    let opcode: &str = operands.next().unwrap();
    println!("OPC:\t{}", opcode);

    let funct3: &str = match opcode.to_uppercase().as_str() {
        "SB" => "000",
        "SH" => "001",
        "SW" => "010",
        _ => panic!("Unrecognized opcode: {}!", opcode),
    };
    println!("FN3:\t{}", funct3);

    let rs1: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("2 missing operands in instruction {}!", opcode),
    };
    println!("rs1:\t{}", rs1);

    let rs2: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("1 missing operand in instruction {}!", opcode),
    };
    println!("rs2:\t{}", rs2);

    let mut immediate: String = into_bin(operands.next().unwrap());
    while immediate.len() < 12 {
        immediate = String::from("0") + immediate.as_str();
    }
    println!("imm:\t{}", immediate);

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }

    String::from("") + &immediate[5..11] + &rs2 + &rs1 + funct3 + &immediate[0..4] + "0100011"
}

// B-Types not yet implemented!
fn btype(instruction: &str) -> String {
    panic!("B-Types not yet implemented!!");
    let mut operands: SplitWhitespace = instruction.split_whitespace();

    let opcode: &str = operands.next().unwrap();

    let funct3: &str = match opcode.to_uppercase().as_str() {
        "BEQ" => "000",
        "BNE" => "001",
        "BLT" => "100",
        "BGE" => "101",
        "BLTU" => "110",
        "BGEU" => "111",
        _ => panic!("Unrecognized opcode: {}!", opcode),
    };

    let rs1: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("3 missing operands in instruction {}!", opcode),
    };

    let rs2: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("2 missing operands in instruction {}!", opcode),
    };

    let mut immediate: String = into_bin(operands.next().unwrap());
    while immediate.len() < 12 {
        immediate = String::from("0") + immediate.as_str();
    }

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }
    println!("Immediate: {}", immediate);
    String::from("") + &immediate[..1] + &immediate[1..7] + &rs2 + &rs1 + funct3 + &immediate[7..] + "1100011"
}

// U-Type not yet implemented
fn utype(instruction: &str) -> String {
    panic!("U-Types are not yet implemented!!");
    let mut operands: SplitWhitespace = instruction.split_whitespace();

    let opcode: &str = operands.next().unwrap();

    let funct3: &str = match opcode.to_uppercase().as_str() {
        "JALR" | "LB" | "ADDI" => "000",
        "LH" => "001",
        "LW" | "SLTI" => "010",
        "SLTIU" => "011",
        "LBU" | "XORI" => "100",
        "LHU" => "101",
        "ORI" => "110",
        "ANDI" => "111",
        _ => panic!("Unrecognized opcode: {}!", opcode),
    };

    let rd: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("3 missing operands in instruction {}!", opcode),
    };

    let rs1: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("2 missing operands in instruction {}!", opcode),
    };

    let mut immediate: String = into_bin(operands.next().unwrap());
    while immediate.len() < 12 {
        immediate = String::from("0") + immediate.as_str();
    }

    let opcode_bin: &str = match opcode.to_uppercase().as_str() {
        "JALR" => "1100111",
        "LB" | "LH" | "LW" | "LBU" | "LHU" => "0000011",
        "ADDI" | "SLTI" | "SLTIU" | "XORI" | "ORI" | "ANDI" => "0010011",
        _ => panic!("Uncecognized opcode: {}!", opcode),
    };

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }

    immediate + &rs1 + funct3 + &rd + opcode_bin
}

// J-Type is not yet implemented!
fn jtype(instruction: &str) -> String {
    panic!("J-Type is not yet implemented!!!");
    let mut operands: SplitWhitespace = instruction.split_whitespace();

    let opcode: &str = operands.next().unwrap();

    let funct3: &str = match opcode.to_uppercase().as_str() {
        "JALR" | "LB" | "ADDI" => "000",
        "LH" => "001",
        "LW" | "SLTI" => "010",
        "SLTIU" => "011",
        "LBU" | "XORI" => "100",
        "LHU" => "101",
        "ORI" => "110",
        "ANDI" => "111",
        _ => panic!("Unrecognized opcode: {}!", opcode),
    };

    let rd: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("3 missing operands in instruction {}!", opcode),
    };

    let rs1: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("2 missing operands in instruction {}!", opcode),
    };

    let mut immediate: String = into_bin(operands.next().unwrap());
    while immediate.len() < 12 {
        immediate = String::from("0") + immediate.as_str();
    }

    let opcode_bin: &str = match opcode.to_uppercase().as_str() {
        "JALR" => "1100111",
        "LB" | "LH" | "LW" | "LBU" | "LHU" => "0000011",
        "ADDI" | "SLTI" | "SLTIU" | "XORI" | "ORI" | "ANDI" => "0010011",
        _ => panic!("Uncecognized opcode: {}!", opcode),
    };

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }

    immediate + &rs1 + funct3 + &rd + opcode_bin
}
