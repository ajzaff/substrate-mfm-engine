# MFM VM Manual

A specification for MFM VMs implemented by the Substrate engine.

## Introduction

This manual contains a specification for MFM VMs.

This is intended to provide instructions in case anyone finds themselves providing a
compliant implementation.

Some background on the project is assumed.

## Specification

### Data Types

The following basic types are supported:

* `unsigned`
* `signed`

The latter uses [Two's complement](https://en.wikipedia.org/wiki/Two's_complement).

In operations taking one or more argument, operation will take place in `signed` mode if one or more arguments is `signed`.

Where not explicitly marked, `unsigned` is default.

### Constants

A constant typed value can be used wherever a constant expression is expected.

```
  5      // unsigned integer by default
  5u     // explicit marking may be omitted.
  -1i    // signed integer.
  0b0111 // binary; always unsigned.
  0xffff // hex; always unsigned.
```

### Registers

|Register Name||
|--------|---------|
|`R_[0-15]`|Intermediate registers 0-15; 96-bits each.|
|`R_UniformRandom`|A uniform random source; 96-bits.|

### Comments

```
// This is a line comment.
start:
  FOO // This is an inline comment.
```

### Symmetries

Symmetries are supported natively in the engine.

Default symmetries may be specified and symmetries can change during execution using `UseSymmetries` and `RestoreSymmetries`.

Symmetries affect which sites are referred to.

|Symmetry Name|Type|
|-----|-----|
|`R_000L`|rotation|Default rotation (normal).|
|`R_090L`|rotation|90 degrees, counter-clockwise.|
|`R_180L`|rotation|180 degrees, counter-clockwise.|
|`R_270L`|rotation|270 degrees, counter-clockwise.|
|`R_000R`|rotation|0 degrees, clockwise flipped.|
|`R_090R`|rotation|90 degrees, clockwise flipped.|
|`R_180R`|rotation|180 degrees, clockwise flipped.|
|`R_270R`|rotation|270 degrees, clockwise flipped.|
|`Flip_X`|flip|Same as `R_180R`|
|`Flip_Y`|flip|Same as `R_000R`|
|`Flip_XY`|flip|Same as `R_180L`|
|`Reflect_X`|reflect|Same as `R_000L`, `Flip_X`|
|`Reflect_Y`|reflect|Same as `R_000L`, `Flip_Y`|
|`All`|convenience|All rotations.|

### Special Operands

A few special operands are provided. They are as follows:

|Operand||
|-----|-----|
|`$`|Field operand. Used to access named fields.|
|`#`|Window operand. Used to access the event window.|
|`#+`|Multiple data window operand. Used to trigger multiple data access mode.|
|`%`|Type operand. Used to get the atom's type from its common name.|

#### Field Operand (`$`)

`$foo` may be used to get the value stored in the named field `foo`.

Sections of data can be marked as fields using the `.Field` meta-instruction.

This may be appended to the end of any data to access fields in that data.

While reading: data fields are shifted such that the LSB is zero.

While writing: values are likewise corrected, and truncated to fit within `length`-bits.

```
.Field foo unsigned 1 0 // field named foo; 1-bit length; LSB at position 0.

  $foo        // $foo in Self atom; same as #0$field_name.
  $foo$signed // $foo as a signed value.
  R_0$foo     // $foo in R_0.
  #1$foo      // $foo in atom data at site #1.


.Field active_count unsigned 4 1
.Field is_active unsigned 1 0

  Add R_0$active_count $active_count $is_active
```

Some fields are built-in. These are also reserved names.

|Builtin Fields||
|-----|-----|
|`type`|Type number part of the atom.|
|`checksum`|Checksum part of the atom. Read only.|
|`header`|Header of the atom (`type` + `checksum`). Read only.|
|`data`|Data part of the atom.|

#### Window Operand (`#`)

The event window is indexed by site number.

```
            38
         31 22 33
      25 15 10 17 27
   29 13  5  2  7 19 35
37 21  9  1 *0  4 12 24 40
   30 14  6  3  8 20 36
      26 16 11 18 28
         32 23 34
            39
```

Symmetries affect what a site number refers to. Namely one of the valid rotation is sampled at random.

```
  #0       // my atom; full 96-bit data.
  #1       // atom at site #1.
  #$foo    // atom at $foo.
```

Invalid window locations should appear as empty or otherwise void.

Multiple data mode is triggered by `#+`. This coreces the input to a bit-mask.

```
  #+$foo   // multiple sites selected from bit-vector $foo.
  #+%Empty // Empty atom sites.
  #+%Type  // Type-element sites.
  #+0x14B7 // explicit marked sites.
```

|Builtin Fields||
|-----|-----|
|`type`|Type number part of the atom.|
|`checksum`|Checksum part of the atom. Read only.|
|`header`|Header of the atom (`type` + `checksum`). Read only.|
|`data`|Data part of the atom.|
|`signed`|Corece as a signed value.|

#### Type Operand (`%`)

Common element type names can be dereferenced with `%` to get their type number.

Type names cannot begin with a number.

This is read only.

```
  %Empty // by convention == 0.
  %DReg  // e.g. 1.
  %Res   // e.g. 2.
  %Self  // type of the current atom.
```

### Labels

A label is represented by label name and a `:`.

```
  Copy R_0 1
loop:
  Add R_0 R_0 R_0
  Jmp loop
```

Control operands can jump to named labels.

A label at the end of the program is often provided, conventionally called `exit`.

```
  // ...
  JmpNonzero exit $field
  Sub $field $field 1
exit:
  // program ends
```

### Metadata

Meta-instructions are specified once at the start of a program.

|Metadata||
|--------|--------|
|`.Name NAME`|The name of the element.|
|`.Desc DESC`|A short description of the element; Repeatable.|
|`.Author AUTHOR`|An author annotation. One author per line; Repeatable.|
|`.License LICENSE`|An SPDX license name.|
|`.Radius RADIUS`|A maximum radius for the element; Values `[0-4]` are valid.|
|`.BgColor COLOR`|A background color for frontends to use.|
|`.FgColor COLOR`|A foreground color for frontends to use.|
|`.Symmetries SYMMETRY [SYMMETRIES...]`|Default symmetries to use.|
|`.Field NAME TYPE POSITION BIT-LENGTH`|A named accessor to element data; Repeatable.|

Metadata are read only and not programmatically accessible.

### Instructions

Instructions fall roughly into one of three informal categories:

* **Window**: Manipulating the event window.
* **Logical**: Arithmetic using basic types.
* **Control**: Program flow control.

Placeholders `DST`, `SRC`, `LHS`, and `RHS` refer to any expression. `DST` should be a writeable.

Instructions support multiple data mode (by using arguments with the `#+` operator), except for noted exceptions.

|Instruction||
|--------|---------|
|`Nop`|Execute an nothing operation.|
|`Exit`|Exit the program immediately.|
|`Copy DST SRC`|Store the value of `SRC` into `DST`. Copy the atom at `SRC` to `DST`.|
|`Swap DST SRC`|Swap the values of `SRC` and `DST`. Swap the atoms at `SRC` and `DST`.|
|`Checksum DST SRC`|Checksum the `SRC` atoms and store 1 if checksum differs; 0 for OK.|
|`UseSymmetries SYM [SYM...]`|Switch to using the given symmetries.|
|`RestoreSymmetries`|Restore the default symmetries; When no `.Symmetries` entry is present, this is `R_000L` (normal).|
|`Add DST LHS RHS`|Store the result of `LHS + RHS` (arithmetic) into `DST`.|
|`Sub DST LHS RHS`|Store the result of `LHS - RHS` (arithmetic) into `DST`.|
|`Mul DST LHS RHS`|Store the result of `LHS * RHS` (arithmetic) into `DST`.|
|`Negate DST SRC`|Store the result of `-SRC` (arithmetic) into `DST`.|
|`Mod DST LHS RHS`|Store the result of `LHS % RHS` (arithmetic) into `DST`.|
|`Div DST LHS RHS`|Store the result of `LHS / RHS` (arithmetic) rounded down into `DST`.|
|`Sum DST SRC`|Store the result of summing `SRC` (arithmetic) into `DST`.|
|`Min DST SRC`|Store the result of minimizing `SRC` into `DST`.|
|`Max DST SRC`|Store the result of maximizing `SRC` into `DST`.|
|`Argmin DST SRC`|Store the result of computing `argmin SRC` into `DST`.|
|`Argmax DST SRC`|Store the result of computing `argmax SRC` into `DST`.|
|`Or DST LHS RHS`|Store the result of `LHS || RHS` (logical) into `DST`.|
|`And DST LHS RHS`|Store the result of `LHS && RHS` (logical) into `DST`.|
|`Xor DST LHS RHS`|Store the result of `LHS ^ RHS` (logical) into `DST`.|
|`Not DST SRC`|Store the result of `!SRC` (logical) into `DST`.|
|`LessThan DST LHS RHS`|Store the result of comparing `LHS < RHS` (logical) into `DST`.|
|`LessThanEqual DST LHS RHS`|Store the result of `LHS <= RHS` (logical) into `DST`.|
|`Equal DST LHS RHS`|Store the result of `LHS == RHS` (logical) into `DST`.|
|`LShift DST LHS RHS`|Store the result of `LHS << RHS` (logical) into `DST`.|
|`RShift DST LHS RHS`|Store the result of `LHS >> RHS` (logical) into `DST`.|
|`BitwiseAnd DST LHS RHS`|Store the result of `LHS & RHS` (bitwise) into `DST`.|
|`BitwiseOr DST LHS RHS`|Store the result of `LHS | RHS` (bitwise) into `DST`.|
|`BitwiseNot DST SRC`|Store the result of `^SRC` (bitwise) into `DST`.|
|`BitCount DST SRC`|Store the number of set bits from `SRC` (bitwise) into `DST`.|
|`BitScanForward DST SRC`|Store the masked LSB from `SRC` (bitwise) into `DST`.|
|`BitScanReverse DST SRC`|Store the masked MSB from `SRC` (bitwise) into `DST`.|
|`Jump LABEL`|Jump to `LABEL` unconditionally.|
|`JumpRelativeOffset LABEL SRC`|Jump unconditionally a number of instructions forward or backward specified by `SRC` (may be signed).|
|`JumpZero LABEL SRC`|Jump to `LABEL` iff `SRC == 0`.|
|`JumpNonZero LABEL SRC`|Jump to `LABEL` iff `SRC <> 0`.|