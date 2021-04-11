.name "YellowWalk"
.desc "Diffuses and leaves behind yellow paint."
.symbol "y"
.fgcolor "cc0"
.bgcolor "000"
.author "Alan Zaffetti"
.license "GPL-2.0-or-later"
.symmetries ALL
.radius 1

; YellowWalk is intended to test the site paint integrity.

paint:
  push 0xffff00ff  ; yellow
  setpaint
diffuse:
  push1
  push0
  swapsites        ; #1 <=> #0