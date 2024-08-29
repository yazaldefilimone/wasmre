use serde::{Deserialize, Serialize};

use crate::utils::range::Range;

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
  pub body: Vec<Module>,
}

impl Default for Program {
  fn default() -> Self {
    Self { body: vec![] }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
  pub types: Type,
  pub imports: Import,
  pub functions: Function,
  pub tables: Table,
  pub memories: Memory,
  pub globals: Global,
  pub exports: Export,
  pub start: Option<Start>,
  pub elements: Element,
  pub codes: Code,
  pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
  pub form: String,
  pub params: Vec<ValueType>,
  pub results: Vec<ValueType>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Import {
  pub module: String,
  pub name: String,
  pub desc: ImportDesc,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImportDesc {
  Func(u32),
  Table(TableType),
  Mem(MemoryType),
  Global(GlobalType),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
  pub name: Identifier,
  pub signature: Signature,
  pub body: BlockInstr,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
  pub params: Vec<ValueType>,
  pub results: Vec<ValueType>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
  pub name: String,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Local {
  pub count: u32,
  pub value_type: ValueType,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
  pub table_type: TableType,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
  pub memory_type: MemoryType,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Global {
  pub global_type: GlobalType,
  pub init: Vec<Instr>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Export {
  pub name: String,
  pub desc: ExportDesc,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExportDesc {
  Func(u32),
  Table(u32),
  Mem(u32),
  Global(u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Start {
  pub func_idx: u32,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
  pub table: u32,
  pub offset: Vec<Instr>,
  pub init: Vec<u32>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
  pub locals: Vec<Local>,
  pub body: Vec<Instr>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
  pub data: u32,
  pub offset: Vec<Instr>,
  pub init: Vec<u8>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValueType {
  I32,
  I64,
  F32,
  F64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableType {
  pub element_type: ValueType,
  pub limits: Limits,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryType {
  pub limits: Limits,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalType {
  pub value_type: ValueType,
  pub mutable: bool,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limits {
  pub min: u32,
  pub max: Option<u32>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Instr {
  Unreachable { range: Range },
  Nop { range: Range },
  Block(BlockInstr),
  Loop(LoopInstr),
  If(IfInstr),
  Branch(BranchInstr),
  BranchIf(BranchIfInstr),
  BranchTable(BranchTableInstr),
  Return { range: Range },
  // Function calls
  Call(CallInstr),
  CallIndirect(CallIndirectInstr),
  // Variable instr
  LocalGet(VariableInstr),
  LocalSet(VariableInstr),
  LocalTee(VariableInstr),
  GlobalGet(VariableInstr),
  GlobalSet(VariableInstr),
  // Memory instr
  I32Load(MemInstr),
  I32Store(MemInstr),
  I64Load(MemInstr),
  I64Store(MemInstr),
  F32Load(MemInstr),
  F32Store(MemInstr),
  F64Load(MemInstr),
  F64Store(MemInstr),
  // Constants
  I32Const { value: i32, range: Range },
  I64Const { value: i64, range: Range },
  F32Const { value: f32, range: Range },
  F64Const { value: f64, range: Range },
  // Numeric operations
  I32Add { range: Range },
  I32Sub { range: Range },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockInstr {
  pub instr: Vec<Instr>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoopInstr {
  pub loop_type: Option<ValueType>,
  pub instr: Vec<Instr>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfInstr {
  pub condition: Box<Instr>,
  pub instr: Vec<Instr>,
  pub else_instr: Option<Vec<Instr>>,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchInstr {
  pub label_idx: u32,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchIfInstr {
  pub label_idx: u32,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchTableInstr {
  pub labels: Vec<u32>,
  pub default: u32,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallInstr {
  pub function_idx: u32,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallIndirectInstr {
  pub type_idx: u32,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemInstr {
  pub offset: u32,
  pub align: u32,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableInstr {
  pub index: u32,
  pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NumericInstr {
  I32Add { range: Range },
  I32Sub { range: Range },
}
