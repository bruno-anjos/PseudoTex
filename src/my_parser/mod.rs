#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    MultipleExpr { e1: Box<Expr>, e2: Box<Expr> },
    SingleLineMultipleTerm { t1: Box<Expr>, t2: Box<Expr> },
    MultiLineMultipleTerm { t1: Box<Expr>, t2: Box<Expr> },
    Assign { name: String, value: Box<Expr> },
    Method { name: String, body: Box<Expr> },
    MethodWithArgs { name: String, args: Vec<String>, body: Box<Expr> },
    Event { name: String, body: Box<Expr> },
    EventWithArgs { name: String, args: Vec<String>, body: Box<Expr> },
    If { condition: Box<Expr>, body: Box<Expr> },
    IfElse { condition: Box<Expr>, if_body: Box<Expr>, else_body: Box<Expr> },
    String(String),
    Special(Special),
    State { body: Box<Expr> },
    Interface { reqs: Box<Expr>, indics: Box<Expr> },
    MethodCall { name: String, args: Vec<String> },
    Requests { requests: Box<Expr> },
    Indications { indications: Box<Expr> },
    Trigger { method: Box<Expr> },
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Special {
    In,
    Exists,
    NotExists,
    NotIn,
    SetMinus,
    Union,
    Intersect,
    Undefined,
}

pub trait Translate {
    fn eval_translate(self) -> String;
}

impl Translate for Expr {
    fn eval_translate(self) -> String {
        match self {
            Expr::Assign { name, value } =>
                format!(ASSIGN_CODE!(), name, value.eval_translate()),
            Expr::MultipleExpr { e1, e2 } =>
                format!(
                    "{}\n\\;{}\n",
                    e1.eval_translate(),
                    e2.eval_translate()
                ),
            Expr::MultiLineMultipleTerm { t1, t2 } =>
                format!(
                    "{}\\;\n{}",
                    t1.eval_translate(),
                    t2.eval_translate()
                ),
            Expr::SingleLineMultipleTerm { t1, t2 } =>
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
            Expr::MethodCall { name, args } =>
                format!(
                    METHOD_CALL_CODE!(),
                    name,
                    args.join(", ")
                ),
            Expr::String(s) => s,
            Expr::Special(s) => s.eval_translate(),
            Expr::If { condition, body } =>
                format!(
                    IF_CODE!(),
                    condition.eval_translate(),
                    body.eval_translate()
                ),
            Expr::IfElse { condition, if_body, else_body } =>
                format!(
                    IF_ELSE_CODE!(),
                    condition.eval_translate(),
                    if_body.eval_translate(),
                    else_body.eval_translate()
                ),
            Expr::State { body } =>
                format!(
                    STATE_CODE!(),
                    body.eval_translate()
                ),
            Expr::Interface { reqs, indics } =>
                format!(
                    INTERFACE_CODE!(),
                    reqs.eval_translate(),
                    indics.eval_translate()
                ),
            Expr::Requests { requests } =>
                format!(
                    REQUESTS_CODE!(),
                    requests.eval_translate()
                ),
            Expr::Indications { indications } =>
                format!(
                    INDICATIONS_CODE!(),
                    indications.eval_translate()
                ),
            Expr::Trigger { method } =>
                format!(
                    TRIGGER_CODE!(),
                    method.eval_translate()
                ),
            Expr::Empty => String::new()
        }
    }
}

impl Translate for Special {
    fn eval_translate(self) -> String {
        match self {
            Special::Exists => EXISTS_CODE!(),
            Special::In => IN_CODE!(),
            Special::NotExists => NOT_EXISTS_CODE!(),
            Special::NotIn => NOT_IN_CODE!(),
            Special::SetMinus => SET_MINUS_CODE!(),
            Special::Union => UNION_CODE!(),
            Special::Intersect => INTERSECT_CODE!(),
            Special::Undefined => UNDEFINED_CODE!(),
        }.to_string()
    }
}
