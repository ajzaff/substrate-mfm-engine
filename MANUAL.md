# MFM VM Manual

A specification for MFM VMs implemented by the Substrate engine.

## Introduction

This manual contains a specification for MFM VMs using the words _should_, and _can_.

This is intended to provide instructions in case anyone finds themselves providing a
compliant implementation.

## Specification

Substrate implements high-level opcodes for MFM element programs.
This is not a comprehensive list.

### A program _should_

* have a lifetime (runtime) consisting of one or more program runs.
* have state persisted so long as the parent element (origin element) exists.
* provide exclusive access to a local spatial memory (event window) during each run.
* provide access to 96-bit element registers.

### A program _may_

* have a runtime consisting of runs sharded arbitrarily (e.g. by limiting the number of instructions per run).
* provide access to 16 additional 96 bit runtime-persisted registers.

### An event window _should_

* have a size corresponding to a city-block distance `[0-4]` around the origin.
* have standard site indices:

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

* support standard symmetries.

### A register _should_

* be unsigned.
* only if holding element data: contain type information, a checksum, and free data bits. 

### An element _should_

* have a 96-bit data representation consisting of type information, a checksum, and free data bits.
* provide typed fields within free data bits.

### Misc. requirements

* A program manager _should_ provide globally unique IDs to elements (within the context of a physics).
* SPLAT bindings and rule storage are optional but are required for compiling from SPLAT.

### Features

#### Constants

A constant typed value can be used wherever a constant expression is expected.

```
  5      // unsigned integer by default
  5u     // unsigned integer
	-1i    // signed integer
	0b0111 // binary
```

Boolean `true` and `false` are 1-bit values equivalent to `1u` and `0u`. 

#### Labels

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

#### Comments

```
// This is a line comment.
start:
  FOO // This is an inline comment.
```

#### Symmetries

Symmetries simplify the definition of elements with common symmetries.

Symmetries apply when window references are used.

When a command has window references and symmetry is not `NONE`: the cross-product of 
candidate sites are dereferenced and sampled uniformly and the first matching permutation
is used.

Symmetries:

* `None`
* `All`

#### Window Deference

The event window should be indexed by site number (refer to the table in the specification section of this document).

A leading `#` and an unsigned integer (without the `u`) is used to reference a site relative to the origin (`#0`).

```
  #0 // self
	#1 // left, assuming NONE symmetries, or adjacent to me.

```

#### Field Dereference

Fields can be dereferenced with `$field_name` to get the value stored in `field_name` in the origin element data.

Other registers can be dereferenced by prepending the register name (as in `R_register$field_name`).

This can be used wherever an expression is expected.

A failure to dereference a field is implementation specific.

```
.Field active_count
.Field is_active 1

  Add R_1 $active_count R_SelfData$is_active // R1 = active_count + is_active;
```


##### Builtin Field References

* `...$type`: evaluates to the type number of the element.
* `...$checksum`: evaluates to the checksum of the element. Read only.
* `...$checksum`: evaluates to the header of the element. Read only.
* `...$data`: evaluates to the data part of the element.

#### Type Dereference

Type names can be dereferenced with `%` to get their type number.

This can be used wherever a constant expression is expected.

Type names cannot begin with a number.

This is read only.

```
%Empty // by convention == 0
%DReg  // e.g. 1
%Res   // e.g. 2
```

##### Self Type

The type of the current element is named `Self` and can be dereferenced using `%Self`.

### Registers

#### R_SelfRaw

Raw element data. Read only.

#### R_SelfType

Element type data. Meaningful within the context of a specific MFM physics.

#### R_SelfHeader

Type and checksum content. Read only.

#### R_SelfChecksum

Checksum of the header content. Read only.

#### R_SelfData

Free element data.

#### R_[0-15]

Extra persistant 96-bit registers 0-15.

### Metadata

#### .Name `NAME`

The name of the element.

#### .Desc `DESC`

A short description of the element.

#### .Author `AUTHOR`

A repeatable author annotation. One author per line.

#### .License `LICENSE`

A SPDX license name.

#### .Radius `RADIUS`

A radius for the element. Values `[0-4]` are valid.

#### .BgColor `COLOR`

A background color for frontends to use.

#### .FgColor `COLOR`

A foreground color for frontends to use.

#### .Symmetries `SYMMETRIES [SYMMETRIES...]`

The symmetries to use. Defaults to `None`.

#### .Field `NAME` `BIT-LENGTH`

A repeatable field annotation. A field value is a named accessor to element data.

Field layout is implementation dependent.

### Instructions

Instructions define the basic operations available to a program.

Operands fall roughly into one of three informal categories:

* **Window**: Manipulating the event window.
* **Logical**: Arithmetic using basic types.
* **Control**: Program flow control.

Placeholders `SRC`, `LHS`, and `RHS` generally refer to any constant or deferencable expression.

`DST` should be a writeable register, field, or element reference.

#### Nop

Execute an nothing operation.

#### Exit

Exit program immediately.

#### Copy `DST` `SRC`

Store the value of `SRC` into `DST`.

Copy the element at `SRC` to `DST`.

#### Swap `DST` `SRC`

Swap the values of `SRC` and `DST`.

Swap the elements at `SRC` and `DST`.

#### Add `DST` `LHS` `RHS`

Store the result of `LHS + RHS` into `DST`.

#### Sub `DST` `LHS` `RHS`

Store the result of `LHS - RHS` into `DST`.

#### Mul `DST` `LHS` `RHS`

Store the result of `LHS * RHS` into `DST`.

#### Negate `DST` `SRC`

Store the result of `-SRC` (arithmetic) into `DST`.

#### Or `DST` `LHS` `RHS`

Store the result of `LHS || RHS` (logical) into `DST`.

#### And `DST` `LHS` `RHS`

Store the result of `LHS && RHS` (logical) into `DST`.

#### Xor `DST` `LHS` `RHS`

Store the result of `LHS ^ RHS` (logical) into `DST`.

#### Not `DST` `SRC`

Store the result of `!SRC` (logical) into `DST`.

#### Equal `DST` `LHS` `RHS`

Store the result of `LHS == RHS` (logical) into `DST`.

#### Compare `DST` `LHS` `RHS`

Store the result of comparing `LHS` and `RHS`:

* -1 if `LHS < RHS`
*  0 if `LHS == RHS`
*  1 if `LHS > RHS`

into `DST` (as a 2s-complement signed value).

#### Jump `LABEL`

Jump to `LABEL` unconditionally.

### JumpRelativeOffset `SRC`

Jump unconditionally a number of instructions forward or backward specified by `SRC` (signed 2s-complement).

#### JumpZero `LABEL` `SRC`

Jump to `LABEL` iff `SRC == 0`.

#### JumpNonZero `LABEL` `SRC`

Jump to `LABEL` iff `SRC <> 0`.

#### JumpLessThanZero `LABEL` `SRC`

Jump to `LABEL` iff `SRC < 0` (`SRC` interpreted as a 2s-complement signed value).

#### JumpGreaterThanZero `LABEL` `SRC`

Jump to `LABEL` iff `SRC > 0` (`SRC` is interpreted as a 2s-complement signed value).

#### BitwiseAnd `DST` `LHS` `RHS`

Store the result of `LHS & RHS` (bitwise) into `DST`.

#### BitwiseOr `DST` `LHS` `RHS`

Store the result of `LHS | RHS` (bitwise) into `DST`.

#### BitwiseNot `DST` `SRC`

Store the result of `^SRC` (bitwise) into `DST`.

#### SPLAT (Spatial rule) Support

SPLAT operations define support for SPLAT binding.

In order to fit the spatial rule blocks into a linear format, we denormalize the representation
using a few operations designed to be used together hierarchically as if writing XML.

SPLAT operations are evaluated as a block and not counted as individual instructions (for the purposes of `JumpRelativeOffset`).

##### SPLAT_BEGIN

Begin a new SPLAT rule block.

##### SPLAT_RULE `NAME`

Name the SPLAT rule.

##### SPLAT_LET `CHAR`

Implements the `let` statement in SPLAT.

Starts a new normal instruction block up until the next `Exit` in which:

* `R_1` is set to the candidate element's raw value.
* `R_2` is set to the candidate element's site number.

The value stored in `R_0` (upon evaluating `Exit`) is interpreted as the boolean `let`-predicate result.

###### SPLAT_TRANSFORM `OFFSET` `LHS` `RHS`

Defines a SPLAT transform where `OFFSET` is a window reference (e.g. `#1`) and `LHS` and `RHS` are defined by `SPLAT_LET` blocks.

##### SPLAT_END

End the SPALT rule block.