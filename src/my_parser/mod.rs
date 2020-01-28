#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    MultipleExpr { e1: Box<Expr>, e2: Box<Expr> },
    Assign { name: String, value: String },
    Method { name: String, body: Box<Expr> },
    MethodWithArgs { name: String, args: Vec<String>, body: Box<Expr> },
    If { condition: Box<Expr>, body: Box<Expr> },
    IfElse { condition: Box<Expr>, ifBody: Box<Expr>, elseBody: Box<Expr> },
    Condition { left: Box<Expr>, special: Special, right: Box<Expr> },
    ConditionRightSide { special: Special, right: Box<Expr> },
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Special {
    In,
    Exists,
}

pub trait Translate {
    fn eval_translate(self) -> String;
}

impl Translate for Expr {
    fn eval_translate(self) -> String {
        match self {
            Expr::Assign { name, value } =>
                format!(ASSIGN_CODE!(), name, value),
            Expr::MultipleExpr { e1, e2 } =>
                format!(
                    "{}\\;\n{}\n",
                    e1.eval_translate(),
                    e2.eval_translate()
                ),
            Expr::Method { name, body } =>
                format!(
                    FUNCTION_CODE!(),
                    name,
                    body.eval_translate()
                ),
            Expr::MethodWithArgs { name, args, body } =>
                format!(
                    FUNCTION_WITH_ARGS_CODE!(),
                    name,
                    args.join(", "),
                    body.eval_translate()
                ),
            Expr::String(s) => s,
            Expr::Condition { left, special, right } =>
                format!(
                    "{} {} {}",
                    left.eval_translate(),
                    special.eval_translate(),
                    right.eval_translate()
                ),
            Expr::ConditionRightSide { special, right } =>
                format!(
                    "{} {}",
                    special.eval_translate(),
                    right.eval_translate()
                ),
            Expr::If { condition, body} =>
                format!(
                    IF_CODE!(),
                    condition.eval_translate(),
                    body.eval_translate()
                ),
            Expr::IfElse { condition, ifBody, elseBody} =>
                format!(
                    IF_ELSE_CODE!(),
                    condition.eval_translate(),
                    ifBody.eval_translate(),
                    elseBody.eval_translate()
                )
        }
    }
}

impl Translate for Special {
    fn eval_translate(self) -> String {
        match self {
            Special::Exists => EXISTS_CODE!(),
            Special::In => IN_CODE!(),
        }.to_string()
    }
}
