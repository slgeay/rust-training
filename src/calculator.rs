#[derive(Debug)]
enum Expr {
    Number(i64),
    Sqr(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>)
}

#[derive(Debug)]
enum EvalError {

}

fn eval(expr: &Expr) -> Result<i64, EvalError> {
    match expr {
        Expr::Number(i) => Ok(*i),
        Expr::Sqr(e) => Ok(i64::pow(eval(e)?, 2)),
        Expr::Add(a, b) => Ok(eval(a)? + eval(b)?),
        Expr::Sub(a, b) => Ok(eval(a)? - eval(b)?),
        Expr::Mul(a, b) => Ok(eval(a)? * eval(b)?),
        Expr::Div(a, b) => Ok(eval(a)? / eval(b)?),
    }
}

#[derive(Debug)]
enum ParseError {
    BadNumber,
    NotEnoughOperands
}

fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();
    for word in input.split_ascii_whitespace() {
        match word {
            "sqr" => {
                assert_ne!(stack.len(), 0);
                let e = stack.pop().ok_or(ParseError::NotEnoughOperands)?;
                stack.push(Expr::Sqr(Box::new(e)));
            },
            "+"|"-"|"*"|"/" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(
                        match word {
                            "+" => Expr::Add,
                            "-" => Expr::Sub,
                            "*" => Expr::Mul,
                            _ => Expr::Div
                        }
                        (Box::new(a),Box::new(b))
                    );
                } else {
                    return Err(ParseError::NotEnoughOperands);
                }
            },
            _ => {
                let num: i64 = word.parse().map_err(|_| ParseError::BadNumber)?;
                stack.push(Expr::Number(num));
            }
        }
        println!("{:?}", stack);
    };
    assert_eq!(stack.len(), 1);
    let res = stack.pop().unwrap();
    Ok(res)
}


#[test]
fn smoke_test() {
    let input = "3 sqr 4 sqr + 5 sqr 2 * - 5 /";

    let expr = parse(input).unwrap();
    let value = eval(&expr).unwrap();
    assert_eq!(value, -5);
}


