# Data Layout

This appendix document defines the bit layout for the virtual machine.

## OpCodes

Opcodes use a single byte.

## Arguments

Arguments may vary in size to save space and an index records this info (see [BYTECODE.md](BYTECODE.md)).

## Registers

96-bits.

## Atoms

||Atom Type|ECC Checksum|User State|
|---|---|---|---|
|size|16|9|71|

### Registers

Registers are referenced by index. 16 registers exist in total, where register 16 (`0xff`) is the read-only uniform random register.