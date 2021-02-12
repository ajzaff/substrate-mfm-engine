# Data Layout

This appendix document defines the bit layout for the virtual machine.

## Instructions

64-bit instructions.

||Reserved|Opcode|`DST`|`SRC`/`LHS`|`RHS`|
|---|---|---|---|---|---|
|size|8|8|16|16|16|


## Registers

96-bits. May be represented as any of: `u64`, `u32`, or three `u32`s, or if available `u128`.

## Atoms

||Atom Type|ECC Checksum|User State|
|---|---|---|---|
|size|16|9|71|

## Arguments

Instruction argments use a 16-bit layout.

||Value Type|Data/Reference|Field (Optional)|
|---|---|---|---|
|size|2|14||
|size (reference)|2|7|7|

Over the following possible value types:

|Value Enumeration|Value|
|---|---|
|Inline|0|
|Heap|1|
|Register (Reference)|2|
|Site (Reference)|3|

### Constants

Unsigned literals `x < 2^14-1` or signed `-2^13 < x < 2^13` are stored inline using 14-bit representations.

Constants which cannot be packed into 14-bits are stored in the constant pool on the heap.

The meta-instruction `.Parameter` will always use the constant pool.

The heap size is limited by the address space: no more than `2^14` elements.

### Types

Element type numbers are preprocessed into inline constants. In the rare case there are more than `2^14-1` elements, heap slots are used for types.

### Registers

Registers are referenced by index. 16 registers exist in total, where register 16 (`0xff`) is the read-only uniform random register.

### Fields

Fields are heap allocated and referenced by index. Registers and site references take an optional field part (as seen in the table above) allowing 7-bits for each.

This implies a upper limit of 122 user-space fields (less 4 builtin fields and 0 being used as a flag value).