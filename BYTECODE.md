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
  u2          code_lines;
  code_entry  [code; code_lines];
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

The metadata map holds structured data for the program which mostly read like key-value pairs.

## Code Index Size

Not implemented.

Size of the code index described in the next section.

## Code Index

Not implemented.

Used to record the number and types of arguments in the code table.

```
ci_entry {
  u1     num_args;
  u1     [arg_types; num_args];
}
```

## Code Lines

The total number of code lines. This defines the legal range of instruction pointers as `[0, code_lines)`. Labels and comments do not count as code lines.

## Code

Code is a sequence of opcodes and arguments in reverse polish notation.

```
oneof code_entry {
  ??   [args; ??];
  u1   instruction;
}
```