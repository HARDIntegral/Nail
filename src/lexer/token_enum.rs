#[derive(Copy, Clone, Debug)]
pub enum TokenTypes {
    ImportStatement,
    FunctionDeclarationKeyword,
    StructDeclarationKeyword,
    EnumDeclarationKeyword,
    ReturnKeyword,

    IfKeyword,
    ElseKeyword,
    ElifKeyword,
    WhileKeyword,
    ForKeyword,
    InLoopRangeKeyword,
    RangeSymbol,

    IntTypeKeyword,
    DoubleTypeKeyword,
    BoolTypeKeyword,
    CharTypeKeyword,
    StringTypeKeyword,
    NullTypeKeyword,
    TrueKeyword,
    FalseKeyword,

    OpenBracSymbol,
    CloseBracSymbol,
    OpenCurlSymbol,
    CloseCurlSymbol,
    OpenParenSymbol,
    CloseParenSymbol,

    EndOfLineSymbol,
    CommaSeperSymbol,
    ColonSperSymbol,
    DoubleColonSeperSymbol,

    ReturnTypeIdentifier,

    SetOperator,
    LessThanOperator,
    GreaterThanOperator,
    LessEqualOperator,
    GreaterEqualOperator,
    AddOperator,
    SubOperator,
    MultOperator,
    DivOperator,
    ModOperator,

    AddSetOperator,
    SubSetOperator,
    MultSetOperator,
    DivSetOperator,
    ModSetOperator,

    Increment,
    Decrement,

    EndOfFileIdentifier,
    OtherIdentfier,
    NULL,
}
