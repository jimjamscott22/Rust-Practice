# Optional reference answers — spoilers

Try each exercise, then hints, before opening an answer. Compare reasoning, not just output. Each answer uses the same interface and tests as the learner starter.

- ex01: Use string formatting. The format! macro returns an owned String.
- ex02: Use floating-point literals. Multiply by 9.0 / 5.0 before adding 32.0.
- ex03: A match guard adds a condition. Handle zero, then a negative guard, then the remaining case.
- ex04: Start an accumulator at zero. Convert each value with u64::from before adding.
- ex05: The parameter can be mutable even though ownership is moved. Use push with a char, then return text. Ownership permits reusing the input allocation.
- ex06: str::len counts bytes. Iterate with chars, then count.
- ex07: You can mutate the borrowed String without returning it. Use push_str for a string slice.
- ex08: A lifetime connects the returned reference to the inputs. Use the same lifetime on both inputs and the output; >= selects the first on ties. No owned copy is needed because the output lifetime is tied to both inputs.
- ex09: Construct the struct with named fields. Field shorthand lets name mean name: name.
- ex10: Use self to access fields. Convert dimensions before multiplying.
- ex11: Match each variant and bind its payload. Each match arm returns a u64.
- ex12: Return early when a match is found. After the loop, return None.
- ex13: Create a new Vec. Only push a value when contains returns false.
- ex14: HashMap::entry combines lookup and insertion. or_insert(0) returns a mutable reference to the counter.
- ex15: parse infers the target type from the return signature. Return the Result directly; do not unwrap it.
- ex16: Apply ? to each parse result. Wrap the final sum in Ok.
- ex17: copied changes an iterator of references into values. Filter first, widen and square second, then collect.
- ex18: A trait implementation uses the method signature from the trait. Borrow fields when formatting.
- ex19: The iterator yields references, so no cloning is needed. Ord supports finding the minimum through min.
- ex20: split_once separates the ID from the name. Validate delimiters before constructing the record; map_err can turn ParseIntError into String.

Capstone answers live in capstone/. They separate pure model and command behavior from filesystem effects. The CLI loads before modifying, validates before saving, and reports errors through Result. The storage writer creates a sibling temporary file before replacing the data file, preserving the original on errors.

