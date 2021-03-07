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
|---|---|---|
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

The event window is indexed by site number:

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

### Builtin Fields

|||
|-----|-----|
|`type`|Type number part of the atom.|
|`header`|Header of the atom (`type` + `checksum`). Read only.|
|`data`|Data part of the atom.|

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
|`.name [NAME]`|The name of the element.|
|`.symbol [SYMBOL]`|A symbol for the element.|
|`.desc [DESC]`|A short description of the element; Repeatable.|
|`.author [AUTHOR]`|An author annotation. One author per line; Repeatable.|
|`.license [LICENSE]`|An SPDX license name.|
|`.radius [RADIUS]`|A maximum radius for the element; Values `[0-4]` are valid.|
|`.bgcolor [COLOR]`|A background color for frontends to use.|
|`.fgcolor [COLOR]`|A foreground color for frontends to use.|
|`.symmetries [SYM[\|...]]`|Default symmetries to use.|
|`.field [NAME],[POSITION],[BIT-LENGTH]`|A named accessor to element data; Repeatable.|
|`.parameter [NAME],[DEFAULT-VALUE]`|A named constant parameter; Repeatable.|

Metadata are read only and not programmatically accessible.

Parameters may be referenced by name to get their values.

### Instructions

Instructions fall roughly into one of three informal categories:

* **Window**: Manipulating the event window.
* **Logical**: Arithmetic using basic types and a stack.
* **Control**: Program flow control.

Numbered arguments presented in reverse-polish come from the stack. Named arguments are in place.

|Instruction||
|--------|---------|
|`nop`|Execute an nothing operation.|
|`exit`|Exit the program immediately.|
|`[1] [0] setsite`|Set the numbered site `[0]` to the value `[1]`.|
|`[1] [0] setregister`|Set the numbered register `[0]` to the value `[1]`.|
|`[0] getsite`|Get the numbered site `[0]` and push the value onto the stack.|
|`[0] getregister`|Get the numbered register `[0]` and push the value onto the stack.|
|`[0] getfield [FIELD]`|Gets the named field of `[0]` (i.e. `[0].[FIELD]`).|
|`[1] [0] setfield [FIELD]`|Sets the named field of `[0]` to `[1]` (i.e. `[0].[FIELD] = [1]`).|
|`gettype [TYPE]`|Gets the named type `[TYPE]` and pushes the value onto the stack.|
|`[0] scan`|Scan the event window for atoms of type `[0]`. Store the resulting presence bitmask on the stack.|
|`pushsymmetries [SYM[\|...]]`|Push the current symmetries onto the stack and use the new symmetries `[SYM[\|...]]`.|
|`[0] popsymmetries`|Restores the old symmetries off the stack.|
|`push [X]`|Push the value `[X]` onto the stack.|
|`pop`|Pop a value off the stack and discard it.|
|`call [LABEL]`|Call the labelled routine `[LABEL]`. The current instruction pointer is pushed onto the call stack.|
|`ret`|The previous instruction pointer is restored from the call stack.|
|`[0] checksum`|Checksum the header value of `[0]` which should be a full atom. Push the checksum result onto the stack: 1 if checksum differs; 0 otherwise.|
|`[1] [0] add`|Push `[0] + [1]` on the stack|
|`[1] [0] sub`|Push `[0] - [1]` onto the stack.|
|`[0] neg`|Push `-[0]` onto the stack.|
|`[1] [0] mod`|Push `[0] % [1]` onto the stack.|
|`[1] [0] mul`|Push `[0] * [1]` onto the stack.|
|`[1] [0] div`|Push `[0] / [1]` rounded down onto the stack.|
|`[1] [0] less`|Push comparing `[0] < [1]` (arithmetic) onto the stack.|
|`[1] [0] lessequal`|Push `[0] <= [1]` (arithmetic) onto the stack.|
|`[1] [0] or`|Push `[0] \|\| [1]` (logical) onto the stack.|
|`[1] [0] and`|Push `[0] && [1]` (logical) onto the stack.|
|`[1] [0] xor`|Push `[0] ^ [1]` (logical) onto the stack.|
|`[1] [0] equal`|Push `[0] == [1]` (logical) onto the stack.|
|`[0] bitcount`|Push the set bit count from `[0]` onto the stack.|
|`[0] bitscanforward`|Push LSB index from `[0]` (logical) onto the stack.|
|`[0] bitscanreverse`|Push MSB index from `[0]` (logical) onto the stack.|
|`[1] [0] lshift`|Push `[0] << [1]` (logical) onto the stack.|
|`[1] [0] rshift`|Push `[0] >> [1]` (logical) onto the stack.|
|`jump [LABEL]`|Jump to `[LABEL]` unconditionally.|
|`[0] jumprelativeoffset [LABEL]`|Jump unconditionally a number of instructions forward or backward specified by `[0]` (signed).|
|`[0] jumpzero [LABEL]`|Jump to `[LABEL]` iff `[0] == 0`.|
|`[0] jumpnonzero [LABEL]`|Jump to `[LABEL]` iff `[0] != 0`.|