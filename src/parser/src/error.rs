use {koto_lexer::Span, std::fmt};

#[derive(Clone, Debug)]
pub enum InternalError {
    ArgumentsParseFailure,
    AstCapacityOverflow,
    ExpectedIdInImportItem,
    ForParseFailure,
    FunctionParseFailure,
    IdParseFailure,
    LookupParseFailure,
    MissingAssignmentTarget,
    MissingContinuedExpressionLhs,
    MissingScope,
    NumberParseFailure,
    RangeParseFailure,
    UnexpectedIdInExpression,
    UnexpectedToken,
}

/// Errors that arise from expecting an indented block
///
/// Having these errors separated out is useful for the interactive input,
/// where an indented continuation can be started in response to an indentation error.
#[derive(Clone, Debug)]
pub enum ExpectedIndentation {
    ExpectedCatchBody,
    ExpectedElseBlock,
    ExpectedElseIfBlock,
    ExpectedFinallyBody,
    ExpectedForBody,
    ExpectedFunctionBody,
    ExpectedLoopBody,
    ExpectedMatchArm,
    ExpectedRhsExpression,
    ExpectedThenKeywordOrBlock,
    ExpectedTryBody,
    ExpectedUntilBody,
    ExpectedWhileBody,
}

#[derive(Clone, Debug)]
pub enum SyntaxError {
    ExpectedArgsEnd,
    ExpectedAssignmentTarget,
    ExpectedCatchArgument,
    ExpectedCatch,
    ExpectedCloseParen,
    ExpectedElseExpression,
    ExpectedElseIfCondition,
    ExpectedEndOfLine,
    ExpectedExportExpression,
    ExpectedExpression,
    ExpectedExpressionInMainBlock,
    ExpectedForArgs,
    ExpectedForCondition,
    ExpectedForInKeyword,
    ExpectedForRanges,
    ExpectedFunctionArgsEnd,
    ExpectedIdInImportExpression,
    ExpectedIfCondition,
    ExpectedImportKeywordAfterFrom,
    ExpectedImportModuleId,
    ExpectedIndentedLookupContinuation,
    ExpectedIndexEnd,
    ExpectedIndexExpression,
    ExpectedListEnd,
    ExpectedMapEnd,
    ExpectedMapKey,
    ExpectedMapValue,
    ExpectedMatchArmExpression,
    ExpectedMatchArmExpressionAfterThen,
    ExpectedMatchCondition,
    ExpectedMatchExpression,
    ExpectedMatchPattern,
    ExpectedNegatableExpression,
    ExpectedThenExpression,
    ExpectedUntilCondition,
    ExpectedWhileCondition,
    ImportFromExpressionHasTooManyItems,
    LexerError,
    MatchEllipsisOutsideOfNestedPatterns,
    SelfArgNotInFirstPosition,
    TooManyNum2Terms,
    TooManyNum4Terms,
    UnexpectedEscapeInString,
    UnexpectedToken,
    UnexpectedTokenAfterExportId,
    UnexpectedTokenInImportExpression,
}

#[derive(Clone, Debug)]
pub enum ErrorType {
    InternalError(InternalError),
    ExpectedIndentation(ExpectedIndentation),
    SyntaxError(SyntaxError),
}

impl From<InternalError> for ErrorType {
    fn from(e: InternalError) -> ErrorType {
        ErrorType::InternalError(e)
    }
}

impl From<ExpectedIndentation> for ErrorType {
    fn from(e: ExpectedIndentation) -> ErrorType {
        ErrorType::ExpectedIndentation(e)
    }
}

impl From<SyntaxError> for ErrorType {
    fn from(e: SyntaxError) -> ErrorType {
        ErrorType::SyntaxError(e)
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorType::*;

        match &self {
            InternalError(error) => write!(f, "Internal error: {}", error),
            ExpectedIndentation(error) => f.write_str(&error.to_string()),
            SyntaxError(error) => f.write_str(&error.to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParserError {
    pub error: ErrorType,
    pub span: Span,
}

impl ParserError {
    pub fn new(error: ErrorType, span: Span) -> Self {
        Self { error, span }
    }
}

pub fn is_indentation_error(error: &ParserError) -> bool {
    matches!(error.error, ErrorType::ExpectedIndentation(_))
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InternalError::*;

        match self {
            ArgumentsParseFailure => f.write_str("Failed to parse arguments"),
            AstCapacityOverflow => {
                f.write_str("There are more nodes in the program than the AST can support")
            }
            ExpectedIdInImportItem => f.write_str("Expected ID in import item"),
            ForParseFailure => f.write_str("Failed to parse for loop"),
            FunctionParseFailure => f.write_str("Failed to parse function"),
            IdParseFailure => f.write_str("Failed to parse ID"),
            LookupParseFailure => f.write_str("Failed to parse lookup"),
            MissingAssignmentTarget => f.write_str("Missing assignment target"),
            MissingContinuedExpressionLhs => f.write_str("Missing LHS for continued expression"),
            MissingScope => f.write_str("Scope unavailable during parsing"),
            NumberParseFailure => f.write_str("Failed to parse number"),
            RangeParseFailure => f.write_str("Failed to parse range"),
            UnexpectedIdInExpression => {
                f.write_str("Unexpected ID encountered while parsing expression")
            }
            UnexpectedToken => f.write_str("Unexpected token"),
        }
    }
}

impl fmt::Display for ExpectedIndentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ExpectedIndentation::*;

        match self {
            ExpectedCatchBody => f.write_str("Expected indented block for catch expression"),
            ExpectedElseBlock => f.write_str("Expected indented block for 'else'."),
            ExpectedElseIfBlock => f.write_str("Expected indented block for 'else if'."),
            ExpectedForBody => f.write_str("Expected indented block in for loop"),
            ExpectedFinallyBody => f.write_str("Expected indented block for finally expression"),
            ExpectedFunctionBody => f.write_str("Expected function body"),
            ExpectedLoopBody => f.write_str("Expected indented block in loop"),
            ExpectedMatchArm => f.write_str("Expected indented arm for match expression"),
            ExpectedRhsExpression => f.write_str("Expected expression"),
            ExpectedThenKeywordOrBlock => f.write_str(
                "Error parsing if expression, expected 'then' keyword or indented block.",
            ),
            ExpectedTryBody => f.write_str("Expected indented block for try expression"),
            ExpectedUntilBody => f.write_str("Expected indented block in until loop"),
            ExpectedWhileBody => f.write_str("Expected indented block in while loop"),
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SyntaxError::*;

        match self {
            ExpectedArgsEnd => f.write_str("Expected end of arguments ')'"),
            ExpectedAssignmentTarget => f.write_str("Expected target for assignment"),
            ExpectedCatchArgument => f.write_str("Expected argument for catch expression"),
            ExpectedCatch => f.write_str("Expected catch expression after try"),
            ExpectedCloseParen => f.write_str("Expected closing parenthesis"),
            ExpectedElseExpression => f.write_str("Expected 'else' expression."),
            ExpectedElseIfCondition => f.write_str("Expected condition for 'else if'."),
            ExpectedEndOfLine => f.write_str("Expected end of line"),
            ExpectedExportExpression => f.write_str("Expected ID to export"),
            ExpectedExpression => f.write_str("Expected expression"),
            ExpectedExpressionInMainBlock => f.write_str("Expected expression"),
            ExpectedForArgs => f.write_str("Expected arguments in for loop"),
            ExpectedForCondition => f.write_str("Expected condition after 'if' in for loop"),
            ExpectedForInKeyword => f.write_str("Expected in keyword in for loop"),
            ExpectedForRanges => f.write_str("Expected ranges in for loop"),
            ExpectedFunctionArgsEnd => f.write_str("Expected end of function arguments '|'"),
            ExpectedIdInImportExpression => f.write_str("Expected ID in import expression"),
            ExpectedIfCondition => f.write_str("Expected condition in if expression"),
            ExpectedImportKeywordAfterFrom => f.write_str("Expected 'import' after 'from' ID"),
            ExpectedImportModuleId => f.write_str("Expected module ID in import expression"),
            ExpectedIndentedLookupContinuation => {
                f.write_str("Expected indented lookup continuation")
            }
            ExpectedIndexEnd => f.write_str("Unexpected token while indexing a List, expected ']'"),
            ExpectedIndexExpression => f.write_str("Expected index expression"),
            ExpectedListEnd => f.write_str("Unexpected token while in List, expected ']'"),
            ExpectedMapEnd => f.write_str("Unexpected token in Map, expected '}'"),
            ExpectedMapKey => f.write_str("Expected key after '.' in Map access"),
            ExpectedMapValue => f.write_str("Expected value after ':' in Map"),
            ExpectedMatchArmExpression => f.write_str("Expected expression in match arm"),
            ExpectedMatchArmExpressionAfterThen => {
                f.write_str("Expected expression after then in match arm")
            }
            ExpectedMatchCondition => f.write_str("Expected condition after if in match arm"),
            ExpectedMatchExpression => f.write_str("Expected expression after match"),
            ExpectedMatchPattern => f.write_str("Expected pattern for match arm"),
            ExpectedNegatableExpression => f.write_str("Expected negatable expression"),
            ExpectedThenExpression => f.write_str("Expected 'then' expression."),
            ExpectedUntilCondition => f.write_str("Expected condition in until loop"),
            ExpectedWhileCondition => f.write_str("Expected condition in while loop"),
            ImportFromExpressionHasTooManyItems => {
                f.write_str("Too many items listed after 'from' in import expression")
            }
            LexerError => f.write_str("Found an unexpected token while lexing input"),
            MatchEllipsisOutsideOfNestedPatterns => {
                f.write_str("Ellipsis found outside of nested match patterns")
            }
            SelfArgNotInFirstPosition => f.write_str("self is only allowed as the first argument"),
            TooManyNum2Terms => f.write_str("num2 only supports up to 2 terms"),
            TooManyNum4Terms => f.write_str("num4 only supports up to 4 terms"),
            UnexpectedEscapeInString => f.write_str("Unexpected escape pattern in string"),
            UnexpectedToken => f.write_str("Unexpected token"),
            UnexpectedTokenAfterExportId => f.write_str("Unexpected token after export ID"),
            UnexpectedTokenInImportExpression => {
                f.write_str("Unexpected token in import expression")
            }
        }
    }
}
