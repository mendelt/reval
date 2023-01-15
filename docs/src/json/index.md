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

## Values
The simpelest expressions are value expressions. Reval values can have different types and there is one value expression for each type. The primitive types that Reval uses are `String`, `Int`, `Decimal`, `Float` and `Bool` and there are expressions for each of these.

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

## Conversions
Reval has three expressions that allow conversions between numeric types, each of these takes one expression as a parameter.

* CFloat
* CInt
* CDecimal

## Comparison
Reval supports a number of comparison expressions that can be used to compare values. Al these expressions result in a boolean value.

* Eq
* Neq
* Gt
* Gte
* Lt
* Lte

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

## Special expressions
There are a couple of special expressions that allow 
### Access to input data
* Ref
### Field access for Map and Vec types
* Idx
