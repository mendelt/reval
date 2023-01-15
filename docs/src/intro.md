# Writing Reval rules

## Intro

Reval is a library (or crate as libraries are called in Rust, the language that Reval is written in) for managing, interpreting and executing rules.
This document is targeted at people writing rules for Reval. It describes the syntax and structure of the rules. This document does not describe details of a specific Reval implementation. Any information on the structure of input data, output data or extra functions provided by any system that uses Reval should be described in the documentation of that system.

## Concepts

### Rules

Rules are pieces of text describing how to process input data to generate the required output. How the input and output data is structured depends depends on your particular Reval implementation.
Each rule has a name, this name is used to identify the rule in error messages and other output and should be unique in a ruleset. Each rule also contains an expression, this is the actual rule that will be evaluated to generate the output.

Rules can be writting in one of two languages. First is a language based on the Json text format. This is a very precise but verbose way of expressing rules. A rule written in the Json language will look like this;
```json
{
    "name": "Example rule",
    "expr": {"gte": [{"ref": "input_field"}, {"int": "5"}]}
}
```

This rule has the name `"Example rule"` and compares the value in `input_field` with the integer `5`. It will return `true` if it is greater than or equal to 5 and `false` otherwise. This rule will return an error if `input_field` does not contain a value with an integer type.

Reval will also support a more concise Reval language. Right now this language is still experimental and only supports a small subset of the Reval features. In the future it might replace the json language. The rule that we just looked at might look like this in the Reval language
```
### Example rule
[input_field] >= 5
```

### Rulesets

Rules are bundled into rulesets. A ruleset will typically contain rules that work on the same type of input data and produce the same type of output data. Normally all the rules in a ruleset are run on each piece of input data. The output from running each of the rules is then combined.
A ruleset that is used to trigger some action might contain rules that will each return a boolean, like the example rule in the previous paragraph. The values output by each of the rules will then be combined so that the action is triggered when one of the rules was triggered. Or outputs of rules might also be averaged or added together.

Data passed into the rules is usually one or more named values. These can be accessed inside the rule. These values can have primitive types like `string`, `integer`, `decimal`, `float` or `boolean`, or they can themselves be structured values, they can be a `map` or a `vector` containing either named values or numbered values.

### Functions

A ruleset can also have custom functions that allow rules to access functionality provided by the software that embeds the Reval rules-engine. Functions have a name, take a value as their input and return a value as output. Depending on how the function is implemented the output can be cached. So calling it multiple times might not incur a performance impact. 

### Tests

T.B.D.