# Capstone: task-list CLI

Build a CLI that remembers tasks between invocations. Complete the milestones in order; later tests depend on earlier implementations. Each milestone takes roughly 20–45 minutes, though storage may take longer.

Public types and function signatures are supplied in `src/`. Implement their TODO bodies without changing the tests. String error wording is flexible except that the executable prefixes errors with `Error: ` and exits with status 1.

## 1. Model (depends on exercises 09–12)

Implement `TaskList::new() -> Self` and `all(&self) -> &[Task]` in model.rs. The supplied Task has public id, title, and done fields; TaskList keeps its Vec private. new returns an empty list, and all borrows its contents. Default delegates to new.

Run: `cargo test -p capstone --test milestone1`

## 2. Operations (depends on milestone 1 and exercises 13–16)

Implement `validate_title(&str) -> Result<(), String>`, `add(&mut self, String) -> Result<u32, String>`, and `complete(&mut self, u32) -> Result<(), String>`.

Reject blank/whitespace-only titles and any tab, CR, or LF. Preserve other whitespace and Unicode. Adding assigns max(existing ID) + 1, starting at 1; do not use list length. IDs never change when completing. Repeated completion succeeds. Unknown IDs fail. Check u32 overflow and leave the list unchanged on any error.

Run: `cargo test -p capstone --test milestone2`

## 3. Commands (depends on milestone 2 and exercises 11, 15–16)

Implement `commands::parse(&[String]) -> Result<Options, String>`. Input excludes the executable name. Accept exactly:

```text
task-lab [--file PATH] add "TITLE"
task-lab [--file PATH] list
task-lab [--file PATH] done ID
```

The optional flag must precede the command and occur once. Default file is tasks.tsv in the current directory. Reject empty paths, missing or extra arguments, unknown verbs, invalid titles, and IDs that fail standard u32 parsing or equal zero. A title is one argument; quote multiword titles in your shell. Return Command and PathBuf values without performing I/O.

Run: `cargo test -p capstone --test milestone3`

## 4. Storage (depends on milestones 1–3 and exercise 20)

Implement `TaskList::from_tasks(Vec<Task>) -> Result<Self, String>`, rejecting invalid titles, zero IDs, and duplicate IDs, while preserving order. Then implement the four supplied storage functions:
`decode(&str)`, `encode(&TaskList)`, `load(&Path)`, and `save(&Path, &TaskList)`.

File format: UTF-8, one task per line, three tab-separated fields: ID, status (0 pending / 1 done), title. Encode with LF and a final newline for each record. Empty list encodes as an empty file. Decode accepts LF or CRLF, and a final line without a newline; reject blank records, malformed fields and invalid task data. IDs use standard u32 parsing.

Missing files load as an empty list. Other read errors, including invalid UTF-8, must propagate. Create a uniquely named sibling temporary file with create_new, write and flush it, close it, then rename it over the destination. Clean up temporary files after errors. Do not truncate the destination before a successful write. Missing parent directories are errors. Concurrent writers are outside this learning project's scope.

Run: `cargo test -p capstone --test milestone4`

## 5. CLI (depends on all previous milestones)

Implement `run() -> Result<(), String>` in main.rs. Parse arguments, load storage, execute the command, save only successful mutations, and print success only after saving. The provided main handles errors.

Exact stdout (each ends with a newline):

- Add: `Added task 1.`
- Complete: `Completed task 1.`
- Empty list: `No tasks.`
- Each pending task: `1 [ ] Learn Rust`
- Each completed task: `1 [x] Learn Rust`

List tasks in stored order. Listing a missing file must not create it. Never overwrite malformed storage. Invalid commands, unknown IDs, or failed file operations must exit nonzero; successful commands exit zero.

Run: `cargo test -p capstone --test milestone5`, then `cargo test -p capstone`.

Try the completed application:

```text
cargo run -p capstone -- add "Learn Rust"
cargo run -p capstone -- list
cargo run -p capstone -- done 1
cargo run -p capstone -- --file practice.tsv list
```

## Progressive hints

<details><summary>Milestones 1–2: first hint</summary>

Use a Vec for storage. Borrow it as a slice. Search mutable tasks by ID when completing.

</details>

<details><summary>Milestones 1–2: second hint</summary>

Use iter().map(...).max().unwrap_or(0).checked_add(1) to allocate the next ID. Validate before pushing. Find a mutable task with iter_mut().find(...).

</details>

<details><summary>Milestone 3: first hint</summary>

Peel off the optional prefix, then match on the remaining slice's length and contents.

</details>

<details><summary>Milestone 3: second hint</summary>

Slice patterns such as [verb, title] make argument counts explicit. Reuse validate_title.

</details>

<details><summary>Milestone 4: first hint</summary>

Separate pure text parsing from file I/O. Validate parsed tasks through from_tasks.

</details>

<details><summary>Milestone 4: second hint</summary>

Use lines and split for decoding, HashSet for duplicate IDs, and ErrorKind::NotFound for the missing-file case. Scope the temporary-file handle so it closes before rename or cleanup, especially on Windows.

</details>

<details><summary>Milestone 5: first hint</summary>

Match the parsed Command. Load once, mutate, save, then print.

</details>

<details><summary>Milestone 5: second hint</summary>

Use env::args().skip(1) for arguments and ? to propagate errors to the provided main. The list branch does not call save.

</details>

Optional answers: [solutions](../solutions/capstone/). Compare only after trying the tests and hints.

