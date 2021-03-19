.name "EdgeDetect"
.desc "Runs edge detection on an input image"
.symbol "C"
.fgcolor "0c0"
.bgcolor "000"
.author "Alan Zaffetti"
.license "GPL-2.0-or-later"
.symmetries NONE
.radius 1
.field paint, 0,32
  .field r,   24,8
  .field g,   16,8
  .field b,   8,8
  .field rr   32,16
  .field gg   16,16
  .field bb   0,16
.field done,  40,1

; Edge detection is a simple operation which finds sharp changes in brightness in an image.
; It's also a kernel convolution which means there's a simple local procedure for calculing
; the result. A perfect fit for the MFM architecture!
;
; [[ -1 -1 -1 ]]
; [[ -1  8 -1 ]]
; [[ -1 -1 -1 ]]
;
; A single element initializes the operation and runs all parts of the computation.
; The resulting image is read from and wrote to the site paint.
;
; The element works by copying itself across the grid rapidly, then, when all 8 neighbors
; are present, writes the output to the site paint and self destructs.
;
; One quirk is the site paint is only accessible at site 0, so atoms must expose their
; site paint in a field named `paint`.
;
; The `done` bit indicates we've writen our result and are waiting to self-destruct.

                               ; Have we convolved already? Have our neighbors convolved?
                               ; We might be able to self destruct.
  push0
  getsitefield done            ; #0.done
  jumpzero store_paint
                               ; We are done with the convolution. Check our neighbors.
  push8                        ; i
check_done_loop:
  dup                          ; i i
  getsitefield done            ; #i.done
  dup
  getsitefield type            ; #i.type
  gettype "EdgeDetect"
  equal
  dup
  xor
  or                           ; #i.done || %"EdgeDetect" != #i.type
  jumpzero store_paint_break
  push1
  sub                          ; i--
  jumpnonzero check_done_loop
                               ; Everyone is done... Let's self-destruct.
self_destruct:
  push0
  push0
  setsite                      ; #0 = 0
  exit                         ; Bye!

store_paint_break: pop         ; From: check_done_loop
store_paint:                   ; Otherwise proceed normally. Store the site paint into our `paint` field.
  push 0
  getpaint
  setsitefield paint           ; #0.paint = getpaint()

                               ; The ready_loop makes sure all neighbors are present to begin the
                               ; convolution. If not, we will create them later.
  push8
ready_loop:
  dup                          ; i i
  getsitefield type            ; i.type
  gettype "EdgeDetect"
  equal                        ; i.type == %"EdgeDetect"
  jumpzero reproduce           ; We're not ready to convolve; reproduce.
  push1
  sub                          ; i--
  jumpnonzero ready_loop       ; break

                               ; The convolution operation begins here:
  push0                        ; Initialize an accumulator.
                               ; Using bitwise field operations, We can compute
                               ; each color component within a single loop.
  push8                        ; i
convolve_loop:
  dup                          ; acc i i
  getsitefield paint           ; acc i #i.paint
  swap                         ; acc #i.paint i
  rot                          ; i acc #i.paint
  swap                         ; i #i.paint acc
                               ; red:
  over                         ; ...            #i.paint
  getfield r                   ; ...            #i.r
  over
  getfield rr                  ; ...            #i.r acc.rr
  add
  setfield rr                  ; ...        acc.rr += i.r
                               ; green:
  over                         ; ...            #i.paint
  getfield g                   ; ...            #i.g
  over                         ;                #i.g acc
  getfield gg                  ; ...            #i.g acc.gg
  add
  setfield gg                  ; ...        acc.gg += i.g
                               ; blue:
  over                         ; ...            #i.paint
  getfield r                   ; ...            #i.b
  over
  getfield rr                  ; ...            #i.b acc.bb
  add
  setfield rr                  ; ...        acc.bb += i.b
  rot                          ; acc i #i.paint
  pop                          ; acc i
  push1
  sub                          ; acc i-1
  dup                          ; acc i-1 i-1
  jumpnonzero convolve_loop
  pop                          ; acc
                               ; Negate each color channel:
  dup                          ; red:
  getfield rr
  neg
  setfield rr                  ; acc.rr = -acc.rr
  dup                          ; green:
  getfield rr
  neg
  setfield rr                  ; acc.gg = -acc.gg
  dup                          ; blue:
  getfield bb
  neg
  setfield bb                  ; acc.bb = -acc.bb

                               ; Finish the convolution by adding `8 * 0.paint`
                               ; to each color channel:
  push0
  getsitefield paint           ; acc #0.paint
  dup                          ; acc #0.paint #0.paint
  rot                          ; #0.paint acc #0.paint
                               ; red:
  getfield r                   ; ...      acc #0.r
  push8
  mul                          ; ...      acc (8 * #0.r)
  over                         ; ...      acc #0.r acc
  getfield rr
  add
  setfield rr                  ; ...      acc += #0.r
  swap                         ; acc #0.paint
  dup                          ; acc #0.paint #0.paint
  rot                          ; #0.paint acc #0.paint
                               ; green:
  getfield g                   ; ...      acc #0.g
  push8
  mul                          ; ...      acc (8 * #0.g)
  over                         ; ...      acc #0.g acc
  getfield gg
  add
  setfield gg                  ; ...      acc += #0.g
  swap                         ; acc #0.paint
  dup                          ; acc #0.paint #0.paint
  rot                          ; #0.paint acc #0.paint
                               ; blue:
  getfield b                   ; ...      acc #0.b
  push8
  mul                          ; ...      acc (8 * #0.b)
  over                         ; ...      acc #0.b acc
  getfield bb
  add
  setfield bb                  ; ...      acc += #0.b
  swap                         ; acc #0.paint
  pop                          ; acc contains the result with 16-bit color channels.
                               ; `setfield` will handle clipping channels to RGBA8.
  dup
  getfield rr
  setfield r                   ; acc.r = acc.rr
  dup
  getfield gg
  setfield g                   ; acc.g = acc.gg
  dup
  getfield bb
  setfield b                   ; acc.b = acc.bb
  setpaint                     ; Store the result in the site paint and set our done status.

done:
  push0
  push1
  setsitefield done            ; #0.done = 1
  exit                         ; Bye!

reproduce: pop                 ; From: ready_loop. Spread myself.
  push0
  gettype "EdgeDetect"
  setfield type                ; Stack now contains a new empty EdgeDetect atom.
  push8                        ; a i
reproduce_loop:
  over
  over                         ; a i a i
  swap
  setsite                      ; #i = a
  push1
  sub                          ; a i-1
  dup
  jumpnonzero reproduce_loop

quit:                          ; Goodbye!
