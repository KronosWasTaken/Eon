use crate::statics::typecheck::Reason;
use crate::statics::typecheck::TypeVar;
use crate::statics::typecheck::{Nominal, TypeKey};
use heck::ToSnakeCase;
use strum::AsRefStr;
use strum::IntoEnumIterator;
use strum::VariantArray;
use strum_macros::EnumIter;
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, VariantArray, AsRefStr,
)]
pub enum BuiltinOperation {
    AddInt,
    SubtractInt,
    MultiplyInt,
    DivideInt,
    PowerInt,
    SqrtInt,
    Modulo,
    AddFloat,
    SubtractFloat,
    MultiplyFloat,
    DivideFloat,
    PowerFloat,
    SqrtFloat,
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
    EqualString,
    IntToString,
    FloatToString,
    ConcatStrings,
    ArrayPush,
    ArrayLength,
    ArrayPop,
    Panic,
    Newline,
}
impl BuiltinOperation {
    pub(crate) fn enumerate() -> Vec<Self> {
        Self::iter().collect()
    }
    pub(crate) fn name(&self) -> String {
        self.as_ref().to_snake_case()
    }
    pub(crate) fn type_signature(&self) -> TypeVar {
        let reason = Reason::Builtin(*self);
        match self {
            BuiltinOperation::AddInt
            | BuiltinOperation::SubtractInt
            | BuiltinOperation::MultiplyInt
            | BuiltinOperation::DivideInt
            | BuiltinOperation::Modulo
            | BuiltinOperation::PowerInt => TypeVar::make_func(
                vec![
                    TypeVar::make_int(reason.clone()),
                    TypeVar::make_int(reason.clone()),
                ],
                TypeVar::make_int(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::SqrtInt => TypeVar::make_func(
                vec![TypeVar::make_int(reason.clone())],
                TypeVar::make_int(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::AddFloat
            | BuiltinOperation::SubtractFloat
            | BuiltinOperation::MultiplyFloat
            | BuiltinOperation::DivideFloat
            | BuiltinOperation::PowerFloat => TypeVar::make_func(
                vec![
                    TypeVar::make_float(reason.clone()),
                    TypeVar::make_float(reason.clone()),
                ],
                TypeVar::make_float(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::SqrtFloat => TypeVar::make_func(
                vec![TypeVar::make_float(reason.clone())],
                TypeVar::make_float(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::LessThanInt
            | BuiltinOperation::LessThanOrEqualInt
            | BuiltinOperation::GreaterThanInt
            | BuiltinOperation::GreaterThanOrEqualInt
            | BuiltinOperation::EqualInt => TypeVar::make_func(
                vec![
                    TypeVar::make_int(reason.clone()),
                    TypeVar::make_int(reason.clone()),
                ],
                TypeVar::make_bool(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::LessThanFloat
            | BuiltinOperation::LessThanOrEqualFloat
            | BuiltinOperation::GreaterThanFloat
            | BuiltinOperation::GreaterThanOrEqualFloat
            | BuiltinOperation::EqualFloat => TypeVar::make_func(
                vec![
                    TypeVar::make_float(reason.clone()),
                    TypeVar::make_float(reason.clone()),
                ],
                TypeVar::make_bool(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::EqualString => TypeVar::make_func(
                vec![
                    TypeVar::make_string(reason.clone()),
                    TypeVar::make_string(reason.clone()),
                ],
                TypeVar::make_bool(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::IntToString => TypeVar::make_func(
                vec![TypeVar::make_int(reason.clone())],
                TypeVar::make_string(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::FloatToString => TypeVar::make_func(
                vec![TypeVar::make_float(reason.clone())],
                TypeVar::make_string(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::ConcatStrings => TypeVar::make_func(
                vec![
                    TypeVar::make_string(reason.clone()),
                    TypeVar::make_string(reason.clone()),
                ],
                TypeVar::make_string(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::ArrayPush => {
                let a = TypeVar::empty();
                TypeVar::make_func(
                    vec![
                        TypeVar::make_nominal(reason.clone(), Nominal::Array, vec![a.clone()]),
                        a.clone(),
                    ],
                    TypeVar::make_void(reason.clone()),
                    reason.clone(),
                )
            }
            BuiltinOperation::ArrayLength => {
                let a = TypeVar::empty();
                TypeVar::make_func(
                    vec![TypeVar::make_nominal(
                        reason.clone(),
                        Nominal::Array,
                        vec![a.clone()],
                    )],
                    TypeVar::make_int(reason.clone()),
                    reason.clone(),
                )
            }
            BuiltinOperation::ArrayPop => {
                let a = TypeVar::empty();
                TypeVar::make_func(
                    vec![TypeVar::make_nominal(
                        reason.clone(),
                        Nominal::Array,
                        vec![a.clone()],
                    )],
                    TypeVar::make_void(reason.clone()),
                    reason.clone(),
                )
            }
            BuiltinOperation::Panic => TypeVar::make_func(
                vec![TypeVar::make_string(reason.clone())],
                TypeVar::make_never(reason.clone()),
                reason.clone(),
            ),
            BuiltinOperation::Newline => TypeVar::make_string(reason.clone()),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BuiltinType {
    Int,
    Bool,
    Float,
    Void,
    String,
    Tuple(u8),
}
impl BuiltinType {
    pub fn name(&self) -> &str {
        match self {
            Self::Int => "int",
            Self::Bool => "bool",
            Self::Float => "float",
            Self::Void => "void",
            Self::String => "string",
            Self::Tuple(_) => "tuple",
        }
    }
    pub fn to_type_key(self) -> TypeKey {
        match self {
            Self::Int => TypeKey::Int,
            Self::Bool => TypeKey::Bool,
            Self::Float => TypeKey::Float,
            Self::Void => TypeKey::Void,
            Self::String => TypeKey::String,
            Self::Tuple(arity) => TypeKey::Tuple(arity),
        }
    }
}