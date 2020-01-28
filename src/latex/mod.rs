macro_rules! BEGIN {
    () => ("\\documentclass{article}\n\
    \\usepackage[utf8]{inputenc}\n\
    \\usepackage[ruled,vlined,linesnumbered]{algorithm2e}\n\
    \\begin{document}\n\
    \\begin{algorithm}[H]\n\
    \\DontPrintSemicolon\n\
    \\SetAlgoLined\n")
}

macro_rules! END {
    () => ("\\end{algorithm}\n\
    \\end{document}")
}

macro_rules! FUNCTION_CODE {
    () => ("\\SetKwFunction{{FunctionID}}{{{}}}\n\
    \\SetKwProg{{Fn}}{{Upon }}{{ do:}}{{}}\n\
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

macro_rules! ASSIGN_CODE {
    () => ("{} $\\longleftarrow$ {}")
}

macro_rules! IN_CODE {
    () => ("$\\in$")
}

macro_rules! EXISTS_CODE {
    () => ("$\\exists$")
}

macro_rules! IF_CODE {
    () => ("\\If{{{}}}{{{}}}");
}

macro_rules! IF_ELSE_CODE {
    () => ("\\eIf{{{}}}{{{}}}{{{}}}");
}