use {koto_lexer::Span, std::fmt};

#[derive(Debug)]
pub enum InternalError {
    AstCapacityOverflow,
    MissingScope,
    NumberParseFailure,
    FunctionParseFailure,
}

#[derive(Debug)]
pub enum SyntaxError {
    UnexpectedToken,
    UnexpectedIndentation,
    ExpectedExpression,
    ExpectedRhsExpression,
    ExpectedCloseParen,
    ExpectedListEnd,
    ExpectedMapSeparator,
    ExpectedMapValue,
    ExpectedMapEnd,
    ExpectedIfCondition,
    ExpectedThenKeywordOrBlock,
    ExpectedThenExpression,
    ExpectedElseIfCondition,
    ExpectedElseIfBlock,
    ExpectedElseExpression,
    ExpectedElseBlock,
    ExpectedAssignmentTarget,
    ExpectedFunctionArgsEnd,
    ExpectedCallArgsEnd,
    ExpectedRangeRhs,
    ExpectedForArgs,
    ExpectedForInKeyword,
    ExpectedForRanges,
    ExpectedForCondition,
    ExpectedForBody,
}

#[derive(Debug)]
pub enum ErrorType {
    InternalError(InternalError),
    SyntaxError(SyntaxError),

    // To be removed
    PestSyntaxError(String),
    OldParserError(String),
}

impl From<InternalError> for ErrorType {
    fn from(e: InternalError) -> ErrorType {
        ErrorType::InternalError(e)
    }
}

impl From<SyntaxError> for ErrorType {
    fn from(e: SyntaxError) -> ErrorType {
        ErrorType::SyntaxError(e)
    }
}

#[derive(Debug)]
pub struct ParserError {
    pub error: ErrorType,
    pub span: Span,
}

impl ParserError {
    pub fn new(error: ErrorType, span: Span) -> Self {
        Self { error, span }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorType::*;

        match &self.error {
            InternalError(error) => write!(f, "Internal error {}: {}", self.span.start, error),
            SyntaxError(error) => write!(f, "Syntax error {}: {}", self.span.start, error),

            PestSyntaxError(error) => f.write_str(&error),
            OldParserError(error) => f.write_str(&error),
        }
    }
}
impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InternalError::*;

        match self {
            AstCapacityOverflow => {
                f.write_str("There are more nodes in the program than the AST can support")
            }
            MissingScope => f.write_str("Scope unavailable during parsing"),
            NumberParseFailure => f.write_str("Failed to parse number"),
            FunctionParseFailure => f.write_str("Failed to parse function"),
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SyntaxError::*;

        match self {
            UnexpectedToken => f.write_str("Unexpected Token"),
            UnexpectedIndentation => f.write_str("Unexpected indentation level"),
            ExpectedExpression => f.write_str("Expected expression"),
            ExpectedRhsExpression => f.write_str("Expected expression"),
            ExpectedCloseParen => f.write_str("Expected closing parenthesis"),
            ExpectedListEnd => f.write_str("Unexpected token while in List, expected ']'"),
            ExpectedMapSeparator => f.write_str("Expected key/value separator ':' in Map"),
            ExpectedMapValue => f.write_str("Expected value after ':' in Map"),
            ExpectedMapEnd => f.write_str("Unexpected token in Map, expected '}'"),
            ExpectedIfCondition => f.write_str("Expected condition in if expression"),
            ExpectedThenKeywordOrBlock => f.write_str(
                "Error parsing if expression, expected 'then' keyword or indented block.",
            ),
            ExpectedThenExpression => f.write_str("Expected 'then' expression."),
            ExpectedElseIfCondition => f.write_str("Expected condition for 'else if'."),
            ExpectedElseIfBlock => f.write_str("Expected indented block for 'else if'."),
            ExpectedElseExpression => f.write_str("Expected 'else' expression."),
            ExpectedElseBlock => f.write_str("Expected indented block for 'else'."),
            ExpectedAssignmentTarget => f.write_str("Expected target for assignment"),
            ExpectedFunctionArgsEnd => f.write_str("Expected end of function arguments '|'"),
            ExpectedCallArgsEnd => f.write_str("Expected end of function call arguments '|'"),
            ExpectedRangeRhs => f.write_str("Expected end expression for range"),
            ExpectedForArgs => f.write_str("Expected arguments in for loop"),
            ExpectedForInKeyword => f.write_str("Expected in keyword in for loop"),
            ExpectedForRanges => f.write_str("Expected ranges in for loop"),
            ExpectedForCondition => f.write_str("Expected condition after 'if' in for loop"),
            ExpectedForBody => f.write_str("Expected indented block in for loop"),
        }
    }
}
