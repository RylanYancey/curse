
use super::token::Token;

pub struct Lexer<'s> {
	/// The string being lexed.
	src: &'s str,

	/// Cursor over 'src'.
	curr: usize,

	/// The current line number.	
	line: usize,

	/// The num of tokens returned
	/// while in the current line.
	token: usize,
}

impl<'s> Lexer<'s> {
	pub fn new(src: &'s str) -> Self {
		Self {
			src,
			curr: 0,
			line: 1,
			token: 0,
		}
	}

	fn char(&self) -> char {
		self.src.as_bytes()[self.curr] as char
	}

	fn len(&self) -> usize {
		self.src.len()
	}

	fn peek(&self) -> Option<char> {
		self.src.as_bytes().get(self.curr + 1).map(|n| *n as char)
	}

	fn slice(&self, start: usize) -> &'s str {
		&self.src[start..self.curr]
	}

	/// Increment the token, returning
	/// what it was before.
	fn inc_token(&mut self) -> usize {
		let result = self.token;
		self.token += 1;
		result
	}
}

impl<'s> Iterator for Lexer<'s> {
	// 'Token', 'line', 'token_in_line'.
	type Item = (Token<'s>, usize, usize);

	fn next(&mut self) -> Option<Self::Item> {
		'outer: loop {
			let start = self.curr;

			if self.curr == self.src.len() {
				return None;
			}

			match self.char() {
				'\n' => {
					// advance the line counter and
					// reset the token-per-line count.
					self.curr += 1;
					self.line += 1;
					self.token = 0;
					continue 'outer;
				}

				// whitespace is nothing.
				c if c.is_whitespace() => {
					self.curr += 1;
					continue 'outer;
				}

				// comment checking. 
				'/' => {
					match self.peek() {
						// single-line comment.
						Some('/') => {
							self.curr += 1;
	
							loop {
								self.curr += 1;
	
								if self.curr == self.len() {
									return None;
								}
	
								if self.char() == '\n' {
									self.line += 1;
									self.curr += 1;
									self.token = 0;
									continue 'outer;
								}
							}
						},
	
						// multi-line comment.
						Some('*') => {
							self.curr += 1;
	
							loop {
								self.curr += 1;
	
								if self.curr == self.len() {
									return None;
								}
	
								if self.char() == '\n' {
									self.line += 1;
									self.token = 0;
								} else {
									if self.char() == '*' && self.peek() == Some('/') {
										self.curr += 2; 
										continue 'outer;
									}
								}
							}
						},
						// symbol 
						_ => {
							self.curr += 1;
							return Some((
								Token::Symbol(self.char()), 
								self.line, 
								self.inc_token(), 
							))
						}
					}
				}

				// An identifier starts with A-Za-z_
				c if is_valid_ident_start(c) => {
					// loop until EOF or a character that is
					// not valid for an identifier. 
					loop {
						self.curr += 1;

						// return whatever we have so far if EOF.
						if self.curr == self.len() {
							return Some((
								Token::Ident(&self.slice(start)), 
								self.line,
								self.inc_token(),
							));
						}

						// stop tracking for an ident when
						// a non-ident compatible char is found.
						if !is_valid_for_ident(self.char()) {
							return Some((
								Token::Ident(&self.slice(start)), 
								self.line,
								self.inc_token(),
							));
						}
					}
				}

				// if a token starts with a digit, 
				// then it is a numeric literal.
				c if is_valid_numlit_start(c) => {
					// loop until EOF or a character that
					// is not valid for a numeric literal.
					loop {
						self.curr += 1;

						if self.curr == self.len() {
							return Some((
								Token::NumLit(&self.slice(start)), 
								self.line,
								self.inc_token(),
							));
						}

						if !is_valid_for_numlit(self.char()) {
							return Some((
								Token::NumLit(&self.slice(start)), 
								self.line,
								self.inc_token(),
							));
						}
					}
				}
				// its' a symbol.
				c => {
					self.curr += 1;
					return Some((
						Token::Symbol(c), 
						self.line,
						self.inc_token(),
					));
				}
			}
		}
	}
}

// Ident starts must be _ or A-Za-z.
// Starts must not be a number.  
fn is_valid_ident_start(c: char) -> bool {
	c.is_ascii_alphabetic() || c == '_'
}

// The content of an ident is the same
// as the ident start, but an ident
// can have numbers in its content. 
// '_', 'A-Z', 'a-z', or '0-9'.
fn is_valid_for_ident(c: char) -> bool {
	c.is_ascii_alphanumeric() || c == '_'
}

// Numeric Literals ALWAYS start with 0-9. 
fn is_valid_numlit_start(c: char) -> bool {
	c.is_ascii_digit()
}

// a numeric literal will keep
// going until a char that is not
// alphanumeric or _. This allows
// syntax like '0xFF3DB_i32', a hex
// lit with a type to infer.
fn is_valid_for_numlit(c: char) -> bool {
	c.is_ascii_alphanumeric() ||
	c == '_'
}

#[cfg(test)]
mod tests {
	use super::Lexer;
	use super::Token;

	#[test]
	fn lexer() {
		let src = "
			#[kernel]
			fn vec_add<F: Float>(a: &[F], b: &[F], c: &mut [F]) {
				let index = thread.x;

				if index < a.len() {
					c[index] = a[index] + b[index];
				}
			}
		";

		let expected = "
			#_[_kernel_]_
			fn_vec_add_<_F_:_Float_>_(_a_:_&_[_F_]_,_b_:_&_[_F_]_,_c_:_&_mut_[_F_]_)_{_
				let_index_=_thread_._x_;_
				
				if_index_<_a_._len_(_)_{
					_c_[_index_]_=_a_[_index_]_+_b_[_index_]_;_
				}_
			}_
		"
			.replace('\t', "")
			.replace('\n', "")
			.replace(' ', "");


		let mut output = String::new();

		for (token, _, _) in Lexer::new(src) {
			match token {
				Token::Ident(ident) => {
					output.push_str(ident);
					output.push('_');
				},
				Token::NumLit(lit) => {
					output.push_str(lit);
					output.push('_');
				},
				Token::Symbol(symbol) => {
					output.push(symbol);
					output.push('_');
				}
			}
		}

		assert_eq!(expected, output);
	}

	#[test]
	// test if line tracking is working correctly.
	fn lexer_line_tracking() {
		let src = "
			#[kernel]
			fn vec_add<F: Float>(a: &[F], b: &[F], c: &mut [F]) {
				let index = thread.x;

				if index < a.len() {
					c[index] = a[index] + b[index];
				}
			}
		";

		// total number of tokens on each line.
		let expected = vec![4, 31, 7, 9, 15, 1, 1];

		let mut line_sums = Vec::new();
		let mut curr_line = 0;

		for (_, line, _) in Lexer::new(src) {
			if line != curr_line {
				line_sums.push(0);
				curr_line = line;
			}

			*line_sums.last_mut().unwrap() += 1;
		}

		assert_eq!(expected, line_sums);
	}

	#[test]
	fn lexer_comment_ignoring() {
		let src = "
			#[kernel /* my super epic comment */ ]
			fn vec_add<F: /* my other super epic comment */ Float>(a: &[F], b: &[F], c: &mut [F]) {
				let index = thread.x;
				/*
					a super epic multi-line comment!
					nobody can stop me now!
				*/
				// more single line comments
				if index < a.len() { // single-line comment
					c[index] = a[index] + /* hi */ b[index];
				} // single line comments.
			}
		";

		let expected = "
			#_[_kernel_]_
			fn_vec_add_<_F_:_Float_>_(_a_:_&_[_F_]_,_b_:_&_[_F_]_,_c_:_&_mut_[_F_]_)_{_
				let_index_=_thread_._x_;_
				
				if_index_<_a_._len_(_)_{
					_c_[_index_]_=_a_[_index_]_+_b_[_index_]_;_
				}_
			}_
		"
			.replace('\t', "")
			.replace('\n', "")
			.replace(' ', "");


		let mut output = String::new();

		for (token, _, _) in Lexer::new(src) {
			match token {
				Token::Ident(ident) => {
					output.push_str(ident);
					output.push('_');
				},
				Token::NumLit(lit) => {
					output.push_str(lit);
					output.push('_');
				},
				Token::Symbol(symbol) => {
					output.push(symbol);
					output.push('_');
				}
			}
		}

		assert_eq!(expected, output);
	}
}