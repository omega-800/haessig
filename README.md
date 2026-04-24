# HÄSSIG

A swiss german (Zurich dialect) programming language. Recursive acronym stands for:    
         
H Hässig:    
Ä Äkronym    
S Sind    
S Schwierig    
I Also    
G Lahn ich's    
         
> [!CAUTION]    
> Still in early stages of active development    
> Even though this is a very serious project, compatibility with future versions is not guaranteed    

## Usage

```sh
haessig <input-file.hä>
./.build/out
```

## Grammar

### Example

```
dä x isch 5;
funktion brüeder het N8 y, N8 z git N8 {
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

## Current implementation

```BNF
<Program>       ::= <Stmt> <Program> | <Stmt>
<Expr>          ::= <StEx> | <Prim> | <Bin>
<Prim>          ::= <Str> | <Number> | <Id>
<Stmt>          ::= <Stmt'> ';'
<Stmt'>         ::= <FunAss> | <VarAss> | <StEx> | <Ret> 
<FunAss>        ::= <FunAss'> <FunAssArgs> <FunAssRet> <Block> | <FunAss'> <FunAssRet> <Block> | <FunAss'> <FunAssArgs> <Block> | <FunAss'> <Block>
<FunAss'>       ::= 'funktion' <Id> 
<FunAssArgs>    ::= <Type> <Id> ',' <FunAssArgs> | <Type> <Id>
<FunAssRet>     ::= 'git' <Type>
<VarAss>        ::= <VarAss'> | <VarAss'> <VarAssType>
<VarAss'>       ::= 'dä' <Id> 'isch' <Expr> 
<VarAssType>    ::= 'als' <Type>
<StEx>          ::= <Call> | <Block>
<Call>          ::= <Call'> | <Call'> 'mit' <CallArgs>
<CallArgs>      ::= <Expr> ',' <CallArgs> | <Expr>
<Call'>         ::= 'tuen' <Id> 
<Block>         ::= '{' <Program> '}'
<Ret>           ::= 'gib' <Expr>
<Type>          ::= 'N8' | 'Z8' | 'R8' | 'Zeiche' | 'Wahrheit'
<Bin>           ::= <StEx> <BinOp> <Expr> | <Prim> <BinOp> <Expr>
<BinOp>         ::= 'rescht'|'hoch'|'mal'|'durch'|'plus'|'minus'|'gliich'|'grösser'|'grösser gliich'|'chlinner'|'chlinner gliich'|'ungliich'|'und'|'oder'
```
```
ID  = [a-zA-Z_][0-9a-zA-Z_'-]*
STR = "([^"]|[^\\]\\")*"
NUM = -?[0-9]*([0-9]|([0-9]\.[0-9]))[0-9]*
BIN = (richtig|falsch)
```

All of the heavy lifting is currently being done by your cc, as the code is
being transpiled into c99 (badly). Future goal is a custom codegen implementation from
scratch.
