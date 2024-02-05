pub enum Operand {
    Add {
        a: Expr,
        b: Expr,
    },
    Min {
        a: Expr,
        b: Expr,
    },
    Neg {
        a: Expr,
    },
    Mul {
        a: Expr,
        b: Expr,
    },
    Div {
        a: Expr,
        b: Expr,
    },
    Pow {
        a: Expr,
        b: Expr,
    },
    Abs {
        a: Expr,
    },
}

pub enum OperandKind {
    Add,
    Min,
    Neg,
    Mul,
    Div,
    Pow,
    Abs,

}

impl Operand {
    fn eval(&self, x: f64) -> f64 {
        match self {
            Self::Add { a, b } => a.eval(x) + b.eval(x),
            Self::Min { a, b } => a.eval(x) - b.eval(x),
            Self::Neg { a } => -a.eval(x),
            Self::Mul { a, b } => a.eval(x) * b.eval(x),
            Self::Div { a, b } => a.eval(x) / b.eval(x),
            Self::Pow { a, b } => a.eval(x).powf(b.eval(x)),
            Self::Abs { a } => a.eval(x).abs(),
            _ => { unimplemented!("Not implemented yet") }
        }
    }
    
}

pub enum Expression {
    Operand(Operand),
    Number(f64),
    Var,
}

pub type Expr = Box<Expression>;

impl Expression {
    fn eval(&self, x: f64) -> f64 {
        match self {
            Expression::Operand(op) => op.eval(x),
            Expression::Number(n) => *n,
            Expression::Var => x,
        }
    }
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    pub fn next(&mut self) -> Option<char> {
        let next = self.peek();
        if next.is_some() {
            self.position += 1;
        }
        next
    }

    pub fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    pub fn consume_number(&mut self) -> f64 {
        let mut number = String::new();
        while let Some(c) = self.peek() {
            if c.is_digit(10) || c == '.' {
                number.push(c);
                self.next();
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }

    pub fn consume_operand(&mut self) -> OperandKind {
        self.consume_whitespace();
        let c = self.next().unwrap();
        match c {
            '+' => OperandKind::Add,
            '-' => OperandKind::Min,
            '*' => OperandKind::Mul,
            '/' => OperandKind::Div,
            '^' => OperandKind::Pow,
            _ => { unimplemented!("Not implemented yet") }
        }
    }
    

}

struct Function {
    expr: Expr,
}

impl Function {
    fn new(expr: Expr) -> Function {
        Function { expr }
    }

    fn from_string(s: &str) -> Function {
        //let expr = parse_expr(s);
        unimplemented!()
    }
}