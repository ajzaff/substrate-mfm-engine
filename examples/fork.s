.name "Fork"
.symbol "X"
.fgcolor "000"
.bgcolor "f00"
.author "Dave Ackley"
.author "Alan Zaffetti"
.license "GPL-2.0-or-later"
.symmetries NONE
.radius 1

    rand
    push7
    and        ; r:=[0-7]
    push1
    add        ; r:=[1-8]
    push0      ; r 0
    getsite    ; r #0
    setsite    ; #0 = r