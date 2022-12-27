# Dependent Integer Types

- [Dependent Integer Types](#dependent-integer-types)
	- [Specification](#specification)
	- [Possible Types](#possible-types)

Interpreter for a tiny language used to test out dependent integer types before adding them to [Rym][rym].

## Specification

For general types (when the exact data is not know) these rules apply:

```lua
-- Difference
uint + uint => uint
uint + int => int
int + uint => int
int + int => int

-- Sum
uint - uint => int
uint - int => int
int - uint => int
int - int => int

-- Remainder
-- second parameter <= 0 is invalid
uint % uint => uint
int % uint => int
```

And for specific types (still without data) these:

```lua
u{a} + u{b}
	a < b => u{b + 1}
	a > b => u{a + 1}
	a == b => u{a|b + 1}

-- u{a} + i{b}
-- 	i{b} < 0 => u{a} - u{b - 1}
-- 	i{b} > 0 => u{a} + u{b - 1}
```

```lua
u{a} - u{b}
	a < b => i{b + 1}
	a > b => u{a}
	a == b => u{a|b}
```

```lua
u{a} % u{b}
	a < b => u{a}
	a > b => u{b}

i{a} % u{b}
	a < b => i{a}
	a > b => i{b}
```

```lua
u8  + u11  => u12
255 + 2047 => 2302
2^8 + 2^11 => 2^12

u2  + u1  => u3
4   + 1   => 5
2^2 + 2^1 => 2^3
```

## Possible Types

| Name | u1  | u2  | u3  | u4  | u5  | u6  | u7  | u8  | u9  | u10  | u11  | u12  |
| ---- | --- | --- | --- | --- | --- | --- | --- | --- | --- | ---- | ---- | ---- |
| Min  | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0   | 0    | 0    | 0    |
| Max  | 1   | 3   | 7   | 15  | 31  | 63  | 127 | 255 | 511 | 1023 | 2047 | 4095 |
| Size | 2   | 4   | 8   | 16  | 32  | 64  | 128 | 256 | 512 | 1024 | 2048 | 4096 |

| Name | i1  | i2  | i3  | i4  | i5  | i6  | i7  | i8   | i9   | i10  | i11   | i12   |
| ---- | --- | --- | --- | --- | --- | --- | --- | ---- | ---- | ---- | ----- | ----- |
| Min  | -1  | -2  | -4  | -8  | -16 | -32 | -64 | -128 | -256 | -512 | -1024 | -2048 |
| Max  | 0   | 1   | 3   | 7   | 15  | 31  | 63  | 127  | 255  | 511  | 1023  | 2047  |
| Size | 2   | 4   | 8   | 16  | 32  | 64  | 128 | 256  | 512  | 1024 | 2048  | 4096  |

[rym]: https://github.com/creatorsiso/rym
