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
	CrashWithArgs {
		args: Vec<String>,
		body: Box<Expr>,
	},
	Procedure {
		name: String,
		body: Box<Expr>,
	},
	ProcedureWithArgs {
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
	SetupTimer {
		name: String,
		args: Box<Expr>,
	},
	SetupPeriodicTimer {
		name: String,
		args: Box<Expr>,
	},
	CancelTimer {
		name: String,
	},
	CancelTimerWithArgs {
		name: String,
		args: Box<Expr>,
	},
	ForEach {
		condition: Box<Expr>,
		body: Box<Expr>,
	},
	String(String),
	StringWithUnderscore {
		s: String,
		t: Box<Expr>,
	},
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
	Return {},
	Continue {},
	Break {},
	Operation {
		e1: Box<Expr>,
		op: Special,
		e2: Box<Expr>,
	},
	Access {
		l: Box<Expr>,
		r: Box<Expr>,
	},
	Set {
		e: Box<Expr>
	},
	EmptySet {},
	CallingArgs {
		e1: Box<Expr>,
		e2: Box<Expr>,
	},
	Empty,
	Comment {
		string: String
	},
	Undefined {},
	Cardinality {
		e: Box<Expr>,
	},
	Negate {
		e: Box<Expr>,
	},
	LineWithComment {
		e: Box<Expr>,
		c: Box<Expr>
	},
	Negative {
		e: Box<Expr>
	}
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
	Minus,
	LessThan,
	LessEqThan,
	GreaterThan,
	GreaterEqThan,
	Division,
	Times,
	Diff,
}

pub trait Translate {
	fn eval_translate(self) -> String;
}

impl Translate for Expr {
	fn eval_translate(self) -> String {
		match self {
			Expr::Assign { name, value } => format!(assign_code!(), name.eval_translate(), value.eval_translate()),
			Expr::MultipleExpr { e1, e2 } => {
				format!("{}\n\\;{}", e1.eval_translate(), e2.eval_translate())
			}
			Expr::MultipleExprNoNewLine { e1, e2 } => {
				format!("{}\n{}", e1.eval_translate(), e2.eval_translate())
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
			Expr::Method { name, body } => format!(function_code!(), name, body.eval_translate()),
			Expr::MethodWithArgs { name, args, body } => format!(
				function_with_args_code!(),
				name,
				args.join(", "),
				body.eval_translate()
			),
			Expr::Event { name, body } => format!(event_code!(), name, body.eval_translate()),
			Expr::EventWithArgs { name, args, body } => format!(
				event_with_args_code!(),
				name,
				args.join(", "),
				body.eval_translate()
			),
			Expr::MethodCall { name } => format!(method_call_code!(), name),
			Expr::MethodCallWithArgs { name, args } => format!(method_call_with_args_code!(), name, args.eval_translate()),
			Expr::ProcedureCall { name } => format!(procedure_call_code!(), name),
			Expr::ProcedureCallWithArgs { name, args } => format!(procedure_call_with_args_code!(), name, args.eval_translate()),
			Expr::String(s) => s,
			Expr::StringWithUnderscore { s, t } => format!(string_with_underscore_code!(), s, t.eval_translate()),
			Expr::Special(s) => s.eval_translate(),
			Expr::If { condition, body } => format!(
				if_code!(),
				condition.eval_translate(),
				body.eval_translate()
			),
			Expr::ElseIf {
				condition,
				else_if_body,
			} => format!(
				else_if_code!(),
				condition.eval_translate(),
				else_if_body.eval_translate()
			),
			Expr::Else { else_body } => format!(else_code!(), else_body.eval_translate()),
			Expr::IfComposed { condition, if_body } => format!(
				if_composed_code!(),
				condition.eval_translate(),
				if_body.eval_translate()
			),
			Expr::ElseIfComposed {
				condition,
				else_if_body,
			} => format!(
				else_if_composed_code!(),
				condition.eval_translate(),
				else_if_body.eval_translate()
			),
			Expr::Init { body } => format!(
				init_code!(),
				body.eval_translate()
			),
			Expr::InitWithArgs { args, body } => format!(
				init_with_args_code!(),
				args.join(", "),
				body.eval_translate()
			),
			Expr::Timer { name, body } => format!(
				timer_code!(),
				name,
				body.eval_translate()
			),
			Expr::TimerWithArgs { name, args, body } => format!(
				timer_with_args_code!(),
				name,
				args.join(", "),
				body.eval_translate()
			),
			Expr::CrashWithArgs { args, body } => format!(
				crash_with_args_code!(),
				args.join(", "),
				body.eval_translate()
			),
			Expr::Procedure { name, body } => format!(
				procedure_code!(),
				name,
				body.eval_translate()
			),
			Expr::ProcedureWithArgs { name, args, body } => format!(
				procedure_with_args_code!(),
				name,
				args.join(", "),
				body.eval_translate()
			),
			Expr::SetupTimer { name, args } => format!(
				setup_timer_code!(),
				name,
				args.eval_translate(),
			),
			Expr::SetupPeriodicTimer { name, args } => format!(
				setup_periodic_timer_code!(),
				name,
				args.eval_translate(),
			),
			Expr::CancelTimer { name } => format!(
				cancel_timer_code!(),
				name,
			),
			Expr::CancelTimerWithArgs { name, args } => format!(
				cancel_timer_with_args_code!(),
				name,
				args.eval_translate(),
			),
			Expr::ForEach { condition, body } => format!(
				foreach_code!(),
				condition.eval_translate(),
				body.eval_translate(),
			),
			Expr::State { body } => format!(state_code!(), body.eval_translate()),
			Expr::Interface { reqs, indics } => format!(
				interface_code!(),
				reqs.eval_translate(),
				indics.eval_translate()
			),
			Expr::Requests { requests } => format!(requests_code!(), requests.eval_translate()),
			Expr::Indications { indications } => {
				format!(indications_code!(), indications.eval_translate())
			}
			Expr::Trigger { method } => format!(trigger_code!(), method.eval_translate()),
			Expr::Return {} => format!(return_code!()),
			Expr::Continue {} => format!(continue_code!()),
			Expr::Break {} => format!(break_code!()),
			Expr::Empty => String::new(),
			Expr::Operation { e1, op, e2 } => format!(operation_code!(),
													  e1.eval_translate(), op.eval_translate(), e2.eval_translate()),
			Expr::Access { l, r } => format!(access_code!(), l.eval_translate(), r.eval_translate
			()),
			Expr::Set { e } => format!(set_code!(), e.eval_translate()),
			Expr::CallingArgs { e1, e2 } => format!("{}, {}", e1.eval_translate(), e2
				.eval_translate()),
			Expr::EmptySet {} => format!(empty_set_code!()),
			Expr::Comment { string } => format!("// {}", string),
			Expr::Undefined {} => format!(undefined_code!()),
			Expr::Cardinality { e } => format!(cardinality_code!(), e.eval_translate()),
			Expr::Negate { e } => format!(not_code!(), e.eval_translate()),
			Expr::LineWithComment{e, c} => format!("{} {}", e.eval_translate(), c
				.eval_translate()),
			Expr::Negative {e} => format!("-{}", e.eval_translate())
		}
	}
}

impl Translate for Special {
	fn eval_translate(self) -> String {
		match self {
			Special::Exists => exists_code!(),
			Special::In => in_code!(),
			Special::NotExists => not_exists_code!(),
			Special::NotIn => not_in_code!(),
			Special::SetMinus => set_minus_code!(),
			Special::Union => union_code!(),
			Special::Intersect => intersect_code!(),
			Special::Undefined => undefined_code!(),
			Special::OpenBra => open_bra_code!(),
			Special::CloseBra => close_bra_code!(),
			Special::OpenCurlyBra => open_curly_bra_code!(),
			Special::CloseCurlyBra => close_curly_bra_code!(),
			Special::Dot => dot_code!(),
			Special::CompEq => comp_eq_code!(),
			Special::And => and_code!(),
			Special::Comma => comma_code!(),
			Special::Plus => plus_code!(),
			Special::Minus => minus_code!(),
			Special::LessThan => less_code!(),
			Special::LessEqThan => less_eq_code!(),
			Special::GreaterThan => greater_code!(),
			Special::GreaterEqThan => greater_eq_code!(),
			Special::Division => division_code!(),
			Special::Times => times_code!(),
			Special::Diff => different_code!()
		}
			.to_string()
	}
}
