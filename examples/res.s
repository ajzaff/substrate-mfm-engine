.name "Res"
.symbol "r"
.fgcolor "ff0"
.bgcolor "000"
.author "Dave Ackley"
.author "Alan Zaffetti"
.author "Luke Wilson"
.license "GPL-2.0-or-later"
.symmetries ALL
.radius 1

  push1
  getsitefield type  ; #1.type
  gettype "Empty"    ; %Empty
  equal              ; #1.type == %Empty
  jumpzero quit
  push1
  push0
  swapsites          ; 1 <=> 0
quit: