# 文法

```
tag -> text | img | at | sign | endl | if | loop
literal -> chars

expr -> tag | literal
exprs -> expr exprs

text -> <text (line="(true|false)")?>exprs</text>
img -> <img url="expr"|file="expr"/>
at -> <at uid="[0-9]+" (sep="char")/>
sign -> <sign s="char">
var 

expr-> 标记 | 字面量

exprs -> expr exprs
```