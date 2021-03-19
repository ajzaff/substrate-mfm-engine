.name "SuperFork"
.symbol "X"
.fgcolor "000"
.bgcolor "f00"
.author "Dave Ackley"
.author "Alan Zaffetti"
.license "GPL-2.0-or-later"
.symmetries NONE
.radius 1

    push40           // [i:=40]
loop:
    dup              // [i,i]
    push0            // [i,i,0]
    getsite          // [i,i,#0]
kill:
    setsite          // #i=#0; [i]
    push1            // [i,1]
    sub              // [i-1]
    dup              // [i-1,i-1]
    jumpnonzero loop // [i-1]
quit: