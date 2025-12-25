<div align="center">
    <h1><code>error-combinator</code></h1>
    <p><strong>Validation Support Tool</strong></p>
</div>

# error-combinator
error-combinator is a toolkit designed to streamline data validation workflows and error handling logic.
When validating complex data structures—such as user form inputs or configuration files—it is common practice to create granular checker functions, each returning a distinct error type. While Rust's Result<T, E> is powerful, coordinating multiple validation steps often leads to complexity, specifically regarding:

- Error Aggregation: How to merge varying error types into a single result.
- Redundancy: Ensuring the same validation logic isn't executed multiple times.

error-combinator was created to address these pitfalls by decoupling validation logic into independent, composable parts.

## Key Features
- Error Composition:
    Easily define rules for merging multiple distinct error types (e.g., combining Result<T, E1> and Result<T, E2>).
- Flow Control:
    Declaratively control the validation flow using combinators. You can specify whether to short-circuit on the first error (stop immediately) or continue processing subsequent checks (accumulate errors) even if a previous check failed.
- Type-Level Validation Status:
    Prevent redundant checks by expressing "validated" states at the type level. This ensures that a specific check is performed only once and provides compile-time guarantees of the data's validity.

## Usage
Here is an example of how to chain multiple validation checks. You can combine checkers and specify the error handling strategy (e.g., VecCombine to accumulate all errors) inline.

```rs
let checker = check_starts_with_hello
    .and::<_, VecCombine<ValidateErr>>(check_min6)
    .and::<_, VecCombine<ValidateErr>>(check_ends_with_world)
    .and::<_, VecCombine<ValidateErr>>(check_includes_abc);
```

