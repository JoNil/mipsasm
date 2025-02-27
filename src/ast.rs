use alloc::{
    format,
    string::{String, ToString},
};
use core::convert::{From, TryFrom};
use core::fmt;
use core::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug)]
pub enum RegParseError {
    RegParseError(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Target(pub u32);

#[derive(Debug, PartialEq, Eq)]
pub struct Immediate(pub u16);

struct Signed(u16);
// Format an i16 as a sign-aware hex string
// https://stackoverflow.com/a/44712309
impl fmt::LowerHex for Signed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.0 as i16;
        let p = if f.alternate() { "0x" } else { "" };
        let x = format!("{:x}", val.abs());
        f.pad_integral(val >= 0, p, &x)
    }
}

#[derive(PartialEq, Eq)]
pub enum Instruction {
    Immediate {
        op: ITypeOp,
        rs: Register,
        rt: Register,
        imm: Immediate,
    },
    Jump {
        op: JTypeOp,
        target: Target,
    },
    Register {
        op: RTypeOp,
        rs: Register,
        rt: Register,
        rd: Register,
        sa: u32,
    },
    Vector {
        op: VTypeOp,
        vd: VuRegister,
        vs: VuRegister,
        vt: VuRegister,
        de: u32,
        e: u32,
    },
}

type I = ITypeOp;
type R = RTypeOp;
type V = VTypeOp;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Instruction::Immediate {
                op,
                rs,
                rt,
                imm: Immediate(imm),
            } => match op {
                I::Lb
                | I::Lbu
                | I::Ld
                | I::Ldl
                | I::Ldr
                | I::Lh
                | I::Lhu
                | I::Ll
                | I::Lld
                | I::Lw
                | I::Lwl
                | I::Lwr
                | I::Lwu
                | I::Sb
                | I::Sc
                | I::Scd
                | I::Sd
                | I::Sdl
                | I::Sdr
                | I::Sh
                | I::Sw
                | I::Swl
                | I::Swr => {
                    write!(f, "{:7} {}, {:#x}({})", op, rt, Signed(*imm), rs)
                }
                I::Lbv => {
                    write!(
                        f,
                        "{:7} {}[{}], {:#x}({})",
                        op,
                        rt,
                        *imm >> 7,
                        *imm & 0b1111111,
                        rs
                    )
                }
                I::Cache => {
                    write!(
                        f,
                        "{:7} {:#x}, {:#x}({})",
                        op,
                        rt.as_num(),
                        Signed(*imm),
                        rs
                    )
                }
                I::Addi | I::Addiu | I::Daddi | I::Daddiu | I::Slti | I::Sltiu => {
                    write!(f, "{:7} {}, {}, {:#x}", op, rt, rs, Signed(*imm))
                }
                I::Andi | I::Ori | I::Xori => write!(f, "{:7} {}, {}, {:#x}", op, rt, rs, imm),
                I::Lui => write!(f, "{:7} {}, {:#x}", op, rt, imm),
                I::Beqz | I::Bgtz | I::Bgtzl | I::Blez | I::Blezl | I::Bnez => {
                    write!(f, "{:7} {}, {:#x}", op, rs, Signed(*imm))
                }
                I::Beq | I::Beql | I::Bne | I::Bnel => {
                    write!(f, "{:7} {}, {}, {:#x}", op, rs, rt, Signed(*imm))
                }
                I::Bgez
                | I::Bgezal
                | I::Bgezall
                | I::Bgezl
                | I::Bltz
                | I::Bltzal
                | I::Bltzall
                | I::Bltzl
                | I::Teqi
                | I::Tgei
                | I::Tgeiu
                | I::Tlti
                | I::Tltiu
                | I::Tnei => {
                    write!(f, "{:7} {}, {:#x}", op, rs, Signed(*imm))
                }
                I::Bc0f
                | I::Bc1f
                | I::Bc0fl
                | I::Bc1fl
                | I::Bc0t
                | I::Bc1t
                | I::Bc0tl
                | I::Bc1tl => {
                    write!(f, "{:7}{:#x}", op, Signed(*imm))
                }
                I::Ldc1 | I::Lwc1 | I::Sdc1 | I::Swc1 => {
                    write!(
                        f,
                        "{:7} {}, {:#x}({})",
                        op,
                        FloatRegister::from(*rt),
                        Signed(*imm),
                        rs
                    )
                }
                e => panic!("Unhandled immediate instruction: {:?}", e),
            },
            Instruction::Jump {
                op,
                target: Target(target),
            } => {
                write!(f, "{:7}{:#X?}", op, target)
            }
            Instruction::Register { op, rs, rt, rd, sa } => match op {
                R::Sync => write!(f, "{}", op),
                R::Add
                | R::Addu
                | R::And
                | R::Dadd
                | R::Daddu
                | R::Dsub
                | R::Dsubu
                | R::Nor
                | R::Or
                | R::Slt
                | R::Sltu
                | R::Sub
                | R::Subu
                | R::Xor => {
                    write!(f, "{:7} {}, {}, {}", op, rd, rs, rt)
                }
                R::Dsll
                | R::Dsll32
                | R::Dsra
                | R::Dsra32
                | R::Dsrl
                | R::Dsrl32
                | R::Sll
                | R::Sra
                | R::Srl => {
                    write!(f, "{:7} {}, {}, {:#x?}", op, rd, rt, sa)
                }
                R::Dsllv | R::Dsrav | R::Dsrlv | R::Sllv | R::Srav | R::Srlv => {
                    write!(f, "{:7} {}, {}, {}", op, rd, rt, rs)
                }
                R::Break | R::Syscall => {
                    if *sa == 0 {
                        write!(f, "{}", op)
                    } else {
                        write!(f, "{:7} {:#x?}", op, sa)
                    }
                }
                R::Ddiv
                | R::Ddivu
                | R::Div
                | R::Divu
                | R::Dmult
                | R::Dmultu
                | R::Mult
                | R::Multu
                | R::Teq
                | R::Tge
                | R::Tgeu
                | R::Tlt
                | R::Tltu
                | R::Tne => {
                    write!(f, "{:7} {}, {}", op, rs, rt)
                }
                R::Jalr => {
                    if let &Register::Ra = rd {
                        write!(f, "{:7} {}", op, rs)
                    } else {
                        write!(f, "{:7} {},  {}", op, rd, rs)
                    }
                }
                R::Jr | R::Mthi | R::Mtlo => {
                    write!(f, "{:7} {}", op, rs)
                }
                R::Mfhi | R::Mflo => {
                    write!(f, "{:7} {}", op, rd)
                }
                R::Cfc0 | R::Ctc0 | R::Dmfc0 | R::Dmtc0 | R::Mfc0 | R::Mtc0 => {
                    write!(f, "{:7} {}, {}", op, rt, Cop0Register::from(*rd))
                }
                R::Cfc1 | R::Ctc1 | R::Dmfc1 | R::Dmtc1 | R::Mfc1 | R::Mtc1 => {
                    write!(f, "{:7} {}, {}", op, rt, FloatRegister::from(*rd))
                }
                R::Eret | R::Tlbp | R::Tlbr | R::Tlbwi | R::Tlbwr => {
                    write!(f, "{}", op)
                }
                R::AddS | R::AddD | R::SubS | R::SubD | R::MulS | R::MulD | R::DivS | R::DivD => {
                    let x = op.to_string().replace('_', ".");
                    write!(
                        f,
                        "{:7} {},  {},  {}",
                        x,
                        FloatRegister::from(*rd),
                        FloatRegister::from(*rs),
                        FloatRegister::from(*rt)
                    )
                }
                R::AbsS
                | R::AbsD
                | R::CvtDS
                | R::CvtDW
                | R::CvtDL
                | R::CvtLS
                | R::CvtLD
                | R::CvtSD
                | R::CvtSW
                | R::CvtSL
                | R::CvtWD
                | R::CvtWS
                | R::MovS
                | R::MovD
                | R::NegS
                | R::NegD
                | R::SqrtS
                | R::SqrtD => {
                    let x = op.to_string().replace('_', ".");
                    write!(
                        f,
                        "{:7} {},  {}",
                        x,
                        FloatRegister::from(*rd),
                        FloatRegister::from(*rs)
                    )
                }
                R::CeilLS | R::CeilLD | R::CeilWS | R::CeilWD => {
                    let x = op.to_string().replace('_', ".");
                    write!(
                        f,
                        "{} {}, {}",
                        x,
                        FloatRegister::from(*rd),
                        FloatRegister::from(*rs)
                    )
                }
                R::FloorLS
                | R::FloorLD
                | R::FloorWS
                | R::FloorWD
                | R::RoundLS
                | R::RoundLD
                | R::RoundWS
                | R::RoundWD
                | R::TruncLS
                | R::TruncLD
                | R::TruncWS
                | R::TruncWD => {
                    let x = op.to_string().replace('_', ".");
                    write!(
                        f,
                        "{} {}, {}",
                        x,
                        FloatRegister::from(*rd),
                        FloatRegister::from(*rs)
                    )
                }
                R::Cs => {
                    write!(
                        f,
                        "c.{}.s {}, {}",
                        FloatCond::try_from(*sa).unwrap(),
                        FloatRegister::from(*rs),
                        FloatRegister::from(*rt)
                    )
                }
                R::Cd => {
                    write!(
                        f,
                        "c.{}.d {}, {}",
                        FloatCond::try_from(*sa).unwrap(),
                        FloatRegister::from(*rs),
                        FloatRegister::from(*rt)
                    )
                }
                e => panic!("{:?} not implemented", e),
            },
            Instruction::Vector {
                op,
                vd,
                vs,
                vt,
                e,
                de,
            } => match op {
                V::Vrsq | V::Vrsqh | V::Vrsql | V::Vrcp | V::Vrcph | V::Vrcpl => {
                    write!(f, "{:7} {}[{}] {}[{}]", op, vd, de, vt, e)
                }
                V::Vabs
                | V::Vadd
                | V::Vaddc
                | V::Vand
                | V::Vch
                | V::Vcl
                | V::Vcr
                | V::Veq
                | V::Vge
                | V::Vlt
                | V::Vmacf
                | V::Vmacq
                | V::Vmacu
                | V::Vmadh
                | V::Vmadl
                | V::Vmadm
                | V::Vmadn
                | V::Vmov
                | V::Vmrg
                | V::Vmudh
                | V::Vmudl
                | V::Vmudm
                | V::Vmudn
                | V::Vmulf
                | V::Vmulq
                | V::Vmulu
                | V::Vnand
                | V::Vne
                | V::Vnor
                | V::Vnxor
                | V::Vor
                | V::Vrndn
                | V::Vrndp
                | V::Vsar
                | V::Vsub
                | V::Vsubc
                | V::Vxor => {
                    write!(f, "{:7} {} {} {}[{}]", op, vd, vs, vt, e)
                }
                V::Vnop => {
                    write!(f, "{:7}", "Vnop")
                }
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Register {
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    A2,
    A3,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    T8,
    T9,
    K0,
    K1,
    Gp,
    Sp,
    Fp,
    Ra,
}

impl Register {
    pub fn null() -> Self {
        Register::Zero
    }

    pub fn as_num(&self) -> u32 {
        *self as u32
    }
}

impl TryFrom<u32> for Register {
    type Error = RegParseError;

    fn try_from(reg: u32) -> Result<Self, Self::Error> {
        match reg {
            0 => Ok(Register::Zero),
            1 => Ok(Register::At),
            2 => Ok(Register::V0),
            3 => Ok(Register::V1),
            4 => Ok(Register::A0),
            5 => Ok(Register::A1),
            6 => Ok(Register::A2),
            7 => Ok(Register::A3),
            8 => Ok(Register::T0),
            9 => Ok(Register::T1),
            10 => Ok(Register::T2),
            11 => Ok(Register::T3),
            12 => Ok(Register::T4),
            13 => Ok(Register::T5),
            14 => Ok(Register::T6),
            15 => Ok(Register::T7),
            16 => Ok(Register::S0),
            17 => Ok(Register::S1),
            18 => Ok(Register::S2),
            19 => Ok(Register::S3),
            20 => Ok(Register::S4),
            21 => Ok(Register::S5),
            22 => Ok(Register::S6),
            23 => Ok(Register::S7),
            24 => Ok(Register::T8),
            25 => Ok(Register::T9),
            26 => Ok(Register::K0),
            27 => Ok(Register::K1),
            28 => Ok(Register::Gp),
            29 => Ok(Register::Sp),
            30 => Ok(Register::Fp),
            31 => Ok(Register::Ra),
            e => Err(RegParseError::RegParseError(e.to_string())),
        }
    }
}

impl From<FloatRegister> for Register {
    fn from(reg: FloatRegister) -> Self {
        Register::try_from(reg as u32).unwrap()
    }
}

impl From<Cop0Register> for Register {
    fn from(reg: Cop0Register) -> Self {
        Register::try_from(reg as u32).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum VuRegister {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,
}

impl VuRegister {
    pub fn null() -> Self {
        VuRegister::V0
    }
}

impl TryFrom<u32> for VuRegister {
    type Error = RegParseError;

    fn try_from(reg: u32) -> Result<Self, Self::Error> {
        match reg {
            0 => Ok(VuRegister::V0),
            1 => Ok(VuRegister::V1),
            2 => Ok(VuRegister::V2),
            3 => Ok(VuRegister::V3),
            4 => Ok(VuRegister::V4),
            5 => Ok(VuRegister::V5),
            6 => Ok(VuRegister::V6),
            7 => Ok(VuRegister::V7),
            8 => Ok(VuRegister::V8),
            9 => Ok(VuRegister::V9),
            10 => Ok(VuRegister::V10),
            11 => Ok(VuRegister::V11),
            12 => Ok(VuRegister::V12),
            13 => Ok(VuRegister::V13),
            14 => Ok(VuRegister::V14),
            15 => Ok(VuRegister::V15),
            16 => Ok(VuRegister::V16),
            17 => Ok(VuRegister::V17),
            18 => Ok(VuRegister::V18),
            19 => Ok(VuRegister::V19),
            20 => Ok(VuRegister::V20),
            21 => Ok(VuRegister::V21),
            22 => Ok(VuRegister::V22),
            23 => Ok(VuRegister::V23),
            24 => Ok(VuRegister::V24),
            25 => Ok(VuRegister::V25),
            26 => Ok(VuRegister::V26),
            27 => Ok(VuRegister::V27),
            28 => Ok(VuRegister::V28),
            29 => Ok(VuRegister::V29),
            30 => Ok(VuRegister::V30),
            31 => Ok(VuRegister::V31),
            e => Err(RegParseError::RegParseError(e.to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum FloatRegister {
    Fv0,
    Fv0f,
    Fv1,
    Fv1f,
    Ft0,
    Ft0f,
    Ft1,
    Ft1f,
    Ft2,
    Ft2f,
    Ft3,
    Ft3f,
    Fa0,
    Fa0f,
    Fa1,
    Fa1f,
    Ft4,
    Ft4f,
    Ft5,
    Ft5f,
    Fs0,
    Fs0f,
    Fs1,
    Fs1f,
    Fs2,
    Fs2f,
    Fs3,
    Fs3f,
    Fs4,
    Fs4f,
    Fs5,
    Fs5f,
}

impl TryFrom<u32> for FloatRegister {
    type Error = RegParseError;

    fn try_from(reg: u32) -> Result<Self, Self::Error> {
        match reg {
            0 => Ok(FloatRegister::Fv0),
            1 => Ok(FloatRegister::Fv0f),
            2 => Ok(FloatRegister::Fv1),
            3 => Ok(FloatRegister::Fv1f),
            4 => Ok(FloatRegister::Ft0),
            5 => Ok(FloatRegister::Ft0f),
            6 => Ok(FloatRegister::Ft1),
            7 => Ok(FloatRegister::Ft1f),
            8 => Ok(FloatRegister::Ft2),
            9 => Ok(FloatRegister::Ft2f),
            10 => Ok(FloatRegister::Ft3),
            11 => Ok(FloatRegister::Ft3f),
            12 => Ok(FloatRegister::Fa0),
            13 => Ok(FloatRegister::Fa0f),
            14 => Ok(FloatRegister::Fa1),
            15 => Ok(FloatRegister::Fa1f),
            16 => Ok(FloatRegister::Ft4),
            17 => Ok(FloatRegister::Ft4f),
            18 => Ok(FloatRegister::Ft5),
            19 => Ok(FloatRegister::Ft5f),
            20 => Ok(FloatRegister::Fs0),
            21 => Ok(FloatRegister::Fs0f),
            22 => Ok(FloatRegister::Fs1),
            23 => Ok(FloatRegister::Fs1f),
            24 => Ok(FloatRegister::Fs2),
            25 => Ok(FloatRegister::Fs2f),
            26 => Ok(FloatRegister::Fs3),
            27 => Ok(FloatRegister::Fs3f),
            28 => Ok(FloatRegister::Fs4),
            29 => Ok(FloatRegister::Fs4f),
            30 => Ok(FloatRegister::Fs5),
            31 => Ok(FloatRegister::Fs5f),
            e => Err(RegParseError::RegParseError(e.to_string())),
        }
    }
}

impl FromStr for FloatRegister {
    type Err = RegParseError;

    fn from_str(reg: &str) -> Result<Self, Self::Err> {
        let reg = reg.trim().trim_start_matches('$');

        if let Ok(x) = reg.parse::<u32>() {
            return FloatRegister::try_from(x);
        }

        match reg.to_lowercase().as_str() {
            "f0" | "fv0" => Ok(FloatRegister::Fv0),
            "f1" | "fv0f" => Ok(FloatRegister::Fv0f),
            "f2" | "fv1" => Ok(FloatRegister::Fv1),
            "f3" | "fv1f" => Ok(FloatRegister::Fv1f),
            "f4" | "ft0" => Ok(FloatRegister::Ft0),
            "f5" | "ft0f" => Ok(FloatRegister::Ft0f),
            "f6" | "ft1" => Ok(FloatRegister::Ft1),
            "f7" | "ft1f" => Ok(FloatRegister::Ft1f),
            "f8" | "ft2" => Ok(FloatRegister::Ft2),
            "f9" | "ft2f" => Ok(FloatRegister::Ft2f),
            "f10" | "ft3" => Ok(FloatRegister::Ft3),
            "f11" | "ft3f" => Ok(FloatRegister::Ft3f),
            "f12" | "fa0" => Ok(FloatRegister::Fa0),
            "f13" | "fa0f" => Ok(FloatRegister::Fa0f),
            "f14" | "fa1" => Ok(FloatRegister::Fa1),
            "f15" | "fa1f" => Ok(FloatRegister::Fa1f),
            "f16" | "ft4" => Ok(FloatRegister::Ft4),
            "f17" | "ft4f" => Ok(FloatRegister::Ft4f),
            "f18" | "ft5" => Ok(FloatRegister::Ft5),
            "f19" | "ft5f" => Ok(FloatRegister::Ft5f),
            "f20" | "fs0" => Ok(FloatRegister::Fs0),
            "f21" | "fs0f" => Ok(FloatRegister::Fs0f),
            "f22" | "fs1" => Ok(FloatRegister::Fs1),
            "f23" | "fs1f" => Ok(FloatRegister::Fs1f),
            "f24" | "fs2" => Ok(FloatRegister::Fs2),
            "f25" | "fs2f" => Ok(FloatRegister::Fs2f),
            "f26" | "fs3" => Ok(FloatRegister::Fs3),
            "f27" | "fs3f" => Ok(FloatRegister::Fs3f),
            "f28" | "fs4" => Ok(FloatRegister::Fs4),
            "f29" | "fs4f" => Ok(FloatRegister::Fs4f),
            "f30" | "fs5" => Ok(FloatRegister::Fs5),
            "f31" | "fs5f" => Ok(FloatRegister::Fs5f),
            e => {
                if let Ok(x) = u32::from_str_radix(reg, 16) {
                    return FloatRegister::try_from(x);
                }
                Err(RegParseError::RegParseError(e.to_string()))
            }
        }
    }
}

impl From<Register> for FloatRegister {
    fn from(reg: Register) -> Self {
        FloatRegister::try_from(reg as u32).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Display)]
#[strum(serialize_all = "PascalCase")]
pub enum Cop0Register {
    IDMemAddressForDMA,
    DramAddressForDMA,
    DmaReadLength,
    DmaWriteLength,
    RspStatus,
    DmaFull,
    DmaBusy,
    CpuRspSemaphore,
    RdpCommandBufferStart,
    RdpCommandBufferEnd,
    RdpCommandBufferCurrent,
    RdpStatus,
    RdpClockCounter,
    RdpCommandBufferBusyCounter,
    RdpPipeBusyCounter,
    RdpTmemBusyCounter,
}

impl TryFrom<u32> for Cop0Register {
    type Error = RegParseError;

    fn try_from(reg: u32) -> Result<Self, Self::Error> {
        match reg {
            0 => Ok(Cop0Register::IDMemAddressForDMA),
            1 => Ok(Cop0Register::DramAddressForDMA),
            2 => Ok(Cop0Register::DmaReadLength),
            3 => Ok(Cop0Register::DmaWriteLength),
            4 => Ok(Cop0Register::RspStatus),
            5 => Ok(Cop0Register::DmaFull),
            6 => Ok(Cop0Register::DmaBusy),
            7 => Ok(Cop0Register::CpuRspSemaphore),
            8 => Ok(Cop0Register::RdpCommandBufferStart),
            9 => Ok(Cop0Register::RdpCommandBufferEnd),
            10 => Ok(Cop0Register::RdpCommandBufferCurrent),
            11 => Ok(Cop0Register::RdpStatus),
            12 => Ok(Cop0Register::RdpClockCounter),
            13 => Ok(Cop0Register::RdpCommandBufferBusyCounter),
            14 => Ok(Cop0Register::RdpPipeBusyCounter),
            15 => Ok(Cop0Register::RdpTmemBusyCounter),
            e => Err(RegParseError::RegParseError(e.to_string())),
        }
    }
}

impl From<Register> for Cop0Register {
    fn from(reg: Register) -> Self {
        Cop0Register::try_from(reg as u32).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Display, EnumString, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "snake_case")]
pub enum ITypeOp {
    Addi,
    Addiu,
    Andi,
    Bc0f,
    Bc0fl,
    Bc0t,
    Bc0tl,
    Bc1f,
    Bc1fl,
    Bc1t,
    Bc1tl,
    Beq,
    Beql,
    Bgez,
    Bgezal,
    Bgezall,
    Bgezl,
    Bgtz,
    Bgtzl,
    Blez,
    Blezl,
    Bltz,
    Bltzal,
    Bltzall,
    Bltzl,
    Bne,
    Bnel,
    Cache,
    Daddi,
    Daddiu,
    Lb,
    Lbu,
    Ld,
    Ldc1,
    Ldl,
    Ldr,
    Lh,
    Lhu,
    Ll,
    Lld,
    Lui,
    Lw,
    Lwc1,
    Lbv,
    Lwl,
    Lwr,
    Lwu,
    Ori,
    Sb,
    Sc,
    Scd,
    Sd,
    Sdc1,
    Sdl,
    Sdr,
    Sh,
    Slti,
    Sltiu,
    Sw,
    Swc1,
    Swl,
    Swr,
    Teqi,
    Tgei,
    Tgeiu,
    Tlti,
    Tltiu,
    Tnei,
    Xori,
    // Pseudoinstructions
    B,
    Bal,
    Beqz,
    Bnez,
    Beqzl,
    Bnezl,
    Bge,
    Bgt,
    Ble,
    Blt,
    Bgeu,
    Bgtu,
    Bleu,
    Bltu,
    Bgel,
    Bgtl,
    Blel,
    Bltl,
    Bgeul,
    Bgtul,
    Bleul,
    Bltul,
    Dli,
    Dsubi,
    Dsubiu,
    Lli,
    Li,
    Subi,
    Subiu,
}

#[derive(Clone, Copy, Debug, Display, EnumString, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "snake_case")]
pub enum VTypeOp {
    // VU istructions
    Vabs,
    Vadd,
    Vaddc,
    Vand,
    Vch,
    Vcl,
    Vcr,
    Veq,
    Vge,
    Vlt,
    Vmacf,
    Vmacq,
    Vmacu,
    Vmadh,
    Vmadl,
    Vmadm,
    Vmadn,
    Vmov,
    Vmrg,
    Vmudh,
    Vmudl,
    Vmudm,
    Vmudn,
    Vmulf,
    Vmulq,
    Vmulu,
    Vnand,
    Vne,
    Vnop,
    Vnor,
    Vnxor,
    Vor,
    Vrcp,
    Vrcph,
    Vrcpl,
    Vrndn,
    Vrndp,
    Vrsq,
    Vrsqh,
    Vrsql,
    Vsar,
    Vsub,
    Vsubc,
    Vxor,
}

#[derive(Clone, Copy, Debug, Display, EnumString, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "snake_case")]
pub enum JTypeOp {
    J,
    Jal,
}

#[derive(Clone, Copy, Debug, Display, EnumString, PartialEq, Eq)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "snake_case")]
pub enum RTypeOp {
    #[strum(to_string = "abs.s")]
    AbsS,
    #[strum(to_string = "abs.d")]
    AbsD,
    Add,
    Addu,
    #[strum(to_string = "add.s")]
    AddS,
    #[strum(to_string = "add.d")]
    AddD,
    And,
    Break,
    #[strum(to_string = "c.s")]
    Cs,
    #[strum(to_string = "c.d")]
    Cd,
    #[strum(to_string = "ceil.l.s")]
    CeilLS,
    #[strum(to_string = "ceil.l.d")]
    CeilLD,
    #[strum(to_string = "ceil.w.s")]
    CeilWS,
    #[strum(to_string = "ceil.w.d")]
    CeilWD,
    Cfc0,
    Cfc1,
    Ctc0,
    Ctc1,
    #[strum(to_string = "cvt.d.s")]
    CvtDS,
    #[strum(to_string = "cvt.d.w")]
    CvtDW,
    #[strum(to_string = "cvt.d.l")]
    CvtDL,
    #[strum(to_string = "cvt.l.s")]
    CvtLS,
    #[strum(to_string = "cvt.l.d")]
    CvtLD,
    #[strum(to_string = "cvt.s.d")]
    CvtSD,
    #[strum(to_string = "cvt.s.w")]
    CvtSW,
    #[strum(to_string = "cvt.s.l")]
    CvtSL,
    #[strum(to_string = "cvt.w.s")]
    CvtWS,
    #[strum(to_string = "cvt.w.d")]
    CvtWD,
    Dadd,
    Daddu,
    Ddiv,
    Ddivu,
    Div,
    Divu,
    #[strum(to_string = "div.s")]
    DivS,
    #[strum(to_string = "div.d")]
    DivD,
    Dmfc0,
    Dmfc1,
    Dmtc0,
    Dmtc1,
    Dmult,
    Dmultu,
    Dsll,
    Dsll32,
    Dsllv,
    Dsra,
    Dsra32,
    Dsrav,
    Dsrl,
    Dsrl32,
    Dsrlv,
    Dsub,
    Dsubu,
    Eret,
    #[strum(to_string = "floor.l.s")]
    FloorLS,
    #[strum(to_string = "floor.l.d")]
    FloorLD,
    #[strum(to_string = "floor.w.s")]
    FloorWS,
    #[strum(to_string = "floor.w.d")]
    FloorWD,
    Jalr,
    Jr,
    Mfc0,
    Mfc1,
    Mfhi,
    Mflo,
    #[strum(to_string = "mov.s")]
    MovS,
    #[strum(to_string = "mov.d")]
    MovD,
    Mtc0,
    Mtc1,
    Mthi,
    Mtlo,
    #[strum(to_string = "mul.s")]
    MulS,
    #[strum(to_string = "mul.d")]
    MulD,
    Mult,
    Multu,
    #[strum(to_string = "neg.s")]
    NegS,
    #[strum(to_string = "neg.d")]
    NegD,
    Nor,
    Or,
    #[strum(to_string = "round.l.s")]
    RoundLS,
    #[strum(to_string = "round.l.d")]
    RoundLD,
    #[strum(to_string = "round.w.s")]
    RoundWS,
    #[strum(to_string = "round.w.d")]
    RoundWD,
    Sll,
    Sllv,
    Slt,
    Sltu,
    #[strum(to_string = "sqrt.s")]
    SqrtS,
    #[strum(to_string = "sqrt.d")]
    SqrtD,
    Sra,
    Srav,
    Srl,
    Srlv,
    Sub,
    Subu,
    #[strum(to_string = "sub.s")]
    SubS,
    #[strum(to_string = "sub.d")]
    SubD,
    Sync,
    Syscall,
    Teq,
    Tge,
    Tgeu,
    Tlbp,
    Tlbr,
    Tlbwi,
    Tlbwr,
    Tlt,
    Tltu,
    Tne,
    #[strum(to_string = "trunc.l.s")]
    TruncLS,
    #[strum(to_string = "trunc.l.d")]
    TruncLD,
    #[strum(to_string = "trunc.w.s")]
    TruncWS,
    #[strum(to_string = "trunc.w.d")]
    TruncWD,
    Xor,
    // pseudoinstructions
    Abs,
    Clear,
    Dabs,
    Dmove,
    Dmul,
    Dmulu,
    Dmulo,
    Dmulou,
    Dneg,
    Dnegu,
    Drem,
    Dremu,
    Drol,
    Dror,
    Move,
    Mul,
    Mulu,
    Mulo,
    Mulou,
    Neg,
    Negu,
    Nop,
    Not,
    Rem,
    Remu,
    Seq,
    Sge,
    Sgeu,
    Sgt,
    Sgtu,
    Sle,
    Sleu,
    Sne,
}

#[derive(Clone, Copy, Debug, Display, EnumString)]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "snake_case")]
pub enum FloatCond {
    F,
    Un,
    Eq,
    Ueq,
    Olt,
    Ult,
    Ole,
    Ule,
    Sf,
    Ngle,
    Seq,
    Ngl,
    Lt,
    Nge,
    Le,
    Ngt,
}

impl TryFrom<u32> for FloatCond {
    type Error = RegParseError;

    fn try_from(cond: u32) -> Result<Self, Self::Error> {
        match cond {
            0 => Ok(FloatCond::F),
            1 => Ok(FloatCond::Un),
            2 => Ok(FloatCond::Eq),
            3 => Ok(FloatCond::Ueq),
            4 => Ok(FloatCond::Olt),
            5 => Ok(FloatCond::Ult),
            6 => Ok(FloatCond::Ole),
            7 => Ok(FloatCond::Ule),
            8 => Ok(FloatCond::Sf),
            9 => Ok(FloatCond::Ngle),
            10 => Ok(FloatCond::Seq),
            11 => Ok(FloatCond::Ngl),
            12 => Ok(FloatCond::Lt),
            13 => Ok(FloatCond::Nge),
            14 => Ok(FloatCond::Le),
            15 => Ok(FloatCond::Ngt),
            e => Err(RegParseError::RegParseError(e.to_string())),
        }
    }
}
