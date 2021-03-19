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

  gettype "Empty"    // %Empty
  push1
  getsitefield type  // #1$type
  equal              // #1$type == %Empty
  jumpzero quit
  push0
  push1
  swapsites          // swap(#0, #1)
quit: