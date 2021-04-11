.name "InvertWalk"
.desc "Diffuses and inverts site paint."
.symbol "i"
.fgcolor "ccc"
.bgcolor "000"
.author "Alan Zaffetti"
.license "GPL-2.0-or-later"
.symmetries ALL
.radius 1

; InvertWalk is intended to test the site paint integrity.

paint:
  getpaint
  push 0xffffff00
  xor
  setpaint
diffuse:
  push1
  push0
  swapsites        ; #1 <=> #0