.name "factorial"
.parameter n,10

    getparameter n
    dup
    push0
    less
    jumpnonzero quit
    call fact
quit:
    exit
fact:
    dup
    jumpzero fact_base
    dup
    push1
    sub
    call fact
    mul
    ret
fact_base:
    pop
    push1
    ret