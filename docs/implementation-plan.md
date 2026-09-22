# Rust Practice Lab implementation plan

The user-approved design is a standard-library-only Cargo workspace using Rust
2024, with 20 isolated exercises, a completed example, and a five-step CLI capstone.
Learner files remain unfinished; hints and solutions stay separate.

1. Create independent exercise targets, instructions, starter signatures, tests,
   progressive hints and reference answers. Verify each compiles and fails at TODOs.
2. Create capstone model, commands, storage and entry point, five milestone test
   groups, and separate answers. Cover persistence, malformed data, IDs and I/O errors.
3. Add onboarding, curriculum checklist, troubleshooting and implementation summary.
   Format everything; test every answer using the learner tests in a temporary copy.

Review focus: Unicode strings, empty collections, numeric overflow, malformed
storage without data loss, and commands issued in separate processes.

No dependencies, custom exercise runner, automatic progress tracker or GUI.
