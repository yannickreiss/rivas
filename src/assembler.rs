use core::panic;
use std::str::SplitWhitespace;

pub fn translate(input: &str) -> String {
    // catch empty lines and comments
    if (input == "") || (&input[0..1] == "#") || (&input[0..1] == ";") || (&input[0..2] == "--") {
        return String::from("");
    }

    let rv_: String;

    // Iterate over lines in input
    // Convert ABI into bare assembler (registers will be replaced later)
    //   replace brackets
    let mut rm_bracket: String = String::from(str::replace(input, "(", " "));
    rm_bracket = String::from(str::replace(&rm_bracket.to_string(), ")", ""));
    let instruction: String = String::from(str::replace(&rm_bracket, ",", " "));

    
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
    reg = if (register.starts_with('x') == true) || (register.starts_with('-') == true) {
        register[1..].to_string()
    } else {
        register.to_string()
    };

    // catch ABI register names
    let mut reg_id: i64;
    match reg.as_str() {
	"zero" => reg_id = 0,
	"ra" => reg_id = 1,
	"sp" => reg_id = 2,
	"gp" => reg_id = 3,
	"tp" => reg_id = 4,
	"t0" => reg_id = 5,
	"t1" => reg_id = 6,
	"t2" => reg_id = 7,
	"s0" | "fp" => reg_id = 8,
	"s1" => reg_id = 9,
	"a0" => reg_id = 10,
	"a1" => reg_id = 11,
	"a2" => reg_id = 12,
	"a3" => reg_id = 13,
	"a4" => reg_id = 14,
	"a5" => reg_id = 15,
	"a6" => reg_id = 16,
	"a7" => reg_id = 17,
	"s2" => reg_id = 18,
	"s3" => reg_id = 19,
	"s4" => reg_id = 20,
	"s5" => reg_id = 21,
	"s6" => reg_id = 22,
	"s7" => reg_id = 23,
	"s8" => reg_id = 24,
	"s9" => reg_id = 25,
	"s10" => reg_id = 26,
	"s11" => reg_id = 27,
	"t3" => reg_id = 28,
	"t4" => reg_id = 29,
	"t5" => reg_id = 30,
	"t6" => reg_id = 31,
	_ => reg_id = reg.parse::<i64>().expect("Parsing not successfull!"),
    }

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

    // 2-Bit complement
    if register.starts_with('-') == true {
        let mut compl_imm: String = String::from("");

        // negate
        for i in 0..reg_bin.len() {
            if reg_bin[i..i + 1] == String::from("1") {
                compl_imm = compl_imm.to_string() + &String::from("0");
            } else {
                compl_imm = compl_imm.to_string() + &String::from("1");
            }
        }

        // add one
        let mut j;
        for i in 0..reg_bin.len() {
            j = reg_bin.len() - i;
            if compl_imm[j - 1..j] == String::from("0") {
                compl_imm = compl_imm[0..j - 1].to_string()
                    + &String::from("1")
                    + &compl_imm[j..compl_imm.len()].to_string();
                break;
            } else {
                compl_imm = compl_imm[0..j - 1].to_string()
                    + &String::from("0")
                    + &compl_imm[j..compl_imm.len()].to_string();
            }
        }

        while compl_imm.len() < 32 {
            compl_imm = String::from("1") + &compl_imm.to_string();
        }

        compl_imm
    } else {
        reg_bin
    }
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

// S-Types
fn stype(instruction: &str) -> String {
    let mut operands: SplitWhitespace = instruction.split_whitespace();

    let opcode: &str = operands.next().unwrap();

    let funct3: &str = match opcode.to_uppercase().as_str() {
        "SB" => "000",
        "SH" => "001",
        "SW" => "010",
        _ => panic!("Unrecognized opcode: {}!", opcode),
    };

    let rs1: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("3 missing operands in instruction {}!", opcode),
    };

    let mut immediate: String = into_bin(operands.next().unwrap());
    while immediate.len() < 32 {
        immediate = String::from("0") + immediate.as_str();
    }

    let rs2: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("1 missing operand in instruction {}!", opcode),
    };

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }

    String::from("") + &immediate[20..27] + &rs1 + &rs2 + funct3 + &immediate[27..32] + "0100011"
}

// B-Types
fn btype(instruction: &str) -> String {
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

    let imm_value: &str = operands.next().unwrap();
    if imm_value.parse::<i32>().expect("Parsing not successfull!") % 4 != 0 {
        panic!("Immediate value is not a valid address (multiple of 4)!");
    }

    if imm_value.parse::<i32>().expect("Parsing not successfull!") > 2048 {
        panic!("Immediate value is exceeding valid address limits (greater than 2048)!");
    }

    let mut immediate: String = into_bin(imm_value);
    while immediate.len() < 32 {
        immediate = String::from("0") + &immediate.to_string();
    }

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }

    String::from("")
        + &immediate[19..20]
        + &immediate[21..27]
        + &rs2
        + &rs1
        + funct3
        + &immediate[27..31]
        + &immediate[20..21]
        + "1100011"
}

// U-Type are implemented
fn utype(instruction: &str) -> String {
    let mut operands: SplitWhitespace = instruction.split_whitespace();

    let opcode: &str = operands.next().unwrap();

    let rd: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("3 missing operands in instruction {}!", opcode),
    };

    let mut immediate: String = into_bin(operands.next().unwrap());
    while immediate.len() < 20 {
        immediate = String::from("0") + immediate.as_str();
    }

    let opcode_bin: &str = match opcode.to_uppercase().as_str() {
        "LUI" => "0110111",
        "AUIPC" => "0010111",
        _ => panic!("Uncecognized opcode: {}!", opcode),
    };

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }

    immediate + &rd + opcode_bin
}

// J-Type is not yet implemented!
fn jtype(instruction: &str) -> String {
    let mut operands: SplitWhitespace = instruction.split_whitespace();

    let opcode: &str = operands.next().unwrap();

    let rd: String = match operands.next() {
        Some(x) => into_bin(x).clone(),
        None => panic!("2 missing operands in instruction {}!", opcode),
    };

    let imm: String = match operands.next() {
        Some(x) => x.to_string(),
        None => panic!("1 missing operand in instruction {}!", opcode),
    };

    let mut immediate: String = into_bin(&imm);
    while immediate.len() < 32 {
        immediate = String::from("0") + immediate.as_str();
    }

    match operands.next() {
        Some(x) => panic!("Operand {} in instruction {} is not used!", x, opcode),
        None => (),
    }

    String::from("")
        + &immediate[11..12]
        + &immediate[21..31]
        + &immediate[20..21]
        + &immediate[12..20]
        + &rd
        + "1101111"
}
