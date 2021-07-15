
items -> item Items
item -> literal | tag


literal->litral | litral_tag

tag -> if
       | loops
        | var 
        | img | at | text

if -> <if mod="".... > items </if> elif

elif -> <elif mod='' ...>items </elif>
        | Nil
else -> <else>items </else>
        |Nil

loops -> loop | while | for 

while -><while mod=""...> items </while>
loop -> <loop times=""> items </loop>
for -> <for name="" from="">items</for>

litral_tag -> <text> items </text>
            | <sign s="" repeat=""/>
            | <endl/>

img-> <img url="" file=""/>
At -> <at uid="" sep=""/>