.name "Res"
.symbol "r"
.fgcolor "#ff0"
.bgcolor "#000"
.author "Dave Ackley"
.author "Alan Zaffetti"
.license "GPL-2.0-or-later"
.symmetries ALL
.radius 1

  push 1
  getsite
  getfield type
  gettype "Empty"
  equal
  jumpzero end
  push 0
  push 0
  getsite
  setregister
  push 0
  push 1
  getsite
  setsite
  push 1
  push 0
  getregister
  setsite