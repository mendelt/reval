# Reference

A reference guide to quickly find Reval expressions

## Expressions

|Expr | Description |
|:--- | :--- |
|string | Construct a simple string value |
|int | Construct a simple int value |
|float | Construct a simple float value |
|decimal | Construct a simple decimal value |
|bool | Construct a simple boolean value |
|map | Construct a map value |
|vec | Construct a vec value |
|not | Logical inversion, converts `true` to `false` and `false` to `true` |
|and | Logical and, is `true` only when all parameters evaluate to `true` |
|or | Logical or, is `false` only when all parameters evaluate to `false` |
|neg | Numeric inversion, makes positive values negative and negative values positive |
|add | Addition, adds up numeric values |
|sub | Subtraction, subtracts numeric values |
|mul | Multiplication, multiplies numeric values |
|div | Division, divides numeric values |
|if | Control flow expression, `if` takes three expressions as parameters.  If the first expression evaluates to `true` the if expression will take the value of the second or `then` expression.  If the first expression evaluates to `false` the if expression will take the value of the third or `else` expression. |
|ref | Ref references a field from the input |
|idx | Access fields in map or vec types. `idx` indexes fields of maps with strings and vec fields can be indexed using integer values |
|func | Call a custom fuction by name |
|cint | Convert a value to an integer |
|cfloat | Convert a value to float |
|cdecimal | Convert a value to decimal |
