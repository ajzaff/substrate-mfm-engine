.name "RandomWalk"
.desc "Diffuses and writes random colors."
.symbol "R"
.fgcolor "c0c"
.bgcolor "000"
.author "Alan Zaffetti"
.license "GPL-2.0-or-later"
.symmetries NONE
.radius 1

; RandomWalk is intended as a "Hello World" to demonstrate the imops capability.

paint:
  rand
  push 0xffffff
  and           ; c := [u24]
  push8
  rshift
  push 0xff     ; c := [u32]|0xff
  or
  setpaint
diffuse:
  rand
  push7
  and           ; r := [0-7]
  push1
  add           ; r := [1-8]
  push0
  swapsites     ; swap #r #0