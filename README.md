# rust-cas
A simple Computer Algebra System written in Rust (FLVS Coding Club Project)

## TODO
- [ ] Create a tokenizer to convert an expression to abstract types.
  - [x] Implement accurate tokenization for each type. 
  - [x] Add the ability to simplify subtraction as equal to either `[ADD, NUM(-1.0), MUL]` or `[NUM(-1.0), MUL]`
  - [X] Add the ability to split `(...(` into `[LGROUP...LGROUP]` and `)...)` into `[RGROUP...RGROUP]`
  - [ ] Add the ability to seperate values such as 32x into `[NUM(32.0), MUL, VAR("x")].
- [x] Create a `LGROUP/RGROUP` fixer.
  - [x] Add the ability to accurately find group beginning and end.
  - [x] Add the ability to add MUL values between `RGROUP` and `LGROUP` values if there isn't any value between them.
  - [x] Add the ability to simplify `[ADD, LGROUP, ...VALUES..., RGROUP]` as `[ADD, ...VALUES...]`
- [x] Create a binary tree creater.
  - [x] Add the ability to recursively create nodes.
  - [x] Add the ability to accurately split values into raw branches (left vs. right.)
  - [x] Add the ability to weight raw branches, and then make them left or right depending on weighting.
- [ ] Add the ability to simplify expressions.
  - [x] Add the ability to traverse the tree.
  - [ ] Add the ability to locate patterns and simplify them accordingly.
- [ ] Add the ability to solve expressions.
