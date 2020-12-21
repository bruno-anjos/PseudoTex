macro_rules! begin {
	() => ("\\documentclass{article}\n\
	\\usepackage[utf8]{inputenc}\n\
	\\usepackage[ruled,vlined,linesnumbered]{algorithm2e}\n\
	\\usepackage{amssymb}\n\
	\\begin{document}\n\
	{{\\DontPrintSemicolon\n\
	\\SetAlgoNoLine\n\
	\\LinesNumberedHidden\n\
	\\SetFuncSty{textbf}\n\
	\\begin{algorithm}[ht]\n\
	\\BlankLine\n\
	\\BlankLine\n\
	\\BlankLine\n")
}

macro_rules! end {
	() => ("\\BlankLine\n\
	\\BlankLine\n\
	\\BlankLine\n\
	\\end{algorithm}\n\
	\\end{document}\n")
}

macro_rules! function_code {
	() => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
	\\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
	\\Fn{{\\FunctionID{{}}}}{{\n\
	{}\n\
	}}\n")
}

macro_rules! init_code {
	() => ("\\SetKwFunction{{FunctionID}}{{Init}}\n\
	\\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
	\\Fn{{\\FunctionID}}{{\n\
	{}\n\
	}}\n");
}

macro_rules! init_with_args_code {
	() => ("\\SetKwFunction{{FunctionID}}{{Init}}\n\
	\\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
	\\Fn{{\\FunctionID{{{}}}}}{{\n\
	{}\n\
	}}\n")
}

macro_rules! foreach_code {
	() => ("\\ForEach{{{}}}{{{}}}")
}

macro_rules! timer_code {
	() => ("\\SetKwFunction{{FunctionID}}{{Timer {}}}\n\
	\\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
	\\Fn{{\\FunctionID}}{{\n\
	{}\n\
	}}\n");
}

macro_rules! timer_with_args_code {
	() => ("\\SetKwFunction{{FunctionID}}{{Timer {}}}\n\
	\\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
	\\Fn{{\\FunctionID{{{}}}}}{{\n\
	{}\n\
	}}\n")
}

macro_rules! crash_with_args_code {
	() => ("\\SetKwFunction{{FunctionID}}{{Crash}}\n\
	\\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
	\\Fn{{\\FunctionID{{{}}}}}{{\n\
	{}\n\
	}}\n")
}

macro_rules! string_with_underscore_code {
	() => ("{}\\_{}")
}

macro_rules! procedure_code {
	() => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
	\\SetKwProg{{Fn}}{{Procedure }}{{:}}{{}}\n\
	\\Fn{{\\FunctionID}}{{\n\
	{}\n\
	}}\n")
}

macro_rules! procedure_with_args_code {
	() => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
	\\SetKwProg{{Fn}}{{Procedure }}{{:}}{{}}\n\
	\\Fn{{\\FunctionID{{{}}}}}{{\n\
	{}\n\
	}}\n")
}

macro_rules! event_code {
	() => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
	\\SetKwProg{{Fn}}{{Upon event }}{{ do:}}{{}}\n\
	\\Fn{{\\FunctionID{{}}}}{{\n\
	{}\n\
	}}\n")
}

macro_rules! function_with_args_code {
	() => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
	\\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
	\\Fn{{\\FunctionID{{{}}}}}{{\n\
	{}\n\
	}}\n")
}

macro_rules! event_with_args_code {
	() => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
	\\SetKwProg{{Fn}}{{Upon event }}{{ do:}}{{}}\n\
	\\Fn{{\\FunctionID{{{}}}}}{{\n\
	{}\n\
	}}\n")
}

macro_rules! assign_code {
	() => ("{} $\\longleftarrow$ {}")
}

macro_rules! in_code {
	() => ("$\\in$")
}

macro_rules! not_in_code {
	() => ("$\\notin$")
}

macro_rules! set_minus_code {
	() => ("$\\setminus$")
}

macro_rules! intersect_code {
	() => ("$\\cap$")
}

macro_rules! union_code {
	() => ("$\\cup$")
}

macro_rules! exists_code {
	() => ("$\\exists$")
}

macro_rules! not_exists_code {
	() => ("$\\nexists$")
}

macro_rules! undefined_code {
	() => ("$\\bot$")
}

macro_rules! if_code {
	() => ("\\If{{{}}}{{{}}}");
}

macro_rules! else_if_code {
	() => ("\\ElseIf{{{}}}{{{}}}");
}

macro_rules! else_code {
	() => ("\\Else{{{}}}");
}

macro_rules! if_composed_code {
	() => ("\\uIf{{{}}}{{{}}}");
}

macro_rules! else_if_composed_code {
	() => ("\\uElseIf{{{}}}{{{}}}");
}

macro_rules! state_code {
	() => ("\\SetKwFunction{{FunctionID}}{{State}}\n\
	\\SetKwProg{{Fn}}{{}}{{:}}{{}}\n\
	\\Fn{{\\FunctionID}}{{\n\
	{}\n\
	}}\n");
}

macro_rules! requests_code {
	() => ("\\SetKwFunction{{FunctionID}}{{Requests}}\n\
	\\SetKwProg{{Fn}}{{}}{{:}}{{}}\n\
	\\Fn{{\\FunctionID}}{{\n\
	{}\n\
	}}\n");
}

macro_rules! indications_code {
	() => ("\\SetKwFunction{{FunctionID}}{{Indications}}\n\
	\\SetKwProg{{Fn}}{{}}{{:}}{{}}\n\
	\\Fn{{\\FunctionID}}{{\n\
	{}\n\
	}}\n");
}

macro_rules! interface_code {
	() => ("\\SetKwFunction{{FunctionID}}{{Indications}}\n\
	\\SetKwProg{{Fn}}{{}}{{:}}{{}}\n\
	\\Fn{{\\FunctionID}}{{\n\
	{}\\;\n\
	{}\n\
	}}\n");
}

macro_rules! method_call_code {
	() => ("\\FuncSty{{{}(}}\\ArgSty{{}}\\FuncSty{{)}}");
}

macro_rules! method_call_with_args_code {
	() => ("\\FuncSty{{{}(}}\\ArgSty{{{}}}\\FuncSty{{)}}");
}

macro_rules! setup_timer_code {
	() => ("\\FuncSty{{Setup Timer {}(}}\\ArgSty{{{}}}\\FuncSty{{)}}");
}

macro_rules! cancel_timer_code {
	() => ("\\FuncSty{{Cancel Timer {}}}\\ArgSty{{}}\\FuncSty{{}}");
}

macro_rules! cancel_timer_with_args_code {
	() => ("\\FuncSty{{Cancel Timer {}(}}\\ArgSty{{{}}}\\FuncSty{{)}}");
}

macro_rules! setup_periodic_timer_code {
	() => ("\\FuncSty{{Setup Periodic Timer {}(}}\\ArgSty{{{}}}\\FuncSty{{)}}");
}

macro_rules! procedure_call_code {
	() => ("\\FuncSty{{Call {}(}}\\ArgSty{{}}\\FuncSty{{)}}");
}

macro_rules! procedure_call_with_args_code {
	() => ("\\FuncSty{{Call {}(}}\\ArgSty{{{}}}\\FuncSty{{)}}");
}

macro_rules! trigger_code {
	() => ("\\FuncSty{{Trigger}} {}");
}

macro_rules! open_bra_code {
	() => ("[");
}

macro_rules! close_bra_code {
	() => ("]");
}

macro_rules! open_curly_bra_code {
	() => ("\\{");
}

macro_rules! close_curly_bra_code {
	() => ("\\}");
}

macro_rules! dot_code {
	() => (".");
}

macro_rules! comp_eq_code {
	() => ("$=$");
}

macro_rules! return_code {
	() => ("return");
}

macro_rules! continue_code {
	() => ("continue");
}

macro_rules! break_code {
	() => ("break");
}

macro_rules! and_code {
	() => ("$\\land$");
}

macro_rules! comma_code {
	() => (",");
}

macro_rules! plus_code {
	() => ("+");
}

macro_rules! minus_code {
	() => ("-");
}

macro_rules! operation_code {
	() => ("{} {} {}");
}

macro_rules! access_code {
	() => ("{}[{}]");
}

macro_rules! set_code {
	() => ("\\{{{}\\}}");
}

macro_rules! empty_set_code {
	() => ("\\{{\\}}");
}

macro_rules! less_code {
	() => ("$<$");
}

macro_rules! less_eq_code {
	() => ("$\\leq$");
}

macro_rules! greater_code {
	() => ("$>$");
}

macro_rules! greater_eq_code {
	() => ("$\\geq$");
}

macro_rules! different_code {
	() => ("$\\neq$");
}

macro_rules! not_code {
	() => ("$\\neg$ ({})");
}

macro_rules! division_code {
	() => ("/");
}

macro_rules! times_code {
	() => ("*");
}

macro_rules! cardinality_code {
	() => ("\\#{}");
}