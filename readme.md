# Msg-Template-Enginer--消息模板引擎

* 
```xml
    <text line="true">这是自动添加换行符结尾的文本行</text>
    <text>这是没有自动添加换行符结尾的文本行<endl/></text>
    <? 这里面是注释 ?>
    <text>
    内部格式会被忽略，除非使用符号标记: 换行<sign s="\n"/>
    或者<endl/>也能换行
    符号可以是任意符号，比如<sign s="<">
    可以是任何可能会被识别为标记的符号或者字符，长度不限<endl/>
    可以设置复读次数
    <endl/><sign s="-" repeat="6"/><endl/>
    长度为6的分割线
    </text>
    <img url="http://"/>是一张图片，来源网络，原位置由url指定<endl/>
    <img file="/var/pic/..."/>是一张图片，来源本地文件<endl/>
    <at uid="114514" sep=" "/><sign s="@"/>某人<endl/>
    <var name="value_name" mod="new" value="114145"/>
    可以创建一个变量,value视为表达式处理<endl/>
    <?表达式只有 value(变量取用)，'字面量' 2种?>
    <if mod="eq" left="value_name" right="'value'">
    <?'value'视为字面量即字符串value，否则视为表达式 即 value变量的值?>
        如果条件为真，将会输入这里的内容<endl/>
        `<var name="value_name" mod="print"/>`会显示变量<endl/>
        也支持更加复杂的显示
        <var name="value_name" mod="println"/>
        <sign s="{"/>好耶<endl/>
    </if>
    <elif mod="boolT" value="true">
        真的<sign s="/n"/>
    </elif>
    <else>
        <var name="value_name" mod="assign" value="emmc"/>
    </else>
    <loop times="5">
        循环打印5次<endl/>
    </loop>
    <loop times="5" name="i">
        <var name="i" mod="print" format="第 {} 次">
        <? 打印时支持格式化，但是不能执行复杂格式化语句 ?>
        ）循环打印5次<endl/>
    </loop>
    <for name="i" from="value_name">
        从变量<var name="value">里面循环获取变量<endl/>
        为<sign s=":"><var name="i" mod="println">
    </for>
    <while mod="boolT" value="false">
        直到循环条件不满足，才会终止<endl>
    </while>
    <text>
        if和while的mod有这些： <endl/>
    eq相等，neq不等，      <endl/>gt大于，gte大于等于，  <endl/>
    lt小于，lte小于等于，  <endl/>boolT真，boolF非真    <endl/></text>
```

