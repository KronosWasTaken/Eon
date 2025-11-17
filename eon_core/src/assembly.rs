use crate::translate_bytecode::{ConstantsHolder, LabelMap, Translator, TranslatorState};
use crate::vm::{CallData, Instr as VmInstr, ProgramCounter};
use std::fmt::{self, Display, Formatter};
use utils::hash::HashMap;
pub(crate) type Label = String;
#[derive(Debug, Clone)]
pub(crate) enum Line {
    Instr {
        instr: Instr,
        lineno: usize,
        file_id: u32,
        func_id: u32,
    },
    Label(Label),
}
pub(crate) trait LineVariant {
    fn to_line(self, translator: &Translator, translator_state: &TranslatorState) -> Line;
}
impl LineVariant for Line {
    fn to_line(self, _translator: &Translator, _st: &TranslatorState) -> Line {
        self
    }
}
impl LineVariant for Label {
    fn to_line(self, _translator: &Translator, _st: &TranslatorState) -> Line {
        Line::Label(self)
    }
}
impl LineVariant for Instr {
    fn to_line(self, _translator: &Translator, st: &TranslatorState) -> Line {
        Line::Instr {
            instr: self,
            lineno: st.curr_lineno,
            file_id: st.curr_file,
            func_id: st.curr_func,
        }
    }
}
impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Line::Instr { instr, .. } => write!(f, "\t{instr}"),
            Line::Label(label) => write!(f, "{label}:"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Instr {
    Pop,
    Duplicate,
    LoadOffset(i32),
    StoreOffset(i32),
    PushNil(u16),
    PushBool(bool),
    PushInt(i64),
    PushFloat(String),
    PushString(String),
    AddInt,
    SubtractInt,
    MultiplyInt,
    DivideInt,
    PowerInt,
    Modulo,
    AddFloat,
    SubtractFloat,
    MultiplyFloat,
    DivideFloat,
    PowerFloat,
    SquareRoot,
    Not,
    And,
    Or,
    LessThanInt,
    LessThanOrEqualInt,
    GreaterThanInt,
    GreaterThanOrEqualInt,
    LessThanFloat,
    LessThanOrEqualFloat,
    GreaterThanFloat,
    GreaterThanOrEqualFloat,
    EqualInt,
    EqualFloat,
    EqualBool,
    EqualString,
    Jump(Label),
    JumpIf(Label),
    Call(usize, Label),
    CallFuncObj,
    CallExtern(u32),
    Return(u32),
    Stop,
    HostFunc(u16),
    Panic,
    ConstructStruct(u16),
    ConstructArray(usize),
    ConstructVariant { tag: u16 },
    DeconstructStruct,
    DeconstructVariant,
    GetField(u16),
    SetField(u16),
    GetIdx,
    SetIdx,
    MakeClosure { func_addr: Label },
    ArrayAppend,
    ArrayLength,
    ArrayPop,
    ConcatStrings,
    IntToString,
    FloatToString,
    LoadLib,
    LoadForeignFunc,
}
impl Display for Instr {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        match self {
            Instr::Pop => write!(f, "pop"),
            Instr::Duplicate => write!(f, "duplicate"),
            Instr::LoadOffset(n) => write!(f, "load_offset {n}"),
            Instr::StoreOffset(n) => write!(f, "store_offset {n}"),
            Instr::AddInt => write!(f, "add_int"),
            Instr::SubtractInt => write!(f, "subtract_int"),
            Instr::MultiplyInt => write!(f, "multiply_int"),
            Instr::DivideInt => write!(f, "divide_int"),
            Instr::PowerInt => write!(f, "power_int"),
            Instr::Modulo => write!(f, "modulo"),
            Instr::AddFloat => write!(f, "add_float"),
            Instr::SubtractFloat => write!(f, "subtract_float"),
            Instr::MultiplyFloat => write!(f, "multiply_float"),
            Instr::DivideFloat => write!(f, "divide_float"),
            Instr::PowerFloat => write!(f, "power_float"),
            Instr::SquareRoot => write!(f, "square_root"),
            Instr::Not => write!(f, "not"),
            Instr::And => write!(f, "and"),
            Instr::Or => write!(f, "or"),
            Instr::LessThanInt => write!(f, "less_than_int"),
            Instr::LessThanOrEqualInt => write!(f, "less_than_or_equal_int"),
            Instr::GreaterThanInt => write!(f, "greater_than_int"),
            Instr::GreaterThanOrEqualInt => write!(f, "greater_than_or_equal_int"),
            Instr::LessThanFloat => write!(f, "less_than_float"),
            Instr::LessThanOrEqualFloat => write!(f, "less_than_or_equal_float"),
            Instr::GreaterThanFloat => write!(f, "greater_than_float"),
            Instr::GreaterThanOrEqualFloat => write!(f, "greater_than_or_equal_float"),
            Instr::EqualInt => write!(f, "equal_int"),
            Instr::EqualFloat => write!(f, "equal_float"),
            Instr::EqualBool => write!(f, "equal_bool"),
            Instr::EqualString => write!(f, "equal_string"),
            Instr::PushNil(n) => write!(f, "push_nil {n}"),
            Instr::PushBool(b) => write!(f, "push_bool {b}"),
            Instr::PushInt(n) => write!(f, "push_int {n}"),
            Instr::PushFloat(n) => write!(f, "push_float {n}"),
            Instr::PushString(s) => write!(f, "push_string {:?}", s),
            Instr::Jump(loc) => write!(f, "jump {loc}"),
            Instr::JumpIf(loc) => write!(f, "jump_if {loc}"),
            Instr::Call(nargs, addr) => {
                write!(f, "call {} {}", nargs, addr)
            }
            Instr::CallExtern(func_id) => write!(f, "call_extern {func_id}"),
            Instr::CallFuncObj => write!(f, "call_func_obj"),
            Instr::Return(nargs) => write!(f, "return {nargs}"),
            Instr::Stop => write!(f, "stop"),
            Instr::Panic => write!(f, "panic"),
            Instr::ConstructStruct(n) => write!(f, "construct_struct {n}"),
            Instr::ConstructArray(n) => write!(f, "construct_array {n}"),
            Instr::ConstructVariant { tag } => {
                write!(f, "construct_variant {tag}")
            }
            Instr::DeconstructStruct => write!(f, "deconstruct_struct"),
            Instr::DeconstructVariant => write!(f, "deconstruct_variant"),
            Instr::GetField(n) => write!(f, "get_field {n}"),
            Instr::SetField(n) => write!(f, "set_field {n}"),
            Instr::GetIdx => write!(f, "get_index"),
            Instr::SetIdx => write!(f, "set_index"),
            Instr::MakeClosure { func_addr } => {
                write!(f, "make_closure {func_addr}")
            }
            Instr::ArrayAppend => write!(f, "array_append"),
            Instr::ArrayLength => write!(f, "array_len"),
            Instr::ArrayPop => write!(f, "array_pop"),
            Instr::ConcatStrings => write!(f, "concat_strings"),
            Instr::IntToString => write!(f, "int_to_string"),
            Instr::FloatToString => write!(f, "float_to_string"),
            Instr::HostFunc(n) => write!(f, "call_host {n}"),
            Instr::LoadLib => write!(f, "load_lib"),
            Instr::LoadForeignFunc => write!(f, "load_foreign_func"),
        }
    }
}
pub(crate) fn remove_labels(
    items: &Vec<Line>,
    constants: &ConstantsHolder,
) -> (Vec<VmInstr>, LabelMap) {
    let mut instructions: Vec<VmInstr> = vec![];
    let mut offset = 0;
    let mut label_to_idx: LabelMap = HashMap::default();
    for item in items.iter() {
        match item {
            Line::Instr { .. } => {
                offset += 1;
            }
            Line::Label(label) => {
                label_to_idx.insert(label.clone(), offset);
            }
        }
    }
    for item in items {
        if let Line::Instr { instr, .. } = item {
            instructions.push(instr_to_vminstr(instr, &label_to_idx, constants));
        }
    }
    (instructions, label_to_idx)
}
fn _get_label(s: &str) -> Option<String> {
    if s.ends_with(":") {
        Some(s[0..s.len() - 1].into())
    } else {
        None
    }
}
fn instr_to_vminstr(
    instr: &Instr,
    label_to_idx: &HashMap<Label, usize>,
    constants: &ConstantsHolder,
) -> VmInstr {
    match instr {
        Instr::Pop => VmInstr::Pop,
        Instr::Duplicate => VmInstr::Duplicate,
        Instr::LoadOffset(i) => VmInstr::LoadOffset(*i),
        Instr::StoreOffset(i) => VmInstr::StoreOffset(*i),
        Instr::AddInt => VmInstr::AddInt,
        Instr::SubtractInt => VmInstr::SubtractInt,
        Instr::MultiplyInt => VmInstr::MultiplyInt,
        Instr::DivideInt => VmInstr::DivideInt,
        Instr::PowerInt => VmInstr::PowerInt,
        Instr::Modulo => VmInstr::Modulo,
        Instr::AddFloat => VmInstr::AddFloat,
        Instr::SubtractFloat => VmInstr::SubtractFloat,
        Instr::MultiplyFloat => VmInstr::MultiplyFloat,
        Instr::DivideFloat => VmInstr::DivideFloat,
        Instr::PowerFloat => VmInstr::PowerFloat,
        Instr::SquareRoot => VmInstr::SquareRoot,
        Instr::Not => VmInstr::Not,
        Instr::And => VmInstr::And,
        Instr::Or => VmInstr::Or,
        Instr::LessThanInt => VmInstr::LessThanInt,
        Instr::LessThanOrEqualInt => VmInstr::LessThanOrEqualInt,
        Instr::GreaterThanInt => VmInstr::GreaterThanInt,
        Instr::GreaterThanOrEqualInt => VmInstr::GreaterThanOrEqualInt,
        Instr::LessThanFloat => VmInstr::LessThanFloat,
        Instr::LessThanOrEqualFloat => VmInstr::LessThanOrEqualFloat,
        Instr::GreaterThanFloat => VmInstr::GreaterThanFloat,
        Instr::GreaterThanOrEqualFloat => VmInstr::GreaterThanOrEqualFloat,
        Instr::EqualInt => VmInstr::EqualInt,
        Instr::EqualFloat => VmInstr::EqualFloat,
        Instr::EqualBool => VmInstr::EqualBool,
        Instr::EqualString => VmInstr::EqualString,
        Instr::PushNil(n) => VmInstr::PushNil(*n),
        Instr::PushBool(b) => VmInstr::PushBool(*b),
        Instr::PushInt(i) => VmInstr::PushInt(constants.int_constants.try_get_id(i).unwrap()),
        Instr::PushFloat(f) => VmInstr::PushFloat(constants.float_constants.try_get_id(f).unwrap()),
        Instr::PushString(s) => {
            VmInstr::PushString(constants.string_constants.try_get_id(s).unwrap())
        }
        Instr::Jump(label) => VmInstr::Jump(ProgramCounter::new(label_to_idx[label])),
        Instr::JumpIf(label) => VmInstr::JumpIf(ProgramCounter::new(label_to_idx[label])),
        Instr::Call(nargs, label) => VmInstr::Call(CallData::new(
            *nargs as u32,
            *label_to_idx
                .get(label)
                .unwrap_or_else(|| panic!("Could not find label: {label}")) as u32,
        )),
        Instr::CallExtern(func_id) => VmInstr::CallExtern(*func_id),
        Instr::CallFuncObj => VmInstr::CallFuncObj,
        Instr::Return(nargs) => VmInstr::Return(*nargs),
        Instr::Stop => VmInstr::Stop,
        Instr::Panic => VmInstr::Panic,
        Instr::ConstructStruct(n) => VmInstr::ConstructStruct(*n),
        Instr::ConstructArray(n) => VmInstr::ConstructArray(*n as u32),
        Instr::DeconstructStruct => VmInstr::DeconstructStruct,
        Instr::DeconstructVariant => VmInstr::DeconstructVariant,
        Instr::GetField(idx) => VmInstr::GetField(*idx),
        Instr::SetField(idx) => VmInstr::SetField(*idx),
        Instr::GetIdx => VmInstr::GetIdx,
        Instr::SetIdx => VmInstr::SetIdx,
        Instr::ConstructVariant { tag } => VmInstr::ConstructVariant { tag: *tag },
        Instr::MakeClosure { func_addr } => {
            VmInstr::MakeClosure {
                func_addr: ProgramCounter::new(label_to_idx[func_addr]),
            }
        }
        Instr::ArrayAppend => VmInstr::ArrayAppend,
        Instr::ArrayLength => VmInstr::ArrayLength,
        Instr::ArrayPop => VmInstr::ArrayPop,
        Instr::ConcatStrings => VmInstr::ConcatStrings,
        Instr::IntToString => VmInstr::IntToString,
        Instr::FloatToString => VmInstr::FloatToString,
        Instr::HostFunc(n) => VmInstr::HostFunc(*n),
        Instr::LoadLib => VmInstr::LoadLib,
        Instr::LoadForeignFunc => VmInstr::LoadForeignFunc,
    }
}