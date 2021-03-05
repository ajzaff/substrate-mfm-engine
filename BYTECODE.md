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

All multi-byte numeric sequences are little-endian encoded.

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

### Entry Types

|||
|---|---|
|0|Parameter values|
|1|Constant arguments larger than `2^16-1`|
|2|Field Descriptors|
|3|Element type numbers|
|4|UTF-8 Strings|

#### Parameter

```
cp_info {
  u1       entry_type;
  u2       length;
  u1       [name; length];
  u12      param;
}
```

#### Const

```
cp_info {
  u1       entry_type;
  u12      const;
}
```

#### Field Descriptors

```
cp_info {
  u1       entry_type;
  u2       length;
  u1       [name; length];
  field    field;
}

field {
  u1       offset;
  u1       length;
}
```

#### Element Type Number

```
cp_info {
  u1       entry_type;
  u2       length;
  u1       [name; length];
  u2       num;
}
```

#### Strings

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

Metadata for the program. These are meta-instructions.

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