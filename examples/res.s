.name "Res"
.symbol "r"
.fgcolor "#ff0"
.bgcolor "#000"
.author "Dave Ackley"
.author "Alan Zaffetti"
.license "GPL-2.0-or-later"
.symmetries ALL
.radius 1

  gettype "Empty"   /* %Empty */
  push 1
  getsite           /* #1 */
  getfield type
  equal             /* #1$type == %Empty */
  jumpzero end
  push 0
  push 0
  getsite           /* #0 */
  setregister       /* r0 = #0 */
  push 0
  push 1
  getsite           /* #1 */
  setsite           /* #0 = #1 */
  push 1
  push 0
  getregister       /* r0 */
  setsite           /* #1 = r0 */
end: