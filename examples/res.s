.name "Res"
.symbol "r"
.fgcolor "#ff0"
.bgcolor "#000"
.author "Dave Ackley"
.author "Alan Zaffetti"
.author "Luke Wilson"
.license "GPL-2.0-or-later"
.symmetries ALL
.radius 1

  gettype "Empty"    /* %Empty */
  push 1
  getsitefield type  /* #1$type */
  equal              /* #1$type == %Empty */
  jumpzero end
  push 0
  push 1
  swapsites          /* swap(#0, #1) */
end: