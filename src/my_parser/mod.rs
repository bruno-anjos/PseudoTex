#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    MultipleExpr {
        e1: Box<Expr>,
        e2: Box<Expr>,
    },
    MultipleExprNoNewLine {
        e1: Box<Expr>,
        e2: Box<Expr>,
    },
    SingleLineMultipleTerm {
        t1: Box<Expr>,
        t2: Box<Expr>,
	},
	SingleLineMultipleTermNoSpace {
        t1: Box<Expr>,
        t2: Box<Expr>,
    },
    MultiLineMultipleTerm {
        t1: Box<Expr>,
        t2: Box<Expr>,
    },
    Assign {
        name: Box<Expr>,
        value: Box<Expr>,
    },
    Method {
        name: String,
        body: Box<Expr>,
    },
    MethodWithArgs {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
    },
    Event {
        name: String,
        body: Box<Expr>,
    },
    EventWithArgs {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
    },
    Init {
        body: Box<Expr>,
	},
	InitWithArgs {
		args: Vec<String>,
        body: Box<Expr>,
	},
	Timer {
		name: String,
        body: Box<Expr>,
	},
	TimerWithArgs {
		name: String,
		args: Vec<String>,
        body: Box<Expr>,
	},
	CrashWithArgs{
		args: Vec<String>,
		body: Box<Expr>,
	},
	Procedure{
		name: String,
		body: Box<Expr>,
	},
	ProcedureWithArgs{
		name: String,
		args: Vec<String>,
		body: Box<Expr>,
	},
    If {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    ElseIf {
        condition: Box<Expr>,
        else_if_body: Box<Expr>,
    },
    Else {
        else_body: Box<Expr>,
    },
    IfComposed {
        condition: Box<Expr>,
        if_body: Box<Expr>,
    },
    ElseIfComposed {
        condition: Box<Expr>,
        else_if_body: Box<Expr>,
	},
	SetupTimer{
		name: String,
		args: Vec<String>,
	},
	SetupPeriodicTimer{
		name: String,
		args: Vec<String>,
	},
	CancelTimer{
		name: String,
	},
	CancelTimerWithArgs{
		name: String,
		args: Vec<String>,
	},
	ForEach{
		condition: Box<Expr>,
		body: Box<Expr>,
	},
    String(String),
    Special(Special),
    State {
        body: Box<Expr>,
    },
    Interface {
        reqs: Box<Expr>,
        indics: Box<Expr>,
    },
    MethodCall {
        name: String,
	},
	MethodCallWithArgs {
        name: String,
        args: Box<Expr>,
	},
	ProcedureCall {
        name: String,
	},
	ProcedureCallWithArgs {
        name: String,
        args: Box<Expr>,
    },
    Requests {
        requests: Box<Expr>,
    },
    Indications {
        indications: Box<Expr>,
    },
    Trigger {
        method: Box<Expr>,
    },
    Return{

    },
    Continue{

    },
    Break{

    },
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
	OpenBra,
	CloseBra,
	OpenCurlyBra,
	CloseCurlyBra,
	Dot,
	CompEq,
	Comma,
	And,
	Plus,
}

pub trait Translate {
    fn eval_translate(self) -> String;
}

impl Translate for Expr {
    fn eval_translate(self) -> String {
        match self {
            Expr::Assign { name, value } => format!(ASSIGN_CODE!(), name.eval_translate(), value.eval_translate()),
            Expr::MultipleExpr { e1, e2 } => {
                format!("{}\n\\;{}\n", e1.eval_translate(), e2.eval_translate())
            }
            Expr::MultipleExprNoNewLine { e1, e2 } => {
                format!("{}\n{}\n", e1.eval_translate(), e2.eval_translate())
            }
            Expr::MultiLineMultipleTerm { t1, t2 } => {
                format!("{}\\;\n{}", t1.eval_translate(), t2.eval_translate())
            }
            Expr::SingleLineMultipleTerm { t1, t2 } => {
                format!("{} {}", t1.eval_translate(), t2.eval_translate())
			}
			Expr::SingleLineMultipleTermNoSpace { t1, t2 } => {
                format!("{}{}", t1.eval_translate(), t2.eval_translate())
            }
            Expr::Method { name, body } => format!(FUNCTION_CODE!(), name, body.eval_translate()),
            Expr::MethodWithArgs { name, args, body } => format!(
                FUNCTION_WITH_ARGS_CODE!(),
                name,
                args.join(", "),
                body.eval_translate()
            ),
            Expr::Event { name, body } => format!(EVENT_CODE!(), name, body.eval_translate()),
            Expr::EventWithArgs { name, args, body } => format!(
                EVENT_WITH_ARGS_CODE!(),
                name,
                args.join(", "),
                body.eval_translate()
            ),
			Expr::MethodCall { name} => format!(METHOD_CALL_CODE!(), name),
			Expr::MethodCallWithArgs { name, args } => format!(METHOD_CALL_WITH_ARGS_CODE!(), name, args.eval_translate()),
			Expr::ProcedureCall { name } => format!(PROCEDURE_CALL_CODE!(), name),
			Expr::ProcedureCallWithArgs { name, args } => format!(PROCEDURE_CALL_WITH_ARGS_CODE!(), name, args.eval_translate()),
            Expr::String(s) => s,
            Expr::Special(s) => s.eval_translate(),
            Expr::If { condition, body } => format!(
                IF_CODE!(),
                condition.eval_translate(),
                body.eval_translate()
            ),
            Expr::ElseIf {
                condition,
                else_if_body,
            } => format!(
                ELSE_IF_CODE!(),
                condition.eval_translate(),
                else_if_body.eval_translate()
            ),
            Expr::Else { else_body } => format!(ELSE_CODE!(), else_body.eval_translate()),
            Expr::IfComposed { condition, if_body } => format!(
                IF_COMPOSED_CODE!(),
                condition.eval_translate(),
                if_body.eval_translate()
            ),
            Expr::ElseIfComposed {
                condition,
                else_if_body,
            } => format!(
                ELSE_IF_COMPOSED_CODE!(),
                condition.eval_translate(),
                else_if_body.eval_translate()
			),
			Expr::Init{body} => format!(
				INIT_CODE!(),
				body.eval_translate()
			),
			Expr::InitWithArgs{args, body} => format!(
				INIT_WITH_ARGS_CODE!(),
				args.join(", "),
				body.eval_translate()
			),
			Expr::Timer{name, body} => format!(
				TIMER_CODE!(),
				name,
				body.eval_translate()
			),
			Expr::TimerWithArgs{name, args, body} => format!(
				TIMER_WITH_ARGS_CODE!(),
				name,
				args.join(", "),
				body.eval_translate()
			),
			Expr::CrashWithArgs{args, body} => format!(
				CRASH_WITH_ARGS_CODE!(),
				args.join(", "),
				body.eval_translate()
			),
			Expr::Procedure{name, body} => format!(
				PROCEDURE_CODE!(),
				name,
				body.eval_translate()
			),
			Expr::ProcedureWithArgs{name, args, body} => format!(
				PROCEDURE_WITH_ARGS_CODE!(),
				name,
				args.join(", "),
				body.eval_translate()
			),
			Expr::SetupTimer{name, args} => format!(
				SETUP_TIMER_CODE!(),
				name,
				args.join(", "),
			),
			Expr::SetupPeriodicTimer{name, args} => format!(
				SETUP_PERIODIC_TIMER_CODE!(),
				name,
				args.join(", "),
			),
			Expr::CancelTimer{name} => format!(
				CANCEL_TIMER_CODE!(),
				name,
			),
			Expr::CancelTimerWithArgs{name, args} => format!(
				CANCEL_TIMER_WITH_ARGS_CODE!(),
				name,
				args.join(", "),
			),
			Expr::ForEach{condition, body} => format!(
				FOREACH_CODE!(),
				condition.eval_translate(),
				body.eval_translate(),
			),
            Expr::State { body } => format!(STATE_CODE!(), body.eval_translate()),
            Expr::Interface { reqs, indics } => format!(
                INTERFACE_CODE!(),
                reqs.eval_translate(),
                indics.eval_translate()
            ),
            Expr::Requests { requests } => format!(REQUESTS_CODE!(), requests.eval_translate()),
            Expr::Indications { indications } => {
                format!(INDICATIONS_CODE!(), indications.eval_translate())
            }
            Expr::Trigger { method } => format!(TRIGGER_CODE!(), method.eval_translate()),
            Expr::Return{} => format!(RETURN_CODE!()),
            Expr::Continue{} => format!(CONTINUE_CODE!()),
            Expr::Break{} => format!(BREAK_CODE!()),
            Expr::Empty => String::new(),
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
            Special::OpenBra => OPEN_BRA_CODE!(),
            Special::CloseBra => CLOSE_BRA_CODE!(),
            Special::OpenCurlyBra => OPEN_CURLY_BRA_CODE!(),
            Special::CloseCurlyBra => CLOSE_CURLY_BRA_CODE!(),
            Special::Dot => DOT_CODE!(),
            Special::CompEq=> COMP_EQ_CODE!(),
            Special::And => AND_CODE!(),
            Special::Comma => COMMA_CODE!(),
            Special::Plus => PLUS_CODE!(),
        }
        .to_string()
    }
}
