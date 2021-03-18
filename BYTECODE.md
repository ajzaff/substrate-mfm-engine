# Bytecode File Format

The bytecode file format is as follows:

```
File {
  u4          magic;
  u2          minor_version;
  u2          major_version;
  u1          build_tag_len;
  u1          [build_tag; build_tag_len];
  u2          self_type_num;
  u1          metadata_size;
  md_entry    [metadata; metadata_size];
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

## Build Tag

A compile tag is used to identify a batch of files compiled together. In order to create a consistent type numbering (physics), only files with the same compile tag should be used together.

## Self Type Number

My type number, equivalent to `gettype "Self"` or `push0 getsitefield type`.

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

### Types

A byte is used to represent the type of constants that appear in code. See the compiler code for more details.

```
.field is_foo,0,1

  push 0xfff      /* Unsigned(16) */
  push1           /* Unsigned(8)  */
  push0           /* Unsigned(8) */
  getsite         /* => Unsigned(128) */
  push +1         /* Signed(8) */
  getfield is_foo /* Unsigned(8) */
  add             /* => Signed(16) */
```

Note that the code table need not represent the types of instructions lacking arguments as these are determined soley from their inputs (the resultant type denoted with a `=>`).