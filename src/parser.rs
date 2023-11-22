use std::fs::read_to_string;
use std::collections::HashMap;
use std::str::FromStr;

use crate::instruction::Funct::*;
use crate::instruction::RegisterName::*;
use crate::instruction::OpCode::*;
use crate::instruction::*;

pub fn parse(filename: &str) -> Vec<Instruction> {
    let mut label_to_address = HashMap::new();
    let mut instructions = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line = line.to_string();
        let line_splited = line.split(":").collect::<Vec<&str>>();

        // 빈 행일 때 건너뛰기
        if line_splited[0].len() == 0 {
            continue;
        }

        match line_splited.len() {
            1 => {
                let code: Vec<_> = line_splited[0]
                    .split_whitespace().collect();

                instructions.push(parse_code(&code));
            },
            2 => {
                let label = line_splited[0].trim();
                let code: Vec<_> = line_splited[1]
                    .split_whitespace().collect();

                label_to_address.insert(label.to_string(), instructions.len());
                instructions.push(parse_code(&code));
            },
            _ => panic!("more than 1 colon in one line is not allowd"),
        }
    }

    for (k, v) in label_to_address {
        println!("{} {}", k, v);
    }

    // parse_address(&mut instructions, &label_to_address);

    instructions
}

fn parse_code(code: &[&str]) -> Instruction {
    let operand = &code[1..];
    match code[0] {
        "add" => parse_add(operand),
        "addu" => parse_addu(operand),
        "and" => parse_and(operand),
        "jr" => parse_jr(operand),
        "nor" => parse_nor(operand),
        "or" => parse_or(operand),
        "slt" => parse_slt(operand),
        "sltu" => parse_sltu(operand),
        "sll" => parse_sll(operand),
        "srl" => parse_srl(operand),
        "sub" => parse_sub(operand),
        "subu" => parse_subu(operand),
        "div" => parse_div(operand),
        "divu" => parse_divu(operand),
        "mfhi" => parse_mfhi(operand),
        "mflo" => parse_mflo(operand),
        "mult" => parse_mult(operand),
        "multu" => parse_multu(operand),
        "sra" => parse_sra(operand),
        "syscall" => parse_syscall(),

        "addi" => parse_addi(operand),
        "addiu" => parse_addiu(operand),
        "andi" => parse_andi(operand),
        "beq" => parse_beq(operand),
        "bne" => parse_bne(operand),
        "lbu" => parse_lbu(operand),
        "lhu" => parse_lhu(operand),
        "lui" => parse_lui(operand),
        "lw" => parse_lw(operand),
        "ori" => parse_ori(operand),
        "slti" => parse_slti(operand),
        "sltiu" => parse_sltiu(operand),
        "sb" => parse_sb(operand),
        "sh" => parse_sh(operand),
        "sw" => parse_sw(operand),

        "j" => parse_j(operand),
        "jal" => parse_jal(operand),

        _ => panic!("unknown operation: {}", code[0]),

    }
}

//////// parse r format ////////

fn parse_add(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let rt = parse_register(operand[2]);
    RFormat::new(Add, rs, rt, rd, 0)
}

fn parse_addu(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let rt = parse_register(operand[2]);
    RFormat::new(Addu, rs, rt, rd, 0)
}

fn parse_and(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let rt = parse_register(operand[2]);
    RFormat::new(And, rs, rt, rd, 0)
}

fn parse_jr(operand: &[&str]) -> Instruction {
    let rs = parse_register(operand[0]);
    RFormat::new(Jr, rs, ZERO, ZERO, 0)
}

fn parse_nor(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let rt = parse_register(operand[2]);
    RFormat::new(Nor, rs, rt, rd, 0)
}

fn parse_or(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let rt = parse_register(operand[2]);
    RFormat::new(Or, rs, rt, rd, 0)
}

fn parse_slt(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let rt = parse_register(operand[2]);
    RFormat::new(Slt, rs, rt, rd, 0)
}

fn parse_sltu(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let rt = parse_register(operand[2]);
    RFormat::new(Sltu, rs, rt, rd, 0)
}

fn parse_sll(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rt = parse_register(operand[1]);
    let shamt = u8::from_str(operand[2]).unwrap();
    RFormat::new(Sll, ZERO, rt, rd, shamt)
}

fn parse_srl(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rt = parse_register(operand[1]);
    let shamt = u8::from_str(operand[2]).unwrap();
    RFormat::new(Srl, ZERO, rt, rd, shamt)
}

fn parse_sub(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let rt = parse_register(operand[2]);
    RFormat::new(Sub, rs, rt, rd, 0)
}

fn parse_subu(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let rt = parse_register(operand[2]);
    RFormat::new(Subu, rs, rt, rd, 0)
}

fn parse_div(operand: &[&str]) -> Instruction {
    let rs = parse_register(operand[0]);
    let rt = parse_register(operand[1]);
    RFormat::new(Div, rs, rt, ZERO, 0)
}

fn parse_divu(operand: &[&str]) -> Instruction {
    let rs = parse_register(operand[0]);
    let rt = parse_register(operand[1]);
    RFormat::new(Divu, rs, rt, ZERO, 0)
}

fn parse_mfhi(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    RFormat::new(Mfhi, ZERO, ZERO, rd, 0)
}

fn parse_mflo(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    RFormat::new(Mflo, ZERO, ZERO, rd, 0)
}

fn parse_mult(operand: &[&str]) -> Instruction {
    let rs = parse_register(operand[0]);
    let rt = parse_register(operand[1]);
    RFormat::new(Mult, rs, rt, ZERO, 0)
}

fn parse_multu(operand: &[&str]) -> Instruction {
    let rs = parse_register(operand[0]);
    let rt = parse_register(operand[1]);
    RFormat::new(Multu, rs, rt, ZERO, 0)
}

fn parse_sra(operand: &[&str]) -> Instruction {
    let rd = parse_register(operand[0]);
    let rt = parse_register(operand[1]);
    let shamt = u8::from_str(operand[2]).unwrap();
    RFormat::new(Sra, ZERO, rt, rd, shamt)
}

fn parse_syscall() -> Instruction {
    RFormat::new(Syscall, ZERO, ZERO, ZERO, 0)
}

//////// parse i format ////////

fn parse_addi(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let immediate = i16::from_str(operand[2]).unwrap();
    IFormat::new(Addi, rs, rt, immediate)
}

fn parse_addiu(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let immediate = i16::from_str(operand[2]).unwrap();
    IFormat::new(Addiu, rs, rt, immediate)
}

fn parse_andi(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let immediate = i16::from_str(operand[2]).unwrap();
    IFormat::new(Andi, rs, rt, immediate)
}

fn parse_beq(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let label = operand[2].to_string();
    IFormat::new_label(Beq, rs, rt, label)
}

fn parse_bne(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let label = operand[2].to_string();
    IFormat::new_label(Bne, rs, rt, label)
}

fn parse_lbu(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let (offset, rs) = parse_offset(operand[1]);
    IFormat::new(Lbu, rs, rt, offset)
}

fn parse_lhu(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let (offset, rs) = parse_offset(operand[1]);
    IFormat::new(Lhu, rs, rt, offset)
}

fn parse_lui(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let immediate = i16::from_str(operand[1]).unwrap();
    IFormat::new(Lui, ZERO, rt, immediate)
}

fn parse_lw(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let (offset, rs) = parse_offset(operand[1]);
    IFormat::new(Lw, rs, rt, offset)
}

fn parse_ori(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let immediate = i16::from_str(operand[2]).unwrap();
    IFormat::new(Ori, rs, rt, immediate)
}

fn parse_slti(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let immediate = i16::from_str(operand[2]).unwrap();
    IFormat::new(Slti, rs, rt, immediate)
}

fn parse_sltiu(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let rs = parse_register(operand[1]);
    let immediate = i16::from_str(operand[2]).unwrap();
    IFormat::new(Slti, rs, rt, immediate)
}

fn parse_sb(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let (offset, rs) = parse_offset(operand[1]);
    IFormat::new(Sb, rs, rt, offset)
}

fn parse_sh(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let (offset, rs) = parse_offset(operand[1]);
    IFormat::new(Sh, rs, rt, offset)
}

fn parse_sw(operand: &[&str]) -> Instruction {
    let rt = parse_register(operand[0]);
    let (offset, rs) = parse_offset(operand[1]);
    IFormat::new(Sw, rs, rt, offset)
}

//////// parse j format ////////

fn parse_j(operand: &[&str]) -> Instruction {
    let label = operand[0].to_string();
    JFormat::new_label(Jump, label)
}

fn parse_jal(operand: &[&str]) -> Instruction {
    let label = operand[0].to_string();
    JFormat::new_label(Jal, label)
}

fn parse_register(register: &str) -> RegisterName {
    match register {
        "$zero" => ZERO,

        "$at" => AT,

        "$v0" => V0,
        "$v1" => V1,

        "$a0" => A0,
        "$a1" => A1,
        "$a2" => A2,
        "$a3" => A3,

        "$t0" => T0,
        "$t1" => T1,
        "$t2" => T2,
        "$t3" => T3,
        "$t4" => T4,
        "$t5" => T5,
        "$t6" => T6,
        "$t7" => T7,

        "$s0" => S0,
        "$s1" => S1,
        "$s2" => S2,
        "$s3" => S3,
        "$s4" => S4,
        "$s5" => S5,
        "$s6" => S6,
        "$s7" => S7,

        "$t8" => T8,
        "$t9" => T9,

        "$k8" => K0,
        "$k9" => K1,

        "$gp" => GP,
        "$sp" => SP,
        "$fp" => FP,
        "$ra" => RA,

        _ => panic!("unknown register: {}", register),
    }
}

fn parse_offset(offset: &str) -> (i16, RegisterName) {
    let offset: Vec<_> = offset.split(&['(', ')'][..]).collect();

    let rs = parse_register(offset[1]);
    let offset = i16::from_str(offset[0]).unwrap();
    (offset, rs)
}
