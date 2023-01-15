# Json rules

A rule is written as a json structure with a name and an associated expression. The name is what identifies the rule and should be unique in the rule-set.
The expression is what will be evaluated.
The expression is a nested tree-structure. At the root is the first expression that will be evaluated. Expressions can have parameters and depending on the expression these parameters can themselves be expressions.

```json
{
    "name": "Example rule",
    "expr": {"string": ""}
}
```

# Expressions

Expressions are written as a key-value pair in json, the key is the name of the expression and the value is either one parameter if the expression only takes one parameter. In most cases expressions take more than one parameter so the value will be a list.

Expression names are in lower-case.

## Value Expressions
The simpelest expressions are value expressions. Reval values can have different types and there is one value expression for each type. The primitive types that Reval uses are `String`, `Int`, `Decimal`, `Float` and `Bool` and there are expressions for each of these;

```json
{ "string": "This is a string value" }
```

```json
{ "int": -5 }
```

```json
{ "decimal": 5.132 }
```

```json
{ "float": 5.15e28 }
```

```json
{ "bool": true }
```

## Comparison Expressions
Reval supports a number of comparison expressions that can be used to compare values. Al these expressions result in a boolean value.

### Eq and neq
Eq is short for "equals" and compares equality. It returns true when the two parameters are exactly the same
```json
{ "eq": [{"int": 5}, {"ref": "input"}] }
```

Neq is short for "not equal" and is the inverse of the `eq` expression. Compares equality of two parameters and returns `false` if they are the same.
```json
{ "neq": [{"int": 5}, {"ref": "input"}] }
```

## Special expressions
There are a couple of special expressions that allow express
### Ref
The `ref` expression allow rules to access fields from input data by name

```json
{"ref": "input_field_name"}
```

### Idx
The `idx` expression indexes fields from map or vec values.

Fields from a vec can be indexed by number.
```json
{"idx": [{ "ref": "some_vec_value" }, { "int": 5 } ]}
```

Fields from maps are indexed by string.
```json
{"idx": [{ "ref": "some_map_value" }, { "string": "sub-field" } ]}
```

### 

### Gt, gte, lt and lte
Gt is short for "greater than" it compares two numeric values of the same type, lt is "less than" and is the inverse. Gte and lte are the inclusive versions of these operations. They are short for "Greater than or equal" and "Less than or equal"

```json
{ "gt": [{"int": 5}, {"ref": "input"}] }
```

```json
{ "gte": [{"int": 5}, {"ref": "input"}] }
```

```json
{ "lt": [{"int": 5}, {"ref": "input"}] }
```

```json
{ "lte": [{"int": 5}, {"ref": "input"}] }
```

## Logic expressions
Logic expressions perform logic operations on boolean values. These can be used to combine results from comparison expressions for example. Logic expressions take one or more boolean expressions as parameters and return a boolean value when evaluated.

### Not
The not expression inverts a boolean value. So `true` becomes `false` and `false` becomes `true`.

```json
{ "not": { "bool": false } }
```

### And
The logical `and` expression takes two sub-expressions as parameters and only evaluates to `true` if both parameters evaluate to `true`.

```json
{
    "and": [ 
        { "gte": [{ "ref": "input_1" }, { "int": 5 }]},
        { "gte": [{ "ref": "input_2" }, { "float": 15.9 }]}
    ]
}
```

### Or
The logical `or` expression takes two sub-expressions as parameters and only evaluates to `false` if both parameters evaluate to `false`.

```json
{
    "or": [ 
        { "gte": [{ "ref": "input_1" }, { "int": 5 }]},
        { "gte": [{ "ref": "input_2" }, { "float": 15.9 }]}
    ]
}
```

## Calculation
* Add
* Sub
* Mul
* Div


## Conversions
Reval has three expressions that allow conversions between numeric types, each of these takes one expression as a parameter.

* CFloat
* CInt
* CDecimal
