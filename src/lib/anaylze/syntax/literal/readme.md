
// 总语句
items -> itemmeta item

item-> itemmeta item
      | Nil

// 字面和标签
itemmeta -> literal | tag 

// 字面量
literal-> text 文本

tag -> ctrl_tag
        | info_tag
        | litral_tag
        

ctrl_tag->if
       | loops
        | var 

info_tag-> img | at

if -> <if mod="".... > items </if> elif

elif -> <elif mod='' ...>items </elif>
        | Nil
else -> <else>items </else>
        |Nil

// 循环语句
loops -> loop | while | for 

while -><while mod=""...> items </while>
loop -> <loop times=""> items </loop>
for -> <for name="" from="">items</for>


litral_tag -> 
            | <sign s="" repeat=""/>
            | <endl/>

img-> <img url="" file=""/>
At -> <at uid="" sep=""/>