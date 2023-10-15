use std::collections::HashMap;

struct Context<'a> {
    data: HashMap<&'a VariableExpr, bool>,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn lookup(&self, key: &VariableExpr) -> bool {
        *self.data.get(key).expect("key not found")
    }

    fn assign(&mut self, expr: &'a VariableExpr, value: bool) {
        self.data.insert(expr, value);
    }
}

trait BooleanExpr {
    fn evaluate(&self, context: &Context) -> bool;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct VariableExpr {
    name: String,
}

impl VariableExpr {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

impl BooleanExpr for VariableExpr {
    fn evaluate(&self, context: &Context) -> bool {
        context.lookup(self)
    }
}

struct AndExpr<'a, T: BooleanExpr, U: BooleanExpr> {
    operand1: &'a T,
    operand2: &'a U,
}

impl<'a, T, U> AndExpr<'a, T, U>
where
    T: BooleanExpr,
    U: BooleanExpr,
{
    fn new(operand1: &'a T, operand2: &'a U) -> Self {
        Self { operand1, operand2 }
    }
}

impl<'a, T, U> BooleanExpr for AndExpr<'a, T, U>
where
    T: BooleanExpr,
    U: BooleanExpr,
{
    fn evaluate(&self, context: &Context) -> bool {
        self.operand1.evaluate(context) && self.operand2.evaluate(context)
    }
}

struct OrExpr<'a, T: BooleanExpr, U: BooleanExpr>
where
    T: BooleanExpr,
    U: BooleanExpr,
{
    operand1: &'a T,
    operand2: &'a U,
}

impl<'a, T, U: BooleanExpr> OrExpr<'a, T, U>
where
    T: BooleanExpr,
    U: BooleanExpr,
{
    fn new(operand1: &'a T, operand2: &'a U) -> Self {
        Self { operand1, operand2 }
    }
}

impl<'a, T, U> BooleanExpr for OrExpr<'a, T, U>
where
    T: BooleanExpr,
    U: BooleanExpr,
{
    fn evaluate(&self, context: &Context) -> bool {
        self.operand1.evaluate(context) || self.operand2.evaluate(context)
    }
}

struct NotExpr<'a, T: BooleanExpr> {
    operand: &'a T,
}

impl<'a, T: BooleanExpr> NotExpr<'a, T> {
    fn new(operand: &'a T) -> Self {
        Self { operand }
    }
}

impl<'a, T: BooleanExpr> BooleanExpr for NotExpr<'a, T> {
    fn evaluate(&self, context: &Context) -> bool {
        !self.operand.evaluate(context)
    }
}

struct Constant {
    operand: bool,
}

impl Constant {
    fn new(operand: bool) -> Self {
        Self { operand }
    }
}

impl BooleanExpr for Constant {
    fn evaluate(&self, _context: &Context) -> bool {
        self.operand
    }
}

fn main() {
    let expr_x = VariableExpr::new("X");
    let expr_y = VariableExpr::new("Y");

    let and_expression = AndExpr::new(&expr_x, &expr_y);
    let mut context = Context::new();
    let _constant = Constant::new(true);
    let _not_expr = NotExpr::new(&expr_x);

    context.assign(&expr_x, true);
    context.assign(&expr_y, false);

    let result = and_expression.evaluate(&context);
    assert!(!result);

    context.assign(&expr_x, true);
    context.assign(&expr_y, true);

    let result = and_expression.evaluate(&context);
    assert!(result);

    let or_expression = OrExpr::new(&expr_x, &expr_y);
    let result = or_expression.evaluate(&context);
    assert!(result);
}

#[test]
fn test_complex_expr() {
    // A more complex example reflecting different combinations of expressions
    // (true and x) or (y and (not x))

    let expr_x = VariableExpr::new("X");
    let expr_y = VariableExpr::new("Y");

    let mut context = Context::new();

    let constant = Constant::new(true);
    let not_expr = NotExpr::new(&expr_x);

    let operand1 = AndExpr::new(&constant, &expr_x);
    let operand2 = AndExpr::new(&expr_y, &not_expr);
    let expression = OrExpr::new(&operand1, &operand2);

    context.assign(&expr_x, false);
    context.assign(&expr_y, true);

    assert!(expression.evaluate(&context));
}
