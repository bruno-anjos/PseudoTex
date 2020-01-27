#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    MultipleExpr { e1: Box<Expr>, e2: Box<Expr> },
    Assign { name: String, value: String },
    Method { name: String, body: Box<Expr> },
    MethodWithArgs { name: String, args: Vec<String>, body: Box<Expr> },
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
            Expr::Method { name, body } => {
                format!(
                    FUNCTION_CODE!(),
                    name,
                    body.eval_translate()
                )
            }
            Expr::MethodWithArgs { name, args, body } =>
                format!(
                    FUNCTION_WITH_ARGS_CODE!(),
                    name,
                    args.join(", "),
                    body.eval_translate()
                )
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operators {
    Equals
}
