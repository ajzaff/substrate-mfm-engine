# MFM VM Manual

A specification for MFM VMs implemented by the Substrate engine.

## Introduction

This manual contains a specification for a MFM VM.

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
  -1     // signed integer.
  +1     // signed integer.
  0b0111 // binary; always unsigned.
  0xffff // hex; always unsigned.
```

#### Limits

Constants are limited to 96-bits in size.

||`unsigned`|`signed`|
|---|---|---|---|
|Min|`0`|`-2^95`|
|Max|`2^96-1`|`2^95-1`|

### Registers

|Register Name||
|--------|---------|
|`R[0-14]`|Intermediate registers 0-14; 96-bits each.|
|`R?`|A uniform random source; 96-bits.|

### Comments

```
// This is a line comment.
start:
  foo // This is an inline comment.
```

### Symmetries

Symmetries are supported natively in the engine.

Default symmetries may be specified and symmetries can change during execution using `usesymmetries` and `restoresymmetries`.

Symmetries affect which sites are referred to.

|Symmetry Name|Type|
|-----|-----|
|`R000L`|rotation|Default rotation (normal).|
|`R090L`|rotation|90 degrees, counter-clockwise.|
|`R180L`|rotation|180 degrees, counter-clockwise.|
|`R270L`|rotation|270 degrees, counter-clockwise.|
|`R000R`|rotation|0 degrees, clockwise flipped.|
|`R090R`|rotation|90 degrees, clockwise flipped.|
|`R180R`|rotation|180 degrees, clockwise flipped.|
|`R270R`|rotation|270 degrees, clockwise flipped.|
|`ALL`|convenience|All rotations.|

### Special Operands

A few special operands are provided. They are as follows:

|Operand||
|-----|-----|
|`$`|Field operand. Used to access named fields.|
|`#`|Window operand. Used to access the event window.|
|`%`|Type operand. Used to get the atom's type from its common name.|
|`_`|Skip operator. Used to skip arguments in instructions, instead using what is atop the stack.|

#### Field Operand (`$`)

Sections of data can be marked as fields using the `.Field` meta-instruction.

`$foo` may be used to get the value stored in the named field `foo`.

This expression may be appended to the end of any data to access fields in that data.

* While reading: data fields are shifted such that the LSB is zero.
* While writing: values are likewise corrected, and truncated to fit within `length`-bits.

Anonymous fields may be referenced using the syntax: `$(offset; length)`.

```
.field foo 1 0 // field named foo; 1-bit length; LSB at position 0.

  $foo        // $foo in Self atom; same as #0$field_name.
  $foo$signed // $foo as a signed value.
  R0$foo      // $foo in R0.
  #1$foo      // $foo in atom data at site #1.
  R1$(10; 1)  // anonymous field in the 11th bit of R1.


.field active_count 4 1
.field is_active 1 0

  add $active_count $is_active
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
  #foo     // atom at site $foo.
```

Invalid window locations should appear as empty or otherwise void.

#### Type Operand (`%`)

Common element type names can be dereferenced with `%` to get their type number.

Type names cannot begin with a number.

This is read only.

```
  %Empty       // by convention == 0.
  %DReg        // e.g. 1.
  %Res         // e.g. 2.
  %Self        // type of the current atom.
  %"Long Name" // Long name
```

#### Skip Operator (`_`)

The skip operator can be used for syntactic sugar.

```
  add _,R0
```

Is the same as:

```
  pop R1
  add R1,R0
```

But without the extra pop. It may be useful in some situations.

### Labels

A label is represented by label name and a `:`.

```
  copy R0 1
loop:
  add R0 R0
  jump loop
```

Control operands can jump to named labels.

A label at the end of the program is often provided, conventionally called `exit`.

```
  // ...
  jumpnonzero exit $field
  sub $field 1
exit:
  // program ends
```

### Metadata

Meta-instructions are generally specified once at the start of a program.

|Metadata||
|--------|--------|
|`.name NAME`|The name of the element.|
|`.symbol SYMBOL`|A symbol for the element.|
|`.desc DESC`|A short description of the element; Repeatable.|
|`.author AUTHOR`|An author annotation. One author per line; Repeatable.|
|`.license LICENSE`|An SPDX license name.|
|`.radius RADIUS`|A maximum radius for the element; Values `[0-4]` are valid.|
|`.bgcolor COLOR`|A background color for frontends to use.|
|`.fgcolor COLOR`|A foreground color for frontends to use.|
|`.symmetries SYMMETRY|[SYMMETRIES...]`|Default symmetries to use.|
|`.field NAME,POSITION,BIT-LENGTH`|A named accessor to element data; Repeatable.|
|`.parameter NAME,DEFAULT-VALUE`|A named constant parameter; Repeatable.|

Metadata are read only and not programmatically accessible.

Parameters may be referenced by name to get their values.

### Instructions

Instructions fall roughly into one of three informal categories:

* **Window**: Manipulating the event window.
* **Logical**: Arithmetic using basic types and a stack.
* **Control**: Program flow control.

Instructions are 64-bits. The layout is defined in [LAYOUT.md](LAYOUT.md).

|Instruction||
|--------|---------|
|`nop`|Execute an nothing operation.|
|`exit`|Exit the program immediately.|
|`copy DST,SRC`|Store the value of `SRC` into `DST`. Copy the atom at `SRC` to `DST`.|
|`swap DST,SRC`|Swap the values at `SRC` and `DST`.|
|`scan SRC`|Scan the event window for atoms of the given `%Type` specified by `SRC`. Store the resulting mask on the stack.|
|`usesymmetries SYM\|[SYM...]`|Push the current symmetries onto the stack, and use the given ones.|
|`restoresymmetries`|Pop the symmetries off the stack and use them.|
|`push SRC`|Push `SRC` onto the stack.|
|`pop DST`|Pop a value off the stack into `DST`.|
|`call LABEL`|Push the instruction pointer and jump to the labelled instruction.|
|`ret`|Pop and return to the last instruction pointer on the stack.| 
|`checksum SRC`|Checksum the atom at `SRC`. Push the checksum result on the stack: 1 if checksum differs; 0 otherwise.|
|`add LHS,RHS`|Push `LHS + RHS` (arithmetic) on the stack|
|`sub LHS,RHS`|Push `LHS - RHS` (arithmetic) onto the stack.|
|`neg SRC`|Push `-SRC` (arithmetic) onto the stack.|
|`mod LHS,RHS`|Push `LHS % RHS` (arithmetic) onto the stack.|
|`mul LHS,RHS`|Push `LHS * RHS` (arithmetic) onto the stack.|
|`div LHS,RHS`|Push `LHS / RHS` (arithmetic) rounded down onto the stack.|
|`less LHS,RHS`|Push comparing `LHS < RHS` (arithmetic) onto the stack.|
|`lessequal LHS,RHS`|Push `LHS <= RHS` (arithmetic) onto the stack.|
|`or LHS,RHS`|Push `LHS \|\| RHS` (logical) onto the stack.|
|`and LHS,RHS`|Push `LHS && RHS` (logical) onto the stack.|
|`xor LHS,RHS`|Push `LHS ^ RHS` (logical) onto the stack.|
|`equal LHS,RHS`|Push `LHS == RHS` (logical) onto the stack.|
|`bitcount SRC`|Push the number of set bits from `SRC` (logical) onto the stack.|
|`bitscanforward SRC`|Push LSB index from `SRC` (logical) onto the stack.|
|`bitscanreverse SRC`|Push MSB index from `SRC` (logical) onto the stack.|
|`lshift LHS,RHS`|Push `LHS << RHS` (logical) onto the stack.|
|`rshift LHS,RHS`|Push `LHS >> RHS` (logical) onto the stack.|
|`jump LABEL`|Jump to `LABEL` unconditionally.|
|`jumprelativeoffset LABEL,SRC`|Jump unconditionally a number of instructions forward or backward specified by `SRC` (may be signed).|
|`jumpzero LABEL,SRC`|Jump to `LABEL` iff `SRC == 0`.|
|`jumpnonzero LABEL,SRC`|Jump to `LABEL` iff `SRC <> 0`.|