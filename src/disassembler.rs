use crate::ast;

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

type R = ast::Register;
type Vu = ast::VuRegister;

#[rustfmt::skip]
pub fn disassemble(bytes: Vec<u32>) -> Vec<ast::Instruction> {
    let mut insts = vec![];

    for (index, inst) in bytes.iter().copied().enumerate() {
        let op = inst >> 26;
        let rs = (inst >> 21) & 0x1F;
        let rt = (inst >> 16) & 0x1F;
        let rd = (inst >> 11) & 0x1F;
        let sa = (inst >> 6) & 0x1F;
        let code = (inst >> 6) & 0xFFFFF;
        let funct = inst & 0x3F;
        let imm = inst & 0xFFFF;
        let target = ((inst & 0x3FFFFFF) << 2) | 0x80000000;
        let vd = (inst >> 6)&0x1F;
        let vs = (inst >> 11)&0x1F;
        let vt = (inst >> 16)&0x1F;
        let de = (inst >> 11)&0x1F;
        let e = (inst >> 21)&0xF;
        
        let i = match op {
            0 => {
                match funct {
                    0 => ast::Instruction::Register { op: ast::RTypeOp::Sll, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa },
                    2 => ast::Instruction::Register { op: ast::RTypeOp::Srl, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa },
                    3 => ast::Instruction::Register { op: ast::RTypeOp::Sra, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa },
                    4 => ast::Instruction::Register { op: ast::RTypeOp::Sllv, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    6 => ast::Instruction::Register { op: ast::RTypeOp::Srlv, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    7 => ast::Instruction::Register { op: ast::RTypeOp::Srav, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    8 => ast::Instruction::Register { op: ast::RTypeOp::Jr, rs: R::try_from(rs).unwrap(), rt: R::null(), rd: R::null(), sa: 0 },
                    9 => ast::Instruction::Register { op: ast::RTypeOp::Jalr, rs: R::try_from(rs).unwrap(), rt: R::null(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    12 => ast::Instruction::Register { op: ast::RTypeOp::Syscall, rs: R::null(), rt: R::null(), rd: R::null(), sa: code },
                    13 => ast::Instruction::Register { op: ast::RTypeOp::Break, rs: R::null(), rt: R::null(), rd: R::null(), sa: code },
                    15 => ast::Instruction::Register { op: ast::RTypeOp::Sync, rs: R::null(), rt: R::null(), rd: R::null(), sa: 0 },
                    16 => ast::Instruction::Register { op: ast::RTypeOp::Mfhi, rs: R::null(), rt: R::null(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    17 => ast::Instruction::Register { op: ast::RTypeOp::Mthi, rs: R::try_from(rs).unwrap(), rt: R::null(), rd: R::null(), sa: 0 },
                    18 => ast::Instruction::Register { op: ast::RTypeOp::Mflo, rs: R::null(), rt: R::null(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    19 => ast::Instruction::Register { op: ast::RTypeOp::Mtlo, rs: R::try_from(rs).unwrap(), rt: R::null(), rd: R::null(), sa: 0 },
                    20 => ast::Instruction::Register { op: ast::RTypeOp::Dsllv, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    22 => ast::Instruction::Register { op: ast::RTypeOp::Dsrlv, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    23 => ast::Instruction::Register { op: ast::RTypeOp::Dsrav, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    24 => ast::Instruction::Register { op: ast::RTypeOp::Mult, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    25 => ast::Instruction::Register { op: ast::RTypeOp::Multu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    26 => ast::Instruction::Register { op: ast::RTypeOp::Div, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    27 => ast::Instruction::Register { op: ast::RTypeOp::Divu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    28 => ast::Instruction::Register { op: ast::RTypeOp::Dmult, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    29 => ast::Instruction::Register { op: ast::RTypeOp::Dmultu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    30 => ast::Instruction::Register { op: ast::RTypeOp::Ddiv, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    31 => ast::Instruction::Register { op: ast::RTypeOp::Ddivu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    32 => ast::Instruction::Register { op: ast::RTypeOp::Add, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    33 => ast::Instruction::Register { op: ast::RTypeOp::Addu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    34 => ast::Instruction::Register { op: ast::RTypeOp::Sub, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    35 => ast::Instruction::Register { op: ast::RTypeOp::Subu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    36 => ast::Instruction::Register { op: ast::RTypeOp::And, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    37 => ast::Instruction::Register { op: ast::RTypeOp::Or, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    38 => ast::Instruction::Register { op: ast::RTypeOp::Xor, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    39 => ast::Instruction::Register { op: ast::RTypeOp::Nor, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    42 => ast::Instruction::Register { op: ast::RTypeOp::Slt, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    43 => ast::Instruction::Register { op: ast::RTypeOp::Sltu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    44 => ast::Instruction::Register { op: ast::RTypeOp::Dadd, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    45 => ast::Instruction::Register { op: ast::RTypeOp::Daddu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    46 => ast::Instruction::Register { op: ast::RTypeOp::Dsub, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    47 => ast::Instruction::Register { op: ast::RTypeOp::Dsubu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0 },
                    48 => ast::Instruction::Register { op: ast::RTypeOp::Tge, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    49 => ast::Instruction::Register { op: ast::RTypeOp::Tgeu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    50 => ast::Instruction::Register { op: ast::RTypeOp::Tlt, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    51 => ast::Instruction::Register { op: ast::RTypeOp::Tltu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    52 => ast::Instruction::Register { op: ast::RTypeOp::Teq, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    54 => ast::Instruction::Register { op: ast::RTypeOp::Tne, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: 0 },
                    56 => ast::Instruction::Register { op: ast::RTypeOp::Dsll, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa },
                    58 => ast::Instruction::Register { op: ast::RTypeOp::Dsrl, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa },
                    59 => ast::Instruction::Register { op: ast::RTypeOp::Dsra, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa },
                    60 => ast::Instruction::Register { op: ast::RTypeOp::Dsll32, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa },
                    62 => ast::Instruction::Register { op: ast::RTypeOp::Dsrl32, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa },
                    63 => ast::Instruction::Register { op: ast::RTypeOp::Dsra32, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa },
                    e => panic!("Invalid R-type instruction: {} at: {:x} (ins {inst:x})", e, 4 * index),
                }
            }
            1 => match rt {
                0 => ast::Instruction::Immediate { op: ast::ITypeOp::Bltz, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                1 => ast::Instruction::Immediate { op: ast::ITypeOp::Bgez, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                2 => ast::Instruction::Immediate { op: ast::ITypeOp::Bltzl, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                3 => ast::Instruction::Immediate { op: ast::ITypeOp::Bgezl, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                8 => ast::Instruction::Immediate { op: ast::ITypeOp::Tgei, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                9 => ast::Instruction::Immediate { op: ast::ITypeOp::Tgeiu, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                10 => ast::Instruction::Immediate { op: ast::ITypeOp::Tlti, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                11 => ast::Instruction::Immediate { op: ast::ITypeOp::Tltiu, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                12 => ast::Instruction::Immediate { op: ast::ITypeOp::Teqi, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                14 => ast::Instruction::Immediate { op: ast::ITypeOp::Tnei, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                16 => ast::Instruction::Immediate { op: ast::ITypeOp::Bltzal, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                17 => ast::Instruction::Immediate { op: ast::ITypeOp::Bgezal, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                18 => ast::Instruction::Immediate { op: ast::ITypeOp::Bltzall, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                19 => ast::Instruction::Immediate { op: ast::ITypeOp::Bgezall, rs: R::try_from(rs).unwrap(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                e => panic!("Invalid I-type instruction: {} at: {:x} (ins {inst:x})", e, 4 * index),
            }
            2 => ast::Instruction::Jump { op: ast::JTypeOp::J, target: ast::Target(target) },
            3 => ast::Instruction::Jump { op: ast::JTypeOp::Jal, target: ast::Target(target) },
            4 => ast::Instruction::Immediate { op: ast::ITypeOp::Beq, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            5 => ast::Instruction::Immediate { op: ast::ITypeOp::Bne, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            6 => ast::Instruction::Immediate { op: ast::ITypeOp::Blez, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            7 => ast::Instruction::Immediate { op: ast::ITypeOp::Bgtz, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            8 => ast::Instruction::Immediate { op: ast::ITypeOp::Addi, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            9 => ast::Instruction::Immediate { op: ast::ITypeOp::Addiu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            10 => ast::Instruction::Immediate { op: ast::ITypeOp::Slti, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            11 => ast::Instruction::Immediate { op: ast::ITypeOp::Sltiu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            12 => ast::Instruction::Immediate { op: ast::ITypeOp::Andi, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            13 => ast::Instruction::Immediate { op: ast::ITypeOp::Ori, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            14 => ast::Instruction::Immediate { op: ast::ITypeOp::Xori, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            15 => ast::Instruction::Immediate { op: ast::ITypeOp::Lui, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            16 => match (rs, rt) {
                (0, _) => ast::Instruction::Register { op: ast::RTypeOp::Mfc0, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (1, _) => ast::Instruction::Register { op: ast::RTypeOp::Dmfc0, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (2, _) => ast::Instruction::Register { op: ast::RTypeOp::Cfc0, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (4, _) => ast::Instruction::Register { op: ast::RTypeOp::Mtc0, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (5, _) => ast::Instruction::Register { op: ast::RTypeOp::Dmtc0, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (6, _) => ast::Instruction::Register { op: ast::RTypeOp::Ctc0, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (8, 0) => ast::Instruction::Immediate { op: ast::ITypeOp::Bc0f, rs: R::null(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                (8, 1) => ast::Instruction::Immediate { op: ast::ITypeOp::Bc0t, rs: R::null(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                (8, 2) => ast::Instruction::Immediate { op: ast::ITypeOp::Bc0fl, rs: R::null(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                (8, 3) => ast::Instruction::Immediate { op: ast::ITypeOp::Bc0tl, rs: R::null(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                (_, _) => match funct {
                    1 => ast::Instruction::Register { op: ast::RTypeOp::Tlbr, rs: R::null(), rt: R::null(), rd: R::null(), sa: 0 },
                    2 => ast::Instruction::Register { op: ast::RTypeOp::Tlbwi, rs: R::null(), rt: R::null(), rd: R::null(), sa: 0 },
                    6 => ast::Instruction::Register { op: ast::RTypeOp::Tlbwr, rs: R::null(), rt: R::null(), rd: R::null(), sa: 0 },
                    8 => ast::Instruction::Register { op: ast::RTypeOp::Tlbp, rs: R::null(), rt: R::null(), rd: R::null(), sa: 0 },
                    24 => ast::Instruction::Register { op: ast::RTypeOp::Eret, rs: R::null(), rt: R::null(), rd: R::null(), sa: 0 },
                    _ => panic!("Invalid R-type instruction: {} at: {:x} (ins {inst:x})", funct, 4 * index),
                },
            }
            17 => match (rs, rt) {
                (0, _) => ast::Instruction::Register { op: ast::RTypeOp::Mfc1, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (1, _) => ast::Instruction::Register { op: ast::RTypeOp::Dmfc1, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (2, _) => ast::Instruction::Register { op: ast::RTypeOp::Cfc1, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (4, _) => ast::Instruction::Register { op: ast::RTypeOp::Mtc1, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (5, _) => ast::Instruction::Register { op: ast::RTypeOp::Dmtc1, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (6, _) => ast::Instruction::Register { op: ast::RTypeOp::Ctc1, rs: R::null(), rt: R::try_from(rt).unwrap(), rd: R::try_from(rd).unwrap(), sa: 0},
                (8, 0) => ast::Instruction::Immediate { op: ast::ITypeOp::Bc1f, rs: R::null(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                (8, 1) => ast::Instruction::Immediate { op: ast::ITypeOp::Bc1t, rs: R::null(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                (8, 2) => ast::Instruction::Immediate { op: ast::ITypeOp::Bc1fl, rs: R::null(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                (8, 3) => ast::Instruction::Immediate { op: ast::ITypeOp::Bc1tl, rs: R::null(), rt: R::null(), imm: ast::Immediate(imm as u16) },
                (16, _) => match funct {
                    0 => ast::Instruction::Register { op: ast::RTypeOp::AddS, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    1 => ast::Instruction::Register { op: ast::RTypeOp::SubS, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    2 => ast::Instruction::Register { op: ast::RTypeOp::MulS, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    3 => ast::Instruction::Register { op: ast::RTypeOp::DivS, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    4 => ast::Instruction::Register { op: ast::RTypeOp::SqrtS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    5 => ast::Instruction::Register { op: ast::RTypeOp::AbsS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    6 => ast::Instruction::Register { op: ast::RTypeOp::MovS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    7 => ast::Instruction::Register { op: ast::RTypeOp::NegS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    8 => ast::Instruction::Register { op: ast::RTypeOp::RoundLS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    9 => ast::Instruction::Register { op: ast::RTypeOp::TruncLS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    10 => ast::Instruction::Register { op: ast::RTypeOp::CeilLS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    11 => ast::Instruction::Register { op: ast::RTypeOp::FloorLS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    12 => ast::Instruction::Register { op: ast::RTypeOp::RoundWS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    13 => ast::Instruction::Register { op: ast::RTypeOp::TruncWS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    14 => ast::Instruction::Register { op: ast::RTypeOp::CeilWS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    15 => ast::Instruction::Register { op: ast::RTypeOp::FloorWS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    33 => ast::Instruction::Register { op: ast::RTypeOp::CvtDS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    36 => ast::Instruction::Register { op: ast::RTypeOp::CvtWS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    37 => ast::Instruction::Register { op: ast::RTypeOp::CvtLS, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    48..=63 => ast::Instruction::Register { op: ast::RTypeOp::Cs, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: funct & 0xF },
                    e => panic!("Invalid instruction: {} at: {:x} (ins {inst:x})", e, 4 * index),

                }
                (17, _) => match funct {
                    0 => ast::Instruction::Register { op: ast::RTypeOp::AddD, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    1 => ast::Instruction::Register { op: ast::RTypeOp::SubD, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    2 => ast::Instruction::Register { op: ast::RTypeOp::MulD, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    3 => ast::Instruction::Register { op: ast::RTypeOp::DivD, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    4 => ast::Instruction::Register { op: ast::RTypeOp::SqrtD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    5 => ast::Instruction::Register { op: ast::RTypeOp::AbsD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    6 => ast::Instruction::Register { op: ast::RTypeOp::MovD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    7 => ast::Instruction::Register { op: ast::RTypeOp::NegD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    8 => ast::Instruction::Register { op: ast::RTypeOp::RoundLD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    9 => ast::Instruction::Register { op: ast::RTypeOp::TruncLD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    10 => ast::Instruction::Register { op: ast::RTypeOp::CeilLD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    11 => ast::Instruction::Register { op: ast::RTypeOp::FloorLD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    12 => ast::Instruction::Register { op: ast::RTypeOp::RoundWD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    13 => ast::Instruction::Register { op: ast::RTypeOp::TruncWD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    14 => ast::Instruction::Register { op: ast::RTypeOp::CeilWD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    15 => ast::Instruction::Register { op: ast::RTypeOp::FloorWD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    32 => ast::Instruction::Register { op: ast::RTypeOp::CvtSD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    36 => ast::Instruction::Register { op: ast::RTypeOp::CvtWD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    37 => ast::Instruction::Register { op: ast::RTypeOp::CvtLD, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    48..=63 => ast::Instruction::Register { op: ast::RTypeOp::Cd, rs: R::try_from(rd).unwrap(), rt: R::try_from(rt).unwrap(), rd: R::null(), sa: funct & 0xF },
                    e => panic!("Invalid instruction: {} at: {:x} (ins {inst:x})", e, 4 * index),
                }
                (20, _) => match funct {
                    32 => ast::Instruction::Register { op: ast::RTypeOp::CvtSW, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    33 => ast::Instruction::Register { op: ast::RTypeOp::CvtDW, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    e => panic!("Invalid instruction: {} at: {:x} (ins {inst:x})", e, 4 * index),
                }
                (21, _) => match funct {
                    32 => ast::Instruction::Register { op: ast::RTypeOp::CvtSL, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    33 => ast::Instruction::Register { op: ast::RTypeOp::CvtDL, rs: R::try_from(rd).unwrap(), rt: R::null(), rd: R::try_from(sa).unwrap(), sa: 0 },
                    e => panic!("Invalid instruction: {} at: {:x} (ins {inst:x})", e, 4 * index),
                }
                (a, b) => panic!("Invalid instruction: {} {} at: {:x} (ins {inst:x})", a, b, 4 * index),
            }
            18 => match funct {
                0b110100 => ast::Instruction::Vector { op: ast::VTypeOp::Vrsq, vd: Vu::try_from(vd).unwrap(), vs: Vu::null(), vt: Vu::try_from(vt).unwrap(), e: e, de: de },
                0b110110 => ast::Instruction::Vector { op: ast::VTypeOp::Vrsqh, vd: Vu::try_from(vd).unwrap(), vs: Vu::null(), vt: Vu::try_from(vt).unwrap(), e: e, de: de },
                0b110101 => ast::Instruction::Vector { op: ast::VTypeOp::Vrsql, vd: Vu::try_from(vd).unwrap(), vs: Vu::null(), vt: Vu::try_from(vt).unwrap(), e: e, de: de },                
                0b110000 => ast::Instruction::Vector { op: ast::VTypeOp::Vrcp, vd: Vu::try_from(vd).unwrap(), vs: Vu::null(), vt: Vu::try_from(vt).unwrap(), e: e, de: de },      
                0b110010 => ast::Instruction::Vector { op: ast::VTypeOp::Vrcph, vd: Vu::try_from(vd).unwrap(), vs: Vu::null(), vt: Vu::try_from(vt).unwrap(), e: e, de: de },      
                0b110001 => ast::Instruction::Vector { op: ast::VTypeOp::Vrcpl, vd: Vu::try_from(vd).unwrap(), vs: Vu::null(), vt: Vu::try_from(vt).unwrap(), e: e, de: de },
                
                0b010011 => ast::Instruction::Vector { op: ast::VTypeOp::Vabs, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b010000 => ast::Instruction::Vector { op: ast::VTypeOp::Vadd, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b010100 => ast::Instruction::Vector { op: ast::VTypeOp::Vaddc, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b101000 => ast::Instruction::Vector { op: ast::VTypeOp::Vand, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b100101 => ast::Instruction::Vector { op: ast::VTypeOp::Vch, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b100100 => ast::Instruction::Vector { op: ast::VTypeOp::Vcl, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b100110 => ast::Instruction::Vector { op: ast::VTypeOp::Vcr, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b100001 => ast::Instruction::Vector { op: ast::VTypeOp::Veq, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b100011 => ast::Instruction::Vector { op: ast::VTypeOp::Vge, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b100000 => ast::Instruction::Vector { op: ast::VTypeOp::Vlt, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b001000 => ast::Instruction::Vector { op: ast::VTypeOp::Vmacf, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b001011 => ast::Instruction::Vector { op: ast::VTypeOp::Vmacq, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b001001 => ast::Instruction::Vector { op: ast::VTypeOp::Vmacu, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b001111 => ast::Instruction::Vector { op: ast::VTypeOp::Vmadh, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b001100 => ast::Instruction::Vector { op: ast::VTypeOp::Vmadl, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b001101 => ast::Instruction::Vector { op: ast::VTypeOp::Vmadm, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b001110 => ast::Instruction::Vector { op: ast::VTypeOp::Vmadn, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b110011 => ast::Instruction::Vector { op: ast::VTypeOp::Vmov, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b100111 => ast::Instruction::Vector { op: ast::VTypeOp::Vmrg, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b000111 => ast::Instruction::Vector { op: ast::VTypeOp::Vmudh, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b000100 => ast::Instruction::Vector { op: ast::VTypeOp::Vmudl, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b000101 => ast::Instruction::Vector { op: ast::VTypeOp::Vmudm, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b000110 => ast::Instruction::Vector { op: ast::VTypeOp::Vmudn, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b000000 => ast::Instruction::Vector { op: ast::VTypeOp::Vmulf, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b000011 => ast::Instruction::Vector { op: ast::VTypeOp::Vmulq, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b000001 => ast::Instruction::Vector { op: ast::VTypeOp::Vmulu, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b101001 => ast::Instruction::Vector { op: ast::VTypeOp::Vnand, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b100010 => ast::Instruction::Vector { op: ast::VTypeOp::Vne, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b101011 => ast::Instruction::Vector { op: ast::VTypeOp::Vnor, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b101101 => ast::Instruction::Vector { op: ast::VTypeOp::Vnxor, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b101010 => ast::Instruction::Vector { op: ast::VTypeOp::Vor, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b001010 => ast::Instruction::Vector { op: ast::VTypeOp::Vrndn, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b000010 => ast::Instruction::Vector { op: ast::VTypeOp::Vrndp, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b011101 => ast::Instruction::Vector { op: ast::VTypeOp::Vsar, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b010001 => ast::Instruction::Vector { op: ast::VTypeOp::Vsub, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b010101 => ast::Instruction::Vector { op: ast::VTypeOp::Vsubc, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                0b101100 => ast::Instruction::Vector { op: ast::VTypeOp::Vxor, vd: Vu::try_from(vd).unwrap(), vs: Vu::try_from(vs).unwrap(), vt: Vu::try_from(vt).unwrap(), e: e, de: 0 },
                
                0b110111 => ast::Instruction::Vector { op: ast::VTypeOp::Vnop, vd: Vu::null(), vs: Vu::null(), vt: Vu::null(), e: 0, de: 0 },
                e => panic!("Invalid VU instruction: {} at: {:x} (ins {inst:x})", e, 4 * index),
            }
            20 => ast::Instruction::Immediate { op: ast::ITypeOp::Beql, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            21 => ast::Instruction::Immediate { op: ast::ITypeOp::Bnel, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            22 => ast::Instruction::Immediate { op: ast::ITypeOp::Blezl, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            23 => ast::Instruction::Immediate { op: ast::ITypeOp::Bgtzl, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            24 => ast::Instruction::Immediate { op: ast::ITypeOp::Daddi, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            25 => ast::Instruction::Immediate { op: ast::ITypeOp::Daddiu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            26 => ast::Instruction::Immediate { op: ast::ITypeOp::Ldl, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            27 => ast::Instruction::Immediate { op: ast::ITypeOp::Ldr, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            32 => ast::Instruction::Immediate { op: ast::ITypeOp::Lb, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            33 => ast::Instruction::Immediate { op: ast::ITypeOp::Lh, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            34 => ast::Instruction::Immediate { op: ast::ITypeOp::Lwl, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            35 => ast::Instruction::Immediate { op: ast::ITypeOp::Lw, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            36 => ast::Instruction::Immediate { op: ast::ITypeOp::Lbu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            37 => ast::Instruction::Immediate { op: ast::ITypeOp::Lhu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            38 => ast::Instruction::Immediate { op: ast::ITypeOp::Lwr, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            39 => ast::Instruction::Immediate { op: ast::ITypeOp::Lwu, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            40 => ast::Instruction::Immediate { op: ast::ITypeOp::Sb, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            41 => ast::Instruction::Immediate { op: ast::ITypeOp::Sh, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            42 => ast::Instruction::Immediate { op: ast::ITypeOp::Swl, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            43 => ast::Instruction::Immediate { op: ast::ITypeOp::Sw, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            44 => ast::Instruction::Immediate { op: ast::ITypeOp::Sdl, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            45 => ast::Instruction::Immediate { op: ast::ITypeOp::Sdr, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            46 => ast::Instruction::Immediate { op: ast::ITypeOp::Swr, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            47 => ast::Instruction::Immediate { op: ast::ITypeOp::Cache, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            48 => ast::Instruction::Immediate { op: ast::ITypeOp::Ll, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            49 => ast::Instruction::Immediate { op: ast::ITypeOp::Lwc1, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            50 => ast::Instruction::Immediate { op: ast::ITypeOp::Lbv, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            52 => ast::Instruction::Immediate { op: ast::ITypeOp::Lld, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            53 => ast::Instruction::Immediate { op: ast::ITypeOp::Ldc1, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            55 => ast::Instruction::Immediate { op: ast::ITypeOp::Ld, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            56 => ast::Instruction::Immediate { op: ast::ITypeOp::Sc, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            57 => ast::Instruction::Immediate { op: ast::ITypeOp::Swc1, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            60 => ast::Instruction::Immediate { op: ast::ITypeOp::Scd, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            61 => ast::Instruction::Immediate { op: ast::ITypeOp::Sdc1, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            63 => ast::Instruction::Immediate { op: ast::ITypeOp::Sd, rs: R::try_from(rs).unwrap(), rt: R::try_from(rt).unwrap(), imm: ast::Immediate(imm as u16) },
            e => panic!("Invalid instruction: {} at: {:x} (ins {inst:x})", e, 4 * index),
        };

        insts.push(i);
    }
    insts
}
