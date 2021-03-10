# Bytecode File Format

The bytecode file format is as follows:

```
File {
  u4          magic;
  u2          minor_version;
  u2          major_version;
  u1          build_tag_len;
  u1          [build_tag; build_tag_len];
  u1          metadata_size;
  md_entry    [metadata; metadata_size];
  u2          code_index_size;
  ci_entry    [code_index; code_index_size];
  u2          instruction_count;
  code_entry  [code; instruction_count];
}
```

All multi-byte numeric sequences are big-endian encoded.

## Magic Number

```
02 03 07 41
```

## Minor Version

Currently set to 1.

## Major Version

Currently set to 0.

## Compile Tag

A compile tag is used to identify a batch of files compiled together. In order to create a consistent type numbering (physics), only files with the same compile tag should be used together.

## Metadata Size

Size of the metadata map described in the following section.

## Metadata

The metadata map holds structured data for the program which is read as key-value pairs.

|Metadata Key|Byte Sequence|
|---|---|
|`.name`|`00`|
|`.symbol`|`01`|
|`.desc`|`02`|
|`.author`|`03`|
|`.license`|`04`|
|`.radius`|`05`|
|`.bgcolor`|`06`|
|`.fgcolor`|`07`|
|`.symmetries`|`08`|
|`.field`|`09`|
|`.parameter`|`0a`|

The value that follows depends on the key.

## Code Index Size

The size of the code index described in the next section.

## Code Index

The purpose of the code index is to record the types and custom bit-widths of arguments in the following code table.

This index is required to parse the code, as arguments will be packed into a number of bits specified by their `type`.

```
ci_entry {
  u2    instruction_idx;
  u1    type;
}
```

A multimap repeated as neccessary for instructions with multiple arguments.

A type is a single byte containing:

* A sign:
  * `Signed` (`0x80`)
  * `Unsigned` (`0x0`)
* A custom bit width ranging from 1 - 96.

### Example

The code index contents is shown inline in the comments:

```
.field is_foo,0,1

  push 0xfff /* Unsigned(24) */
  push1      /* Unsigned(6)  */
  push0      /* ... */
  getsite    /* => Unsigned(96) */
  push +1         /* Signed(2) */
  getfield is_foo /* Unsigned(1) */
  add             /* => Signed(3) */
```

Note that the code table need not represent the types of instructions lacking arguments as these are determined soley from their inputs (the resultant type denoted with a `=>`).

## Instruction Count

The total number of instructions. This defines the legal range of instruction pointers as `[0, code_lines)`. Labels and comments do not count as code lines.

## Code

Code is a sequence of opcodes followed by their arguments, if any.

```
oneof code_entry {
  u1   instruction;
  ??   [args; n];
}
```

The number of args `n` depends on the instruction (though most instructions have 0 or 1 argument).

The size of arguments are determined in the code index.
