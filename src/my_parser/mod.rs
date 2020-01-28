#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    MultipleExpr { e1: Box<Expr>, e2: Box<Expr> },
    MultipleTerm { t1: Box<Expr>, t2: Box<Expr> },
    Assign { name: String, value: String },
    Method { name: String, body: Box<Expr> },
    MethodWithArgs { name: String, args: Vec<String>, body: Box<Expr> },
    Event { name: String, body: Box<Expr> },
    EventWithArgs { name: String, args: Vec<String>, body: Box<Expr> },
    If { condition: Box<Expr>, body: Box<Expr> },
    IfElse { condition: Box<Expr>, if_body: Box<Expr>, else_body: Box<Expr> },
    String(String),
    Special(Special),
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
                    "{}\n\\;{}\n",
                    e1.eval_translate(),
                    e2.eval_translate()
                ),
            Expr::MultipleTerm { t1, t2 } =>
                format!(
                    "{} {}",
                    t1.eval_translate(),
                    t2.eval_translate()
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
            Expr::Event { name, body } =>
                format!(
                    EVENT_CODE!(),
                    name,
                    body.eval_translate()
                ),
            Expr::EventWithArgs { name, args, body } =>
                format!(
                    EVENT_WITH_ARGS_CODE!(),
                    name,
                    args.join(", "),
                    body.eval_translate()
                ),
            Expr::String(s) => s,
            Expr::Special(s) => s.eval_translate(),
            Expr::If { condition, body} =>
                format!(
                    IF_CODE!(),
                    condition.eval_translate(),
                    body.eval_translate()
                ),
            Expr::IfElse { condition, if_body, else_body} =>
                format!(
                    IF_ELSE_CODE!(),
                    condition.eval_translate(),
                    if_body.eval_translate(),
                    else_body.eval_translate()
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
