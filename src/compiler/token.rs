
pub enum Token<'s> {
	Ident(&'s str),
	NumLit(&'s str),
	Symbol(char),
}

impl<'s> Token<'s> {
	pub fn to_string(&self) -> String {
		match self {
			Token::Ident(str) => str.to_string(),
			Token::NumLit(str) => str.to_string(),
			Token::Symbol(char) => char.to_string(),
		}
	}
}

pub enum Keyword {
	Fn,
	Let,
	If,
	For,
	Return,
	Break,
	Const,
	Continue,
	Else,
	Global,
	Local,
	Mut,
	True,
	False,
	While,
	Match,
	Loop,
	In,
}

pub enum Operator {
	BitAnd,
	BitAndEq,
	BitOr,
	BitOrEq,
	BitXor,
	BitXorEq,
	LeftShift,
	LeftShiftEq,
	RightShift,
	RightShiftEq,
	Constraint,
	Terminator,
	GreaterThan,
	LessThen,
	EqualTo,
	GreaterThanEq,
	LessThanEq,
	Assignment,
	Not,
	NotEq,
	Rem,
	RemEq,
	Borrow,
	Mul,
	MulEq,
	Deref,
	Add,
	AddEq,
	Sub,
	SubEq,
	Div,
	DivEq,
	RangeExclusive,
	RangeInclusive,
	ReturnType,
	Access,
	Neg,
	Separator,
}

pub enum Delimiter {
	RCurly,
	LCurly,
	RBracket,
	LBracket,
	RParen,
	LParen,
}