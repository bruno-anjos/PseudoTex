macro_rules! BEGIN {
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

macro_rules! END {
    () => ("\\BlankLine\n\
    \\BlankLine\n\
    \\BlankLine\n\
    \\end{algorithm}\n\
    \\end{document}")
}

macro_rules! FUNCTION_CODE {
    () => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
    \\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
    \\Fn{{\\FunctionID{{}}}}{{\n\
        {}\n
    }}")
}

macro_rules! EVENT_CODE {
    () => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
    \\SetKwProg{{Fn}}{{Upon event }}{{ do:}}{{}}\n\
    \\Fn{{\\FunctionID{{}}}}{{\n\
        {}\n
    }}")
}

macro_rules! FUNCTION_WITH_ARGS_CODE {
    () => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
    \\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
    \\Fn{{\\FunctionID{{{}}}}}{{\n\
        {}\n
    }}")
}

macro_rules! EVENT_WITH_ARGS_CODE {
    () => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
    \\SetKwProg{{Fn}}{{Upon event }}{{ do:}}{{}}\n\
    \\Fn{{\\FunctionID{{{}}}}}{{\n\
        {}\n
    }}")
}

macro_rules! ASSIGN_CODE {
    () => ("{} $\\longleftarrow$ {}")
}

macro_rules! IN_CODE {
    () => ("$\\in$")
}

macro_rules! NOT_IN_CODE {
    () => ("$\\notin$")
}

macro_rules! SET_MINUS_CODE {
    () => ("$\\setminus$")
}

macro_rules! INTERSECT_CODE {
    () => ("$\\cap$")
}

macro_rules! UNION_CODE {
    () => ("$\\cup$")
}

macro_rules! EXISTS_CODE {
    () => ("$\\exists$")
}

macro_rules! NOT_EXISTS_CODE {
    () => ("$\\nexists$")
}

macro_rules! UNDEFINED_CODE {
    () => ("$\\bot$")
}

macro_rules! IF_CODE {
    () => ("\\If{{{}}}{{{}}}");
}

macro_rules! ELSE_IF_CODE {
    () => ("\\ElseIf{{{}}}{{{}}}");
}

macro_rules! ELSE_CODE {
    () => ("\\Else{{{}}}");
}

macro_rules! IF_COMPOSED_CODE {
    () => ("\\uIf{{{}}}{{{}}}");
}

macro_rules! ELSE_IF_COMPOSED_CODE {
    () => ("\\uElseIf{{{}}}{{{}}}");
}

macro_rules! STATE_CODE {
    () => ("\\SetKwIF{{State}}{{}}{{}}{{State}}{{:}}{{}}{{}}{{}}
    \\State{{}}{{{}}}\n");
}

macro_rules! REQUESTS_CODE {
    () => ("\\SetKwIF{{Requests}}{{}}{{}}{{Requests}}{{:}}{{}}{{}}{{}}
    \\Requests{{}}{{{}}}\n");
}

macro_rules! INDICATIONS_CODE {
    () => ("\\SetKwIF{{Indications}}{{}}{{}}{{Indications}}{{:}}{{}}{{}}{{}}
    \\Indications{{}}{{{}}}\n");
}

macro_rules! INTERFACE_CODE {
    () => ("\\SetKwIF{{Interface}}{{}}{{}}{{Interface}}{{:}}{{}}{{}}{{}}
    \\Interface{{}}{{\
    {}\\;\n\
    {}\n\
    }}\n");
}

macro_rules! METHOD_CALL_CODE {
    () => ("\\FuncSty{{{}(}}\\ArgSty{{{}}}\\FuncSty{{)}}");
}

macro_rules! TRIGGER_CODE {
    () => ("\\FuncSty{{Trigger}} {}");
}
