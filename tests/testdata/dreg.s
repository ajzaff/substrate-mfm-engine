.NAME DReg
.DESC DReg implements a dynamic space regulator atom.
.DESC Roles include producing Res and occasionally deleting sites.
.AUTHOR Alan Zaffetti
.AUTHOR Dave Ackley
.LICENSE LGPL
.RADIUS 2
.BGCOLOR #000
.FGCOLOR #f0f

  JumpNotZero occupied #1$type
  Copy R_0 R_UniformRandom
  JumpRelativeOffset 4 ?9
  JumpRelativeOffset 5 ?11
  Move RSN RAS       // move.
  Jump yield
  Move RAT %Res      // create Res with 1/9 chance.
  Jump yield
  Move RAT %DReg     // create DReg with 8/9 x 1/11 chance.
  Jump yield
occupied:
  Move R0 RAT
  Compare R0 %DReg
  JumpEqualZero destroy_chance R0  // (type == DReg)
  And R0 ?3                //   && (1/3 chance)
  JumpEqualZero yield R0
  Jump destroy
destroy_chance:
  JumpEqualZero exit ?8  // 1/8 chance: clear the site.
destroy:
  Move RAT RAV %Empty
exit: