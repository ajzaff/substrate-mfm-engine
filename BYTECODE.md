# Bytecode File Format

The bytecode file format is as follows:

```
File {
  u4          magic;
  u2          minor_version;
  u2          major_version;
  u8          compile_tag;
  u1          metadata_size;
  md_entry    [metadata; metadata_size];
  u2          code_lines;
  var         [code; code_lines];
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

Size of the constant pool described in the following section.

## Metadata

The metadata map holds structured data for the program which are mostly UTF-8 Strings.

### String Entry

Strings have `entry_type` 0.

```
md_entry {
  u1       entry_type;
  u2       len;
  u1       [data; len];
}
```

## Code Lines

The total number of code lines. This defines the legal range of instruction pointers as `[0, code_lines)`. Labels and comments do not count as code lines.

## Code

Code consists of varint-encoded opcodes and arguments in reverse polish notation.

Example encoding:

```
[u1; overhead] [u16; instruction] [u1; overhead] [u96; arg] ...
```

The overhead byte enum:

```
enum overhead {
  instruction = 0;
  arg = 1;
}
```