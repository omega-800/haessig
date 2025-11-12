# HÄSSIG

A swiss german (Zurich dialect) programming language. Acronym stands for:    
         
H Hässig, Rekursivi     
Ä Äkronym    
S Sind    
S Schwierig    
I Also    
G Lahn ich's    
         
> [!CAUTION]    
> Still in early stages of active development    
> Even though this is a very serious project, compatibility with future versions is not guaranteed    

## Grammar

### Example

```
dä x isch 5;
funktion brüeder het Zahl y, Zahl z git Zahl {
    dä resultat isch 
        y mal x
            wenn z chlinner y suscht 
        z minus x plus y;
    gib resultat;
};
dä wasauimmer isch brüeder mit 7 16;
schrei mit "s resultat isch: " plus wasauimmer;

// s resultat isch: 18
```

### current implementation

```EBNF
<Program>       ::= <Stmt>+
<Expr>          ::= (<StEx> | <Prim>)
<Prim>          ::= (<Str> | <Number> | <Id>)
<Stmt>          ::= (<FunAss> | <VarAss> | <StEx> | <Ret>)) ';'
<FunAss>        ::= 'funktion' <Id> ('git' <Type>)? <Block>
<VarAss>        ::= 'dä' <Id> 'isch' <Expr> ('als' <Type>)?
<StEx>          ::= (<Call> | <Block>)
<Call>          ::= 'tuen' <Id> ('mit' (<Expr> ',')* <Expr>)?
<Block>         ::= '{' <Stmt>+ '}'
<Ret>           ::= 'gib' <Expr>
<Type>          ::= ('N8' | 'Z8' | 'R8' | 'Zeiche' | 'Wahrheit')
```

### EBNF

```EBNF
<Program>       ::= <Statement>+
<StEx>          ::= (<Call> | <IfElse> | <Block>)
<Statement>     ::= (<VarAssignment> | <FunAssignment> | <TypeDef> | <StructDef> | <EnumDef> | <Method> | <Until> | <While> | <DoWhile> | <For> | <ForEach> | <StEx>) ';'
<Expression>    ::= (<Ternary> | <BinaryBool> | <Binary> | <Unary> | <Group> | <Primitive> | <StEx> | <Deref> | <Ref>)
<Ref>           ::= '&' <Expression>
<Deref>         ::= '*' <Expression>
<IfElse>        ::= 'falls' <Expression> 'dänn' (<Statement>) ('suscht' 'falls' <Expression> 'dänn' <Statement>)* ('suscht' <Statement>)?
<Ternary>       ::= <Expression> 'wenn' <Expression> 'suscht' <Expression>
<LetIn>         ::= 'lahn' ((<VarAssignment> | <FunAssignment>) ';')+ 'in' <Expression>
<VarAssignment> ::= 'dä' <Identifier> 'isch' <Expression> ('als' <Type>)?
<FunAssignment> ::= 'funktion' <Identifier> ('het' (<Type> <Identifier> ',')+)? ('git' <Type>)? <Block>
<Call>          ::= 'tuen' <Identifier> ('mit' (<Expression> ',')+)?
<Until>         ::= 'bis' <Expression> 'mach' <Block>
<While>         ::= 'während' <Expression> 'mach' <Block>
<DoWhile>       ::= 'mach' <Block> 'solang' <Expression>
<For>           ::= 'für' <VarAssignment> 'bis' <Expression> 'denn' <Expression> 'mach' <Block>
<ForEach>       ::= 'jede' <Identifier> 'in' <Expression> 'tuet' <Block>
<TypeDef>       ::= 'typ' <Identifier> 'beschriibt' <Type> ';'
<StructDef>     ::= 'struktur' <Identifier> 'beschriibt' '{' (<Identifier> ':' <Type> ',')+ '}' ';'
<Method>        ::= 'methode' <Identifier> 'vo' <Identifier> 'het' ((<Type> <Identifier> ',')+ | 'nüt') ('git' <Type>)? <Block>
<EnumDef>       ::= 'ufzellig' <Identifier> 'beschriibt' (<Type> ',')+ ';'

# <BinaryBool>    ::= <Expression> ('=='|'>'|'>='|'<'|'<='|'!='|'&&'|'||') <Expression>
<BinaryBool>    ::= <Expression> ('gliich'|'grösser'|'grösser gliich'|'chlinner'|'chlinner gliich'|'ungliich'|'und'|'oder') <Expression>
# <Binary>        ::= <Expression> ('%'|'^'|'*'|'/'|'+'|'-') <Expression>
<Binary>        ::= <Expression> ('rescht'|'hoch'|'mal'|'durch'|'plus'|'minus') <Expression>
# <Unary>         ::= ('!'|'-') <Expression>
<Unary>         ::= ('nöd'|'minus') <Expression>
<Group>         ::= '(' <Expression> ')'
<Block>         ::= '{' <Statement>+ '}'
<Primitive>     ::= <String> | <Number> | <Boolean> | <List> | <Object> | <Identifier>
<List>          ::= '[' (<Primitive> ',' )* <Primitive>? ']'
<Object>        ::= '{' (<Identifier> ':' <Primitive> ',')* (<Identifier> ':' <Primitive>)? '}'
<String>        ::= STR 
<Number>        ::= NUM
<Boolean>       ::= BIN
<Identifier>    ::= ID
<Builtin>       ::= 'schrei' | 'gib' | 'verlang'

<Type>          ::= <TypeStr> | <TypeNum> | <TypeBin> | <TypeList> | <TypeObj>
<TypeStr>       ::= 'Zeiche'
<TypeNum>       ::= 'Zahl'
<TypeBin>       ::= 'Wahrheit'
<TypeList>      ::= 'Liste'
<TypeObj>       ::= 'Objäkt'

ID  = [a-zA-Z_][0-9a-zA-Z_'-]*
STR = "([^"]|[^\\]\\")*"
NUM = -?[0-9]*([0-9]|([0-9]\.[0-9]))[0-9]*
BIN = (wahr|falsch)
```
