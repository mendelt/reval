# Writing Reval rules

## Intro

Reval is a library (or crate as libraries are called in Rust, the language that Reval is written in) for managing, interpreting and executing rules.
This document is targeted at people writing rules for Reval. It describes the syntax and structure of the rules. This document does not describe details of a specific Reval implementation. Any information on the structure of input data, output data or extra functions provided by any system that uses Reval should be described in the documentation of that system.

## Concepts

### Rules

Rules are pieces of text describing how to process input data to generate a required output. How the input and output data is structured depends depends on your particular Reval implementation.
Each rule has a name, this name is used to identify the rule in error messages and other output and should be unique in a ruleset. Each rule also contains an expression, this is the actual rule that will be evaluated to generate the output.

Rules can be written in one of two languages. The Reval language is the simpelest and most concise language to write rules. A simple rule written in the Reval language looks like this;
```
// Example rule
input_field >= i5
```

Rules can also be written in a language based on Json, this used to be the only way to write rules and is still available for backward compatibility. Parsing Json rules will be deprecated in version `0.8.0` of Reval and the Json rules parser will be removed in `0.9.0`.
The same rule that was specified above looks like this in Json, its quite a bit more verbose and harder to look at;
```json
{
    "name": "Example rule",
    "expr": {"gte": [{"ref": "input_field"}, {"int": "5"}]}
}
```

The rule has the name `"Example rule"` and compares the value in `input_field` with the integer `5`. It will return `true` if it is greater than or equal to 5 and `false` otherwise. This rule will return an error if `input_field` does not contain a value with an integer type.

### Values and data types

Values can either be passed in to the the rules as input data or specified in the rule itself.
Values passed in to the rules are named and can be used in rules by specifying the name similar to `input_field` in the example rule above.
The value `i5` in the example rule is a value that is specified in the rule itself. The prefix `i` specifies that the value has the Integer type, and `5` specifies the value.

Reval supports a range of data-types. The `String` type for text, a number of numeric types, a boolean type and a 

| Type | Example | Description |
| --- | --- | --- |
| String  | `"this is a String"i` | Values of the string type contain text. String values are delimited with `"` characters. |
| Integer | `i5`                 | This is a numeric type that can contain whole values. Integers can be positive or negative. `Integer` values are prefixed with `i` in rules |
| Float   | `f.14e-5`            | `Float` is a numeric type that contains floating point values. Float types can contain fractional values, positive or negative. `Float` values are prefixed with `f` |
| Decimal | `d14.25`             | `Decimal` is a numeric type that can contain fractional values. This data-type is similar to `Float` has different trade-offs. Decimal is typically used for monetary values. `Decimal` values are prefixed with `d` |
| Boolean | `false` or `true`    | The `Boolean` data type represent truth values and can be `true` or `false` |
| None    | `none`               | `None` is used to indicate that a value does is not set or unspecified. |
| Vec     | `[i4, i6, i12]`      | Vec values contain a list or vector of other values. |
| Map     | `{one: i1, two: i2}` | Map values map names to values, They are similar to a vec but every value in the list has a name that can be used to access that value |

### Operators

todo

### Built in Functions

todo

### User-Functions

A ruleset can have custom functions that allow rules to access functionality provided by the software that embeds the Reval rules-engine. Functions have a name, take a value as their input and return a value as output. Depending on how the function is implemented the output can be cached. So calling it multiple times might not incur a performance impact. 
