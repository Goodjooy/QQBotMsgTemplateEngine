# 表达式功能

# 表达式支持什么
 * 整数加减法
 * 获取变量值
 * 字面字符串

expr -> caculate
    | literal
    | var

caculate -> item subc

subc -> + item subc
        | - item sub
        | Nil

item -> factor subitem

subitem -> * factor subitem
        | / factor subitem
        | Nil

factor -> digit
        | ( expr )
        | var

literal -> 'chars'

var -> \[变量符号表]