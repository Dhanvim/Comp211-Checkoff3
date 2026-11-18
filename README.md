# Moving Ownership in Rust

COMP 211 AI exploration and presentation. Topic: what happens when ownership of a value moves from one variable to another, and why Rust prevents the original variable from being used afterward.

**Name:** Your Name  
**Video:** [add link or filename]

## Repository contents

| Path | What it is |
|---|---|
| `examples/working/` | Programs that compile and run: moves, `.clone()`, `Copy` types, functions and ownership, reassignment, drop order |
| `examples/errors/` | Programs that are *supposed* to fail to compile, each with the error code it triggers (E0382, E0507) |
| `run_all.sh` | Compiles everything and checks that working examples pass and error examples fail with the expected error |
| `slides/` | Presentation slides |
| `reflection.md` | Written AI exploration and reflection |
| `transcript.md` | Full AI conversation |

## Running the examples

Requires Rust (install with [rustup](https://rustup.rs)).

```bash
# one file, like gcc
rustc examples/working/01_move.rs -o move_demo && ./move_demo

# see a compile error
rustc examples/errors/e1_use_after_move.rs

# check everything at once (add -v to print full compiler errors)
bash run_all.sh
```

## Examples

### Working
| File | Idea |
|---|---|
| `01_move.rs` | `let s2 = s1;` moves ownership; only `s2` is usable |
| `02_clone.rs` | `.clone()` makes an independent heap copy |
| `03_copy_types.rs` | `i32` and `bool` are copied, not moved |
| `04_function_takes_ownership.rs` | Passing a `String` to a function moves it; returning it hands ownership back |
| `05_reassign_after_move.rs` | A moved-from variable is valid again once reassigned |
| `06_drop_order.rs` | Prints a message when each value is dropped (freed) |

### Errors
| File | Error | C bug it corresponds to |
|---|---|---|
| `e1_use_after_move.rs` | E0382 | Using a pointer after ownership changed |
| `e2_use_after_function_call.rs` | E0382 | Use-after-free |
| `e3_drop_then_use.rs` | E0382 | `free(p); printf("%s", p);` |
| `e4_move_in_loop.rs` | E0382 | `free(s)` on every iteration |
| `e5_move_in_one_branch.rs` | E0382 | Pointer freed on only some paths |
| `e6_move_out_of_vec.rs` | E0507 | Double free from an aliased buffer |
| `e7_partial_move_struct.rs` | E0382 | Using a struct after freeing one field |

## Sample compiler output (`e1_use_after_move.rs`)

Tested with rustc 1.75. Wording may differ slightly in newer versions.

```text
error[E0382]: borrow of moved value: `a`
 --> e1_use_after_move.rs:6:20
  |
4 |     let a = String::from("hello");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
5 |     let b = a;
  |             - value moved here
6 |     println!("{}", a);
  |                    ^ value borrowed here after move
```
