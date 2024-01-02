# SymboScript optimizer

## Introduction

SymboScript optimizer is an optimizer for SymboScript. It is used to optimize the AST.

## Example

```syms
1+2+(2*2)z;
```

become

```syms
3+4z;
```
