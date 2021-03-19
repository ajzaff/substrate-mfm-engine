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
.field done,  40,1

// Edge detection is a simple operation which finds sharp changes in brightness in an image.
// It's also a kernel convolution which means there's a simple local procedure for calculing
// the result. A perfect fit for the MFM architecture!
//
// [[ -1 -1 -1 ]]
// [[ -1  8 -1 ]]
// [[ -1 -1 -1 ]]
//
// A single element initializes the operation and runs all parts of the computation.
// The resulting image is read from and wrote to the site paint.
//
// The element works by copying itself across the grid rapidly, then, when all 8 neighbors
// are present, writes the output to the site paint and self destructs.
//
// One quirk is the site paint is only accessible at site 0, so atoms must expose their
// site paint in a field named `paint`.
//
// The `done` bit indicates we've writen our result and are waiting to self-destruct.

                               // Have we convolved already? Have our neighbors convolved?
                               // We might be able to self destruct.
  push0
  getsitefield done            // #0.done
  jumpzero store_paint
                               // We are done with the convolution. Check our neighbors.
  push8                        // i
check_done_loop:
  dup                          // i i
  getsitefield done            // #i.done
  jumpzero store_paint_break
  push1
  sub                          // i--
  jumpnonzero check_done_loop
                               // Everyone is done... Let's self-destruct.
self_destruct:
  push0
  push0
  setsite                      // #0 = 0
  exit                         // Bye!

store_paint_break: pop         // From: check_done_loop
store_paint:                   // Otherwise proceed normally. Store the site paint into our `paint` field.
  push 0
  getpaint
  setsitefield paint           // #0.paint = getpaint()

                               // The ready_loop makes sure all neighbors are present to begin the
                               // convolution. If not, we will create them later.
  push8
ready_loop:
  dup                          // i i
  getsitefield type            // i.type
  jumpzero reproduce           // We're not ready to convolve; reproduce.
  push1
  sub                          // i--
  jumpnonzero ready_loop       // break

                               // The convolution operation begins here:
  push0                        // Initialize an accumulator for the convolution.
  push8                        // i
convolve_loop:
  dup                          // i i
  getsitefield paint
  neg                          // -#i.paint
  add                          // acc += -i.paint
  swap                         // acc, i
  push1
  sub                          // acc, i-1
  dup                          // acc, i-1, i-1
  jumpnonzero convolve_loop    // break

                               // Finish the convolution by adding `8 * 0.paint`.
  getsitefield paint           // #0.paint
  push8
  mul                          // 8 * #0.paint
  add                          // acc += 8 * #0.paint
  setpaint                     // acc now contains the edge detection result.
                               // Store it in the site paint and set our done status.
done:
  push0
  push1
  setsitefield done            // #0.done = 1
  exit                         // Bye!

reproduce: pop                 // From: ready_loop. Spread myself.
  push0
  gettype "EdgeDetect"
  setfield type                // Stack now contains a new empty EdgeDetect atom.
  push8                        // a i
reproduce_loop:
  over
  over                         // a i a i
  swap
  setsite                      // #i = a
  push1
  sub                          // a i-1
  dup
  jumpnonzero reproduce_loop

quit:                          // Goodbye!