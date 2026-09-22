# Progressive hints

Open only the section you need. Try hint 1 before revealing hint 2.

## ex01

<details><summary>Hint 1</summary>

Use string formatting.

</details>

<details><summary>Hint 2</summary>

The format! macro returns an owned String.

</details>

## ex02

<details><summary>Hint 1</summary>

Use floating-point literals.

</details>

<details><summary>Hint 2</summary>

Multiply by 9.0 / 5.0 before adding 32.0.

</details>

## ex03

<details><summary>Hint 1</summary>

A match guard adds a condition.

</details>

<details><summary>Hint 2</summary>

Handle zero, then a negative guard, then the remaining case.

</details>

## ex04

<details><summary>Hint 1</summary>

Start an accumulator at zero.

</details>

<details><summary>Hint 2</summary>

Convert each value with u64::from before adding.

</details>

## ex05

<details><summary>Hint 1</summary>

The parameter can be mutable even though ownership is moved.

</details>

<details><summary>Hint 2</summary>

Use push with a char, then return text.

</details>

## ex06

<details><summary>Hint 1</summary>

str::len counts bytes.

</details>

<details><summary>Hint 2</summary>

Iterate with chars, then count.

</details>

## ex07

<details><summary>Hint 1</summary>

You can mutate the borrowed String without returning it.

</details>

<details><summary>Hint 2</summary>

Use push_str for a string slice.

</details>

## ex08

<details><summary>Hint 1</summary>

A lifetime connects the returned reference to the inputs.

</details>

<details><summary>Hint 2</summary>

Use the same lifetime on both inputs and the output; >= selects the first on ties.

</details>

## ex09

<details><summary>Hint 1</summary>

Construct the struct with named fields.

</details>

<details><summary>Hint 2</summary>

Field shorthand lets name mean name: name.

</details>

## ex10

<details><summary>Hint 1</summary>

Use self to access fields.

</details>

<details><summary>Hint 2</summary>

Convert dimensions before multiplying.

</details>

## ex11

<details><summary>Hint 1</summary>

Match each variant and bind its payload.

</details>

<details><summary>Hint 2</summary>

Each match arm returns a u64.

</details>

## ex12

<details><summary>Hint 1</summary>

Return early when a match is found.

</details>

<details><summary>Hint 2</summary>

After the loop, return None.

</details>

## ex13

<details><summary>Hint 1</summary>

Create a new Vec.

</details>

<details><summary>Hint 2</summary>

Only push a value when contains returns false.

</details>

## ex14

<details><summary>Hint 1</summary>

HashMap::entry combines lookup and insertion.

</details>

<details><summary>Hint 2</summary>

or_insert(0) returns a mutable reference to the counter.

</details>

## ex15

<details><summary>Hint 1</summary>

parse infers the target type from the return signature.

</details>

<details><summary>Hint 2</summary>

Return the Result directly; do not unwrap it.

</details>

## ex16

<details><summary>Hint 1</summary>

Apply ? to each parse result.

</details>

<details><summary>Hint 2</summary>

Wrap the final sum in Ok.

</details>

## ex17

<details><summary>Hint 1</summary>

copied changes an iterator of references into values.

</details>

<details><summary>Hint 2</summary>

Filter first, widen and square second, then collect.

</details>

## ex18

<details><summary>Hint 1</summary>

A trait implementation uses the method signature from the trait.

</details>

<details><summary>Hint 2</summary>

Borrow fields when formatting.

</details>

## ex19

<details><summary>Hint 1</summary>

The iterator yields references, so no cloning is needed.

</details>

<details><summary>Hint 2</summary>

Ord supports finding the minimum through min.

</details>

## ex20

<details><summary>Hint 1</summary>

split_once separates the ID from the name.

</details>

<details><summary>Hint 2</summary>

Validate delimiters before constructing the record; map_err can turn ParseIntError into String.

</details>


