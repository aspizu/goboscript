from typing import TypeAlias

blockdef_T: TypeAlias = dict[str, tuple[str, tuple[str, ...]]]

reporter: blockdef_T = {
    # fmt: off
    "+"    : ( "operator_add"      , ( "NUM1" , "NUM2" ) ) ,
    "-"    : ( "operator_subtract" , ( "NUM1" , "NUM2" ) ) ,
    "*"    : ( "operator_multiply" , ( "NUM1" , "NUM2" ) ) ,
    "/"    : ( "operator_divide"   , ( "NUM1" , "NUM2" ) ) ,
    "eq"   : ( "operator_equals"   , ( "NUM1" , "NUM2" ) ) ,
    "gt"   : ( "operator_gt"       , ( "NUM1" , "NUM2" ) ) ,
    "lt"   : ( "operator_lt"       , ( "NUM1" , "NUM2" ) ) ,
    "and"  : ( "operator_and"      , ( "NUM1" , "NUM2" ) ) ,
    "or"   : ( "operator_or"       , ( "NUM1" , "NUM2" ) ) ,
    "not"  : ( "operator_not"      , ( "OPERAND" , ) ) ,
    "join" : ( "operator_join"     , ( "STRING1" , "STRING2" ) ) ,
    # fmt: on
}

statement: blockdef_T = {
    # fmt: off
    "say"  : ( "looks_say"        , ( "MESSAGE" , ) ) ,
    "move" : ( "motion_movesteps" , ( "STEPS" , ) ) ,
    # fmt: on
}
