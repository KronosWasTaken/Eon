use crate::ast::{
    AstNode, EnumDef, FileAst, FileDatabase, FileId, FuncDecl, FuncDef, InterfaceDef,
    InterfaceImpl, InterfaceOutputType, NodeId, Polytype, StructDef, Type as AstType, TypeKind,
};
use crate::builtin::{BuiltinOperation, BuiltinType};
use crate::{ErrorSummary, FileProvider};
use resolve::{resolve, scan_declarations};
use std::fmt::{self, Display, Formatter};
use std::ops::Range;
use std::path::PathBuf;
use std::rc::Rc;
use typecheck::{ConstraintReason, PotentialType, SolvedType, TypeKey, TypeVar, solve_types};
use utils::hash::{HashMap, HashSet};
use utils::id_set::IdSet;
mod error;
mod pat_exhaustiveness;
mod resolve;
pub(crate) mod typecheck;
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use pat_exhaustiveness::{DeconstructedPat, check_pattern_exhaustiveness_and_usefulness};
pub(crate) use typecheck::Prov as TypeProv;
pub(crate) use typecheck::SolvedType as Type;
pub(crate) struct StaticsContext {
    _files: FileDatabase,
    _file_provider: Box<dyn FileProvider>,
    pub(crate) root_namespace: Namespace,
    pub(crate) resolution_map: HashMap<NodeId, Declaration>,
    pub(crate) fully_qualified_names: HashMap<NodeId, String>,
    pub(crate) interface_namespaces: HashMap<Rc<InterfaceDef>, Rc<Namespace>>,
    pub(crate) loop_stack: Vec<Option<NodeId>>,
    pub(crate) func_ret_stack: Vec<TypeProv>,
    pub(crate) interface_impls: HashMap<Rc<InterfaceDef>, Vec<Rc<InterfaceImpl>>>,
    pub(crate) interface_impl_analyzed: HashSet<Rc<InterfaceImpl>>,
    pub(crate) interface_def_analyzed: HashSet<Rc<InterfaceDef>>,
    pub(crate) member_functions: HashMap<(TypeKey, String), Declaration>,
    pub(crate) for_loop_make_iterator_types: HashMap<NodeId, SolvedType>,
    pub(crate) for_loop_next_types: HashMap<NodeId, SolvedType>,
    pub(crate) dylibs: IdSet<PathBuf>,
    pub(crate) dylib_to_funcs: HashMap<u32, IdSet<String>>,
    pub(crate) host_funcs: IdSet<Rc<FuncDecl>>,
    pub(crate) unifvars: HashMap<TypeProv, TypeVar>,
    errors: Vec<Error>,
}
impl StaticsContext {
    fn new(files: FileDatabase, file_provider: Box<dyn FileProvider>) -> Self {
        Self {
            _files: files,
            _file_provider: file_provider,
            root_namespace: Default::default(),
            resolution_map: Default::default(),
            fully_qualified_names: Default::default(),
            interface_namespaces: Default::default(),
            loop_stack: Default::default(),
            func_ret_stack: Default::default(),
            interface_impls: Default::default(),
            interface_impl_analyzed: Default::default(),
            interface_def_analyzed: Default::default(),
            member_functions: Default::default(),
            for_loop_make_iterator_types: Default::default(),
            for_loop_next_types: Default::default(),
            dylibs: Default::default(),
            dylib_to_funcs: Default::default(),
            host_funcs: IdSet::new(),
            unifvars: Default::default(),
            errors: Default::default(),
        }
    }
    pub(crate) fn solution_of_node(&self, node: AstNode) -> Option<SolvedType> {
        let prov = TypeProv::Node(node);
        match self.unifvars.get(&prov) {
            Some(unifvar) => unifvar.solution(),
            None => None,
        }
    }
    pub(crate) fn get_interface_declaration(&self, name: &str) -> Rc<InterfaceDef> {
        if let Some(Declaration::InterfaceDef(iface_def)) =
            self.root_namespace.get_declaration(name)
        {
            iface_def.clone()
        } else {
            panic!("Interface '{}' not found in root namespace. Available namespaces: {:?}", 
                   name, self.root_namespace.namespaces.keys().collect::<Vec<_>>())
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Namespace {
    declarations: HashMap<String, Declaration>,
    namespaces: HashMap<String, Rc<Namespace>>,
}
impl Namespace {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_declaration(&self, path: &str) -> Option<Declaration> {
        let segments: Vec<_> = path.split('.').collect();
        let mut current_namespace: &Namespace = self;
        for segment in &segments[0..segments.len() - 1] {
            if let Some(ns) = current_namespace.namespaces.get(*segment) {
                current_namespace = ns;
            } else {
                return None;
            }
        }
        current_namespace
            .declarations
            .get(*segments.last()?)
            .cloned()
    }
}
impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn fmt_tree(tree: &Namespace, f: &mut Formatter, indent: usize) -> fmt::Result {
            for ident in tree.declarations.keys() {
                writeln!(f, "{:indent$}{}", "", ident)?;
            }
            for (ident, subtree) in &tree.namespaces {
                writeln!(f, "{:indent$}{}", "", ident)?;
                fmt_tree(subtree, f, indent + 2)?;
            }
            Ok(())
        }
        fmt_tree(self, f, 0)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Declaration {
    FreeFunction(Rc<FuncDef>),
    HostFunction(Rc<FuncDecl>),
    _ForeignFunction {
        f: Rc<FuncDecl>,
        libname: PathBuf,
        symbol: String,
    },
    InterfaceDef(Rc<InterfaceDef>),
    InterfaceMethod {
        iface: Rc<InterfaceDef>,
        method: usize,
    },
    InterfaceOutputType {
        iface: Rc<InterfaceDef>,
        ty: Rc<InterfaceOutputType>,
    },
    MemberFunction {
        f: Rc<FuncDef>,
    },
    Enum(Rc<EnumDef>),
    EnumVariant {
        e: Rc<EnumDef>,
        variant: usize,
    },
    Struct(Rc<StructDef>),
    Array,
    Builtin(BuiltinOperation),
    BuiltinType(BuiltinType),
    Var(AstNode),
    Polytype(PolytypeDeclaration),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum PolytypeDeclaration {
    InterfaceSelf(Rc<InterfaceDef>),
    Ordinary(Rc<Polytype>),
}
type InterfaceArguments = Vec<(Rc<InterfaceOutputType>, Rc<AstType>, SolvedType)>;
impl Declaration {
    pub fn into_type_key(self: Declaration) -> Option<TypeKey> {
        match self {
            Declaration::FreeFunction(_)
            | Declaration::HostFunction(_)
            | Declaration::_ForeignFunction { .. }
            | Declaration::InterfaceDef(_)
            | Declaration::InterfaceMethod { .. }
            | Declaration::MemberFunction { .. }
            | Declaration::Builtin(_)
            | Declaration::Var(_)
            | Declaration::Polytype(_)
            | Declaration::EnumVariant { .. } => None,
            Declaration::InterfaceOutputType { .. } => unimplemented!(),
            Declaration::Enum(enum_def) => Some(TypeKey::TyApp(Nominal::Enum(enum_def))),
            Declaration::Struct(struct_def) => Some(TypeKey::TyApp(Nominal::Struct(struct_def))),
            Declaration::Array => Some(TypeKey::TyApp(Nominal::Array)),
            Declaration::BuiltinType(builtin_type) => Some(builtin_type.to_type_key()),
        }
    }
}
#[derive(Debug, Clone)]
pub(crate) enum Error {
    Generic {
        msg: String,
        node: AstNode,
    },
    UnresolvedIdentifier {
        node: AstNode,
    },
    UnresolvedMemberFunction {
        receiver_node: AstNode,
        memfn_node: AstNode,
        ty: PotentialType,
    },
    NameClash {
        name: String,
        original: Declaration,
        new: Declaration,
    },
    UnconstrainedUnifvar {
        node: AstNode,
    },
    ConflictingUnifvar {
        types: HashMap<TypeKey, PotentialType>,
    },
    TypeConflict {
        ty1: PotentialType,
        ty2: PotentialType,
        constraint_reason: ConstraintReason,
    },
    MemberAccessNeedsAnnotation {
        node: AstNode,
    },
    MustExtendType {
        node: AstNode,
    },
    MemberFunctionMissingFirstSelfArgument {
        node: AstNode,
    },
    UnqualifiedEnumNeedsAnnotation {
        node: AstNode,
    },
    InterfaceNotImplemented {
        ty: SolvedType,
        iface: Rc<InterfaceDef>,
        node: AstNode,
    },
    InterfaceImplTypeNotGeneric {
        node: AstNode,
    },
    NotInLoop {
        node: AstNode,
    },
    CantReturnHere {
        node: AstNode,
    },
    NonexhaustiveMatch {
        node: AstNode,
        missing: Vec<DeconstructedPat>,
    },
    RedundantArms {
        node: AstNode,
        redundant_arms: Vec<AstNode>,
    },
    #[cfg(not(feature = "ffi"))]
    FfiNotEnabled(AstNode),
}
pub(crate) fn analyze(
    file_asts: &Vec<Rc<FileAst>>,
    files: &FileDatabase,
    file_provider: Box<dyn FileProvider>,
) -> Result<StaticsContext, ErrorSummary> {
    let mut ctx = StaticsContext::new(files.clone(), file_provider);
    scan_declarations(&mut ctx, file_asts);
    resolve(&mut ctx, file_asts);
    solve_types(&mut ctx, file_asts);
    check_pattern_exhaustiveness_and_usefulness(&mut ctx, file_asts);
    check_errors(&ctx, files)?;
    Ok(ctx)
}
pub(crate) fn check_errors(ctx: &StaticsContext, files: &FileDatabase) -> Result<(), ErrorSummary> {
    if ctx.errors.is_empty() {
        return Ok(());
    }
    Err(ErrorSummary {
        msg: "".to_string(),
        more: Some((files.clone(), ctx.errors.clone())),
    })
}
use crate::statics::typecheck::Nominal;
use codespan_reporting::diagnostic::Label as CsLabel;
impl AstNode {
    fn get_file_and_range(&self) -> (FileId, Range<usize>) {
        let loc = self.location();
        (loc.file_id, loc.range())
    }
}
pub(crate) fn _print_node(ctx: &StaticsContext, node: AstNode) {
    let (file, range) = node.get_file_and_range();
    let diagnostic = Diagnostic::note().with_labels(vec![CsLabel::secondary(file, range)]);
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();
    term::emit_to_io_write(&mut writer.lock(), &config, &ctx._files, &diagnostic).unwrap();
}