# Refactoring plan

Status: **approved, in progress**: Phase 1 committed (`dd539f0`); Phase 2 committed (`f4c16bf`); Phase 3 done; Phase 4 done; Phase 5 done; Phase 6 done; Phase 7 next (2026-10-31)

§1 describes the state before Phase 1.

This plan merges the local solutions in `~/Workspace/AdventOfCode` (the "old folder") into this repository (`me-like-coffee/advent-of-code`), fixes flaws in the shared code, and adds the 2022 C++ solutions under `cpp/`.

## 1. Current state

### This repository
| Area | Finding |
|---|---|
| Layout | One Rust crate (edition 2021). `src/year_YYYY/day_NN/solution.rs` exposes `solve() -> Result<(u64, u64)>`, registered through `register_days!`. A `clap` runner (`-y/-d/-a/-p`) and a `generate` scaffolding command are included. |
| Coverage | 53 days from 2015 to 2025, 9,680 lines, 49 commits. No puzzle inputs anywhere in the history. |
| `check` baseline | `cargo fmt` passes. `clippy -D warnings` (1.99) fails with 8 findings: `collapsible_match`, `unnecessary_sort_by`, `chunks_exact` with a constant size, and decimal constants. Tests: 102 pass and 9 fail, all because files under `inputs/` are missing. |
| `read_input` | Builds the path `inputs/…` relative to the working directory, so it fails when run from anywhere but the repo root. |
| `generate` | Builds paths with `format!("{}/…")`, which breaks the portability rule. It doesn't register the new day: it prints "ACTION REQUIRED" instead. |
| Year registry | Adding a year means editing three places: `pub(crate) mod` in `lib.rs`, the `match` in `get_year_solver`, and the hard-coded `2015..=2025` in `run_all`. |
| `day_to_u8` | A malformed module name, such as `day_1` or `dayXX`, gives a wrong number or an arithmetic overflow instead of a compile error. |
| Answer type | `SolveFn` returns `(u64, u64)`, but 6 local days have answers that aren't numbers: 2017/10 part 2 (a hex hash), 2017/16 (strings), 2018/10 (a message drawn as text), 2018/11 and 2018/13 (coordinates), 2021/24 (a digit string). |
| Dependencies | `anyhow` 1.0.104, `clap` 4.6.7, `nom` 8.0.0, `num` 0.4.3 and `rayon` 1.12.0 all resolve to the latest release, so nothing needs upgrading. But the manifest uses caret ranges and `Cargo.lock` is gitignored, so builds aren't reproducible. No `rust-toolchain.toml`. |
| Edition | 2021. A scratch-copy migration to 2024 (`cargo fix --edition`) changed 6 files: import ordering, one added `;`, and rustfmt line wrapping. Tests are unchanged (102 pass, 9 fail for missing inputs). Edition 2024 enables let-chains, which adds 27 `collapsible_if` clippy findings; `cargo clippy --fix` fixes all but the 2 decimal-constant findings. |
| Puzzle examples | 26 solution files contain puzzle examples as string constants in their tests. That conflicts with the decision to keep examples out of the repository (see §5). |

### The old folder
- 79 Rust crates and 17 C++ programs (2022), with no shared code. 7 crates have their own nested git history of 1–3 commits.
- Puzzle inputs are hard-coded in source in 2016/11, 2017/03, 14, 15, 17 and 23, 2018/11 and 22, 2020/25, and 2022/Day05 (C++).
- 16 days exist in both places: Rust 2019/09, 10, 14, 15, 16 and 17, 2021/08, 2022/16 and 19, 2023/10, 2024/01, 02, 03 and 22; plus 2022/21 and 22 in C++ locally and in Rust here.

### Already done (2026-10-30)
- Cloned to `~/Workspace/advent-of-code`.
- Deleted `GEMINI.md`, `src/GEMINI.md` and `src/year_2025/GEMINI.md`. Their still-relevant conventions are now in `CLAUDE.md`.
- Added `LICENSE` (Apache-2.0) and `CLAUDE.md`, and extended `.gitignore` with build, editor, OS and Claude Code entries.
- Deleted the old folder's build outputs (925 MB).

## 2. Target layout

```
advent-of-code/
├── Cargo.toml  Cargo.lock  rust-toolchain.toml
├── src/
│   ├── main.rs               # runner and `generate`
│   ├── lib.rs                # register_years!(year_2015, year_2016, …)
│   ├── utils/                # runner types, Answer, read_input, parser (+ unit tests)
│   ├── year_2017/knot_hash.rs     # shared by days 10 and 14 (+ unit tests)
│   ├── year_2018/device.rs        # shared by days 16, 19 (+ unit tests)
│   ├── year_2019/intcode.rs       # existing (+ unit tests)
│   └── year_YYYY/day_NN/{mod.rs, solution.rs}
├── CMakeLists.txt  CMakePresets.json  vcpkg.json
├── cpp/
│   ├── common/               # aoc::input (+ GoogleTest unit tests)
│   └── year_2022/day_NN/main.cpp
├── inputs/                   # gitignored: year_YYYY/day_NN.txt, day_NN_example*.txt
├── .github/workflows/ci.yml
└── .gitignore  .gitattributes  README.md  LICENSE  CLAUDE.md  TODO.md  REFACTORING.md
```

The Rust and C++ solutions read the same `inputs/` tree.

## 3. Phases

Each phase ends with `check` green and is one commit. I'll make the commits and pushes when you ask, after `check` passes.

### Phase 1: get `check` green on the current code (done 2026-10-31, committed as `dd539f0`)
- Migrate to edition 2024: `cargo fix --edition`, set `edition = "2024"`, then `cargo fmt`. This was validated in a scratch copy.
- Fix the clippy findings: `cargo clippy --fix` for the 33 automatic ones, plus 2 decimal constants by hand. Approving this plan counts as approval for that cleanup.
- Add a `puzzle-inputs` feature. Tests that read `inputs/` get `#[cfg_attr(not(feature = "puzzle-inputs"), ignore = "needs local puzzle files")]`. `check` runs them through `--all-features`; CI and fresh clones skip them.
- Commit `Cargo.lock` (remove it from `.gitignore`), pin dependencies to exact versions, and add `rust-toolchain.toml` (1.99.1) and `rust-version`.
- Add `.gitattributes` (`* text=auto eol=lf`) and `README.md`, covering usage, the input layout, and a note that inputs aren't included.
- Copy the inputs of the 16 days that exist in both places from the old folder to `inputs/`.
- Requires: inputs for the 5 days whose tests read files and have no copy locally (2019/13, 19, 21 and 22, and 2022/18). See §4.1. Done 2026-10-31: all 111 tests pass with `--all-features`, including the 9 that read inputs.

### Phase 2: shared-code fixes (with unit tests) (done 2026-10-31, committed as `f4c16bf`)
- **`Answer` type**: `enum Answer { Num(u64), Text(String) }` with `From` and `Display`. `SolveFn` becomes `fn() -> Result<(Answer, Answer)>`. Existing days change only their return line: `Ok((p1.into(), p2.into()))`.
- **`read_input`**: the inputs root comes from the `AOC_INPUTS` environment variable, or defaults to `<crate root>/inputs` (`env!("CARGO_MANIFEST_DIR")`). Paths are built with `Path::join`. Add `read_example(year, day, n)` for `day_NN_example{n}.txt`.
- **Year registry**: a `register_years!` macro generates the modules, `get_year_solver` and the year list that `run_all` iterates. Adding a year then takes one line. It takes module names (`register_years!(year_2015, …)`), like `register_days!`, because turning `2015` into `year_2015` would need a proc-macro crate.
- **`day_to_u8`**: checked at compile time, so a bad module name is a build error.
- **`generate`**: builds paths with `Path::join` and adds the day to `register_days!` (and the year to `register_years!`) itself.
- **Unit tests**: for everything above plus `utils::parser`. No new tests for individual days.
- **Follow-up (`359387c`)**: `cargo fmt` never reached the modules declared inside `register_days!` and `register_years!`. `rustfmt.toml` (edition 2024) now makes `check` run `rustfmt --check` on every file, and all sources were formatted once. That commit is listed in `.git-blame-ignore-revs`.

### Phase 3: move puzzle examples out of the source (see §5, question 1) (done 2026-10-31)
- Move the 26 inline examples to `inputs/year_YYYY/day_NN_example*.txt` and read them with `read_example`. Mark those tests `puzzle-inputs`.
- Also move the examples in old-folder `test*.txt` files there during the port.
- Done: the count of 26 was low. Examples were in 41 day files, including plain multi-line strings and strings passed straight to functions, and they became 78 example files (an example repeated in one day's tests shares a file). Each file holds the literal's exact value, including leading and trailing spaces and whether it ends with a newline. The 91 tests that read inputs or examples are skipped without `puzzle-inputs`; 49 still run.
- Kept in the source: 2019/22 `test_parsing` (a shuffle written for the test), 2019/15 `test_shortest_path_example` (a grid built in code), the 2019/21 springscript (the solution itself), and expected answers.
- 2023/12 `test_individual_rows` and 2025/03 `test_part_2_examples` copied example lines into the test; they now read them from example 1.
- `generate`: the day template reads example 1 instead of an inline constant, its stub has `#[expect(unused_variables)]`, and an empty `day_NN_example1.txt` is created next to the input. Its `register_days!` and `register_years!` output now matches rustfmt's 60-character `fn_call_width`, a Phase 2 bug that the rustfmt check exposed.

### Phase 4: port the old folder's Rust days (65 days, one year per commit) (done 2026-10-31)
- **Outcome**: 46 days ported, 19 held back for Phase 8. Commits: 2016 `652cd51`, 2017 `0d78417`, 2018 `9e75640`, 2019 `86d30d5`, 2020 `6c12f22`, 2021 `6ec943d`, 2022 `a91b2a3`, 2023 `6ae44f3`.
- **Held back**: 6 days appear adapted from third-party code (plus 2023/19 part 2), 11 compute no checked answer, and 2 were empty stubs. See Phase 8.
- **Dependencies**: none added. `itertools` (2016/11) and `chrono` (2018/04) were replaced with the standard library. `rand` (2023/25) wasn't needed, because that day is held back.
- **Shared code**: `year_2018/device.rs` (days 16 and 19). `knot_hash.rs` wasn't extracted, because 2017/14 is held back. No `utils/grid.rs`: the ported days' `Point` and `Direction` types differ.
- **Puzzle data moved out of source**: hard-coded inputs were rebuilt as input files (2016/11, 2017/03, 15 and 17, 2018/11 and 22, 2020/25), and 2017/21's starting pattern moved to `day_21_start.txt` (read with the new `utils::read_extra`).
- **Tests compile with `opt-level = 3`**, because 2016/11 takes minutes unoptimised.
| Year | Days |
|---|---|
| 2016 | 09, 11, 22 |
| 2017 | 03, 09, 10, 11, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24, 25 |
| 2018 | 03, 04, 05, 06, 09, 10, 11, 12, 13, 15, 16, 17, 18, 19, 22, 23, 24, 25 |
| 2019 | 03, 12 |
| 2020 | 07, 10, 25 |
| 2021 | 05, 06, 09, 10, 11, 12, 13, 14, 15, 16, 17, 21, 24, 25 |
| 2022 | 20, 23 |
| 2023 | 16, 17, 18, 19, 20, 21, 23, 25 |

For each day:
- Copy `input.txt` to `inputs/year_YYYY/day_NN.txt`, and the example files to `day_NN_example*.txt`.
- Replace hard-coded inputs with parsing of the input file.
- Wrap the existing logic in `solve()`. Change it only as far as the signature, errors and clippy require: `?` instead of `unwrap` outside tests, and no commented-out code.
- Keep each day's existing answer assertions as its tests.
- The 16 days already here keep this repository's version, and the old copies are dropped.
- Extract shared code only where the duplication is real: `year_2017/knot_hash.rs` (days 10 and 14) and `year_2018/device.rs` (days 16 and 19), each with unit tests. If `Point` or `Direction` is needed by more than one ported day, it goes into `utils/grid.rs` with tests.
- Dependencies: `regex` and `geo` were used only by days that are dropped (2024/03, 2023/10), and `itertools` only by 2016/11 among the ported days (the others are dropped). The ported days that need a crate are:
  - 2016/11: `itertools` (0.12.1, latest 0.15.0)
  - 2018/04: `chrono` (0.4.34, latest 0.4.45); std string sorting may be enough here
  - 2023/25: `rand` (0.8.5, latest 0.10.3; the API changed, for example `thread_rng` became `rng`)

  Each one is either replaced with std or added at its latest version, pinned exactly, after a maintenance check. Any randomized solution uses a fixed seed so its tests are deterministic.

### Phase 5: C++ 2022 (17 programs) (done 2026-10-31)
- **Outcome**: 15 programs ported, days 01–14 and 21, each printing exactly what the original printed (compared line by line against the originals built from the old folder). `check` now configures, builds and runs the C++ unit tests next to Cargo.
- **Held back**: day 15 (`main.cpp` is commented out from top to bottom; `main2.cpp` appears adapted from another C++ solution, see Phase 8) and day 22 (unfinished: part 1 only, and it didn't finish within 5 minutes). Both days are solved in Rust here.
- **`cpp/common`**: `aoc::input_path`, `aoc::example_path` and `aoc::open` (returning `std::expected`). `read_lines` wasn't needed, because the programs keep their own reading loops. 9 GoogleTest tests.
- **Corrections to the notes below**: days 13, 15 and 22 used `std::variant` and `std::optional`, so they were built as C++17, not C++14. Day 14 already had `#include <algorithm>`, commented out.
- **Day 05**: the starting stacks are parsed from the input. The local input had lost its drawing, so it was rebuilt from the hard-coded stacks; the output matches the original's.
- **Speed**: under the dev preset (Debug, sanitizers) day 14 takes over 10 minutes; built with `-O2` it takes about 100 seconds. `check` runs only the unit tests, not the days.
- **Unchanged**: the programs' debug output and commented-out code, so that the output comparison holds.
- Root `CMakeLists.txt` (CMake 4.x), `CMakePresets.json` "dev" preset (Ninja, C++23, `CMAKE_EXPORT_COMPILE_COMMANDS`, vcpkg toolchain, ASan and UBSan), and `vcpkg.json` with a pinned `builtin-baseline`. `check` finds the root CMake project and builds and tests it next to Cargo.
- `cpp/common/`: `aoc::input_path(year, day, argv)`. The path is `argv[1]` if given, otherwise `AOC_INPUTS`, otherwise the repo's `inputs/` (passed in as a compile definition). Plus `aoc::read_lines` returning `std::expected`. Unit tests use GoogleTest from vcpkg, registered with CTest.
- Each `2022/DayNN/main.cpp` moves to `cpp/year_2022/day_NN/main.cpp` and becomes executable `aoc_2022_day_NN`. Day 15's `main2.cpp` becomes `aoc_2022_day_15_part2`.
- Language level today: the VS Code tasks passed no `-std`, so Apple clang 17 built the code as **C++14** (`__cplusplus` 201402L). A syntax check with `-std=c++23` passes for 16 of the 18 files. Day12 (`std::for_each`) and Day14 (`std::count_if`) need an include fix, because they relied on headers that older libc++ pulled in indirectly.
- Day05: parse the starting stacks from the input instead of hard-coding them.
- 2022/21 and 22 exist in both languages. Both versions are kept.
- `.clang-format` and `.clang-tidy` are left for later, so that existing C++ isn't mass-reformatted.

### Phase 6: CI (done 2026-10-31)
- **Outcome**: all 7 jobs pass on `main` (`00ef96d`), with the expected counts on every system: 57 Rust tests passed and 218 skipped, 7 doc tests, and 9 of 9 C++ tests. On Windows, MSVC's AddressSanitizer works in CI, so no Windows-only preset was needed.
- **Fixes the first runs found**: `SIZE_T_MAX` in C++ days 04, 07 and 11 (defined only by Apple's headers; now `SIZE_MAX`), and `operation operation;` in C++ day 11, which GCC rejects (now `enum operation operation;`). CI builds with `ninja -k 0`, so one run reports every compile error.
- **Workflow**: `.github/workflows/ci.yml`, on every push and pull request and on demand. Formatting runs on Ubuntu; clippy, `cargo test` and the CMake build with `ctest` run on Ubuntu, macOS and Windows. Actions are pinned to commit hashes, permissions are read-only, and no secrets are used.
- **C++ setup in CI**: CMake 4.4.5 and Ninja from `lukka/get-cmake`; vcpkg fetched at the baseline commit, with its built packages cached; the MSVC developer environment and `CXX=cl` on Windows.
- **Expected results**: 57 Rust tests run and 218 are skipped (they need puzzle inputs), plus 7 doc tests; 9 C++ unit tests. The day programs are built, not run.
- **Portability fixes made for it**: `#include <climits>` in C++ days 01, 08, 12 and 14, which relied on libc++ including it indirectly; C++ day 21 uses `std::int64_t` instead of `long`, which is 32 bits on Windows (its output is unchanged); the MSVC sanitizer build drops `/RTC1`, which MSVC's AddressSanitizer rejects.
- `.github/workflows/ci.yml`: an ubuntu, macOS and Windows matrix running `cargo fmt --check`, `rustfmt --check` on every `.rs` file (`cargo fmt` skips macro-declared modules), `clippy -D warnings`, `cargo test` (without `puzzle-inputs`), and the CMake build plus `ctest`.
- Archiving or deleting the old folder moves to the end of Phase 8, which still needs it.

### Phase 7: shared parsing where the duplication is real
This is not a move to one parser for every day. As of 2026-10-31, 15 of the 53 days use `utils::parser` and 18 use nom directly, and none of the 79 old-folder crates uses nom. Rewriting about 100 working parsers would change no answers, and only the answer tests check them, which need local inputs that CI doesn't have. So this phase removes only the duplicated code.
- **Intcode programs**: `intcode::parse_program(&str) -> Result<Vec<i64>>` in `year_2019/intcode.rs`, with unit tests. It replaces the 8-line parsing block that 12 of the 2019 days copy.
- **Character grids**: a `utils::grid` function that turns the input into a grid of `char`, with unit tests, in `utils/grid.rs` if Phase 4 created it. It replaces the `lines().map(|l| l.chars().collect())` code in 8 days. Days whose versions differ, for example by trimming or skipping empty lines, keep that behaviour, or keep their own code.
- **Other patterns**: survey the ported days for parsing repeated in three or more of them, and extract only those.
- **`generate`**: the day template imports `utils::parser`, so new days start from the shared parsers.
- **Not included**: rewriting other days' parsing to nom.
- **Checks**: `check` (with `puzzle-inputs`) gives the same answers before and after. CI runs the unit tests of the new helpers.

### Phase 8: rewrite suspected third-party code and the held-back days
Phase 4 holds back days instead of porting them when they appear copied or adapted from someone else's published code, or when they compute no checked answer. The list of suspected third-party code, with locations and evidence, is in the local file `THIRD_PARTY_REVIEW.md` (excluded from git). As of 2026-10-31 it holds 2017/14 and 23, 2021/14, 17 and 24, 2022/20, and 2023/19 part 2.
- **For each suspected day you choose**: rewrite it from scratch without consulting the suspected source, credit it (if its license allows: for MIT, its copyright notice and license text), or drop it.
- **Held back as unfinished**: 2018/10, 12, 15 and 17; 2021/16 and 21; 2023/18, 20, 21, 23 and 25 (25 used `rand`). Each is written as a new solution or dropped. 2018/09 and 2023/17 were empty `cargo new` stubs.
- **Parts that return 0**: 2018/11, 18 and 19 part 2 (never finished in the original), and 2023/19 part 2 (third-party, see above).
- **C++ held back in Phase 5**: 2022/15 (third-party) and 2022/22 (unfinished). Both are solved in Rust, so they can also be dropped.
- **Answers drawn as letters**: 2018/10, 2019/11 and 2021/13 part 2. Tentatively (2026-10-31), letter recognition in `utils/` with unit tests, using only letter shapes from your own accepted answers, and failing with the drawing for an unknown letter. Revisit before Phase 8 starts.
- **Conventions**: as for Phase 4. A rewritten day's tests check the answers recorded in the old folder's tests; answers that were never recorded are checked against the puzzle website.
- **Order**: after Phase 7, so CI runs on the new code. One commit per year.
- **Wrap-up**: archive or delete the old folder and its nested `.git` histories, once nothing is left to rewrite. That's your call.

## 4. Manual tasks for you

### 4.1 Download the missing puzzle inputs (done 2026-10-31)
Each AoC account gets its own inputs, and the tests here check the answers for *your* inputs. 37 days in this repository had no input anywhere on this machine. Only you could download them, because it needs your AoC login. All are downloaded now, and every solved day runs with `cargo run --release -- -a`. The instructions below still apply to days added later.

**Account.** Use the AoC account you solved with (the one shown as "Emmanuel Schilling"). Your local 2024/02 input gives 402, the same answer this repo's test expects, so the old folder and this repo use the same account. A different login, such as GitHub instead of Google, gives different inputs and failing tests.

**Where files go.**
- Save each input as `~/Workspace/advent-of-code/inputs/year_YYYY/day_NN.txt`, with a two-digit day. For example, 2019 day 13 is `inputs/year_2019/day_13.txt`.
- `inputs/` is gitignored, so these files never leave your machine.
- Save the file exactly as served. Copy-pasting from the browser can lose the final newline or add trailing spaces, so use the browser's *Save As…* on the input page, or the `curl` command below.

**Which days.**
| Needed for | Days |
|---|---|
| Phase 1 (tests that read inputs), **done 2026-10-31** | 2019/13, 2019/19, 2019/21, 2019/22, 2022/18 |
| Running them with the runner (`-y`, `-a`), **done 2026-10-31** | 2015/01; 2018/07; 2019/02, 05, 07, 11, 18, 20, 23, 24, 25; 2020/13, 22; 2022/17, 24; 2023/12; 2024/04, 05, 06, 25; 2025/01–12 |

The other 16 days in this repo already had inputs in the old folder. They were copied into `inputs/` in Phase 1. The days ported in Phase 4 brought their inputs from the old folder too.

**How to download.**
- **In a browser:** log in at https://adventofcode.com, open `https://adventofcode.com/YYYY/day/D/input` (no leading zero in the URL), and use *Save As…* with the path above.
- **With `curl`:** in the browser's developer tools, copy the value of the `session` cookie for adventofcode.com and export it in your shell (don't save it to a file in the repo):
  ```sh
  export AOC_SESSION='<cookie value>'
  cd ~/Workspace/advent-of-code
  get() {  # usage: get 2019 13
    mkdir -p "inputs/year_$1"
    curl --fail -sS --cookie "session=$AOC_SESSION" \
      -A "github.com/me-like-coffee/advent-of-code" \
      -o "inputs/year_$1/day_$(printf %02d "$2").txt" \
      "https://adventofcode.com/$1/day/$2/input"
    sleep 2
  }
  get 2019 13; get 2019 19; get 2019 21; get 2019 22; get 2022 18
  ```
  AoC asks automated tools to identify themselves with a User-Agent (`-A`), to go slowly, and to download each input only once. The `sleep 2` and the one-time list above follow that. The session cookie works like a password, so don't commit it or paste it anywhere shared.

**Example files are not a manual task.** In Phase 3 I'll write the examples from the current test code and the old folder's `test*.txt` files to `inputs/year_YYYY/day_NN_example{n}.txt` myself.

**Verifying.** After downloading, run `check`, which includes the tests that read inputs. If one fails, its input is probably from a different account or was damaged in saving. Verified 2026-10-31: `check` passes, and every solved day runs.

### 4.2 Nothing else
`gh` is set up and the toolchains are installed.

## 5. Decisions
- 2026-10-30: examples are gitignored. The 26 inline examples move to `inputs/…_example{n}.txt` (Phase 3). The published history is not rewritten, so the examples stay visible in old commits.
- 2026-10-30: phase order 1 → 6 as written. Shared code changes before porting.
- 2026-10-31: no repo-wide move to one parser. Phase 7 extracts only parsing that's really duplicated, after CI is in place.
- 2026-10-31: suspected third-party code is held back during the port and dealt with in Phase 8, after the port, rather than immediately.

## 6. Platform divergence
All paths go through `Path::join` and `std::filesystem::path`, and the inputs location comes from `AOC_INPUTS` or a path relative to the crate root. CI on Linux, macOS and Windows checks this. The divergent paths, each also marked in the code or workflow:
- **`CMakeLists.txt`, sanitizer flags** (`PLATFORM: windows (MSVC)`): MSVC gets AddressSanitizer only, because it has no UndefinedBehaviorSanitizer, and `/RTC1` is removed from the Debug flags, because its AddressSanitizer rejects it. Clang and GCC get both sanitizers.
- **`.github/workflows/ci.yml`, Windows steps**: the MSVC developer environment, `CXX=cl` so that CMake picks MSVC, and `bootstrap-vcpkg.bat` instead of `bootstrap-vcpkg.sh`.
