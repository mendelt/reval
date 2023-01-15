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
There are a couple of special expressions that allow 
### Access to input data
* Ref
### Field access for Map and Vec types
* Idx

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

## Logic
Logic expressions perform logic operations on boolean values. These can be used to combine results from comparison expressions for example. Logic expressions take one or more boolean expressions as parameters and return a boolean value when evaluated.

* Not
* And
* Or

If is not a logic expression but it works well with logic expressions. It can be used to evaluate an expression and return one value if the expression is true and another value when that expression is false. The returned values themselves are retrieved by evaluating expressions so `if` can be used to implement simple control flow in expressions

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
