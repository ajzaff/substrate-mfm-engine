.name "Res"
.symbol "r"
.fgcolor "ff0"
.bgcolor "000"
.author "Dave Ackley"
.author "Alan Zaffetti"
.author "Luke Wilson"
.license "GPL-2.0-or-later"
.symmetries NONE
.radius 1

  rand
  push7
  and                ; r:=[0-7]
  push1
  add                ; r:=[1-8]
  dup                ; r r
  getsitefield type  ; . #r.type
  gettype "Empty"    ; . %Empty
  equal              ; . #r.type == %Empty
  jumpzero quit
  push0              ; r 0
  swapsites          ; r <=> 0
quit: