# Bytecode File Format

The bytecode file format is as follows:

```
File {
  u4          magic;
  u2          minor_version;
  u2          major_version;
  u2          constant_pool_size;
  cp_info     [constant_pool; constant_pool_size];
  u1          metadata_size;
  md_info     [metadata; meta_instr_size];
  u2          code_lines;
  u8          [code; code_lines];
}
```

## Magic Number

```
02 03 07 41
```

## Minor Version

Currently set to 1.

## Major Version

Currently set to 0.

## Constant Pool Size

Size of the constant pool described in the following section.

## Constant Pool

The constant pool holds a variety of structured data from the program.

* Parameter values
* Field Descriptors
* Element type numbers
* Constant arguments larger than `2^16-1`
* UTF-8 Strings

For fixed 96-bit values:

```
cp_info {
  u1       entry_type;
  u12      data;
}
```

The following entry format is used for strings only:

```
cp_info {
  u1       entry_type;
  u2       length;
  u1       [data; length];
}
```

## Metadata Size

The size of the metadata table described in the following section.

## Metadata Table

Metadata for the program. Since all values are stored within the constant pool, a fixed-size structure suffices.

```
md_info {
  u2       entry_type;
  u2       cp_index;
}
```

## Code Lines

The number of code lines in the following table. This defines the legal range of instruction pointers as `[0, code_lines)`. Labels and comments do not count as code lines.

## Code

Code is an array of fixed 64-bit instructions. The [LAYOUT](LAYOUT.md) document defines the actual instruction layout.