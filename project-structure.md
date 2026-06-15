loom/
├── README.md                          # what is this, how to get started
├── PLAN.md                            # seed document
├── ARCHITECTURE.md                    # the 3-layer model, decisions
├── SPEC.md                            # the Loom language specification
├── CHANGELOG.md
├── LICENSE                            # Apache-2.0 probably
├── Cargo.toml                         # Rust workspace
│
├── crates/
│   ├── loom-syntax/                   # parser, AST, lexer
│   │   ├── src/
│   │   │   ├── ast.rs                 # the AST types
│   │   │   ├── parser.rs              # parser entry point
│   │   │   ├── lexer.rs
│   │   │   └── grammar/               # if using chumsky/lalrpop
│   │   └── tests/
│   │
│   ├── loom-check/                    # static checking of claims
│   │   ├── src/
│   │   │   ├── types.rs               # type checking
│   │   │   ├── registers.rs           # cross-register coverage (knows→relates→shows→does→proves)
│   │   │   ├── anti_patterns.rs       # lint-style rules
│   │   │   └── diagnostics.rs
│   │   └── tests/
│   │
│   ├── loom-compile-dafny/            # claims → Dafny
│   │   ├── src/
│   │   │   ├── translate.rs           # AST → Dafny IR
│   │   │   ├── emit.rs                # Dafny IR → .dfy text
│   │   │   └── invoke.rs              # subprocess Dafny, parse results
│   │   └── tests/
│   │
│   ├── loom-compile-python/           # implementation → Python
│   │   ├── src/
│   │   │   ├── translate.rs
│   │   │   └── emit.rs
│   │   └── tests/
│   │
│   ├── loom-verify/                   # orchestrates verification
│   │   ├── src/
│   │   │   ├── pipeline.rs            # check → compile → verify → report
│   │   │   ├── gap_report.rs          # the bidirectional gap report
│   │   │   └── results.rs
│   │   └── tests/
│   │
│   ├── loom-cli/                      # the `loom` binary
│   │   └── src/
│   │       └── main.rs                # `loom check`, `loom build`, `loom verify`
│   │
│   ├── loom-llm/                      # LLM operations
│   │   ├── src/
│   │   │   ├── client.rs              # Anthropic API client
│   │   │   ├── distill.rs             # prose → umbrella
│   │   │   ├── generate.rs            # umbrella → sibling
│   │   │   └── summarize.rs           # sibling → parent claims
│   │   ├── prompts/
│   │   │   ├── distill.md
│   │   │   ├── generate.md
│   │   │   └── summarize.md
│   │   └── tests/
│   │
│   └── specq/                         # spec quality reporter (the companion paper)
│       ├── src/
│       │   ├── mutate/
│       │   │   ├── operators.rs       # the §6.2 mutation operators
│       │   │   └── direction.rs       # strengthening vs weakening
│       │   ├── domain.rs              # precondition saturation, example diversity
│       │   ├── coverage.rs            # cross-register coverage rules
│       │   └── report.rs              # quality report output
│       └── tests/
│
├── tree-sitter-loom/                  # editor support
│   ├── grammar.js
│   ├── queries/
│   │   ├── highlights.scm
│   │   └── locals.scm
│   └── package.json
│
├── examples/
│   ├── 01-hello-umbrella/             # smallest possible
│   │   ├── hello.lm
│   │   └── README.md
│   ├── 02-ledger/                     # the conservation example from the paper
│   │   ├── ledger.lm
│   │   └── README.md
│   ├── 03-todo-list/                  # a more practical case
│   │   ├── todos.lm
│   │   └── README.md
│   └── 04-bidirectional-demo/         # shows the gap report doing work
│       └── ...
│
├── docs/
│   ├── language-reference.md          # complete syntax reference
│   ├── claims-reference.md            # all claim forms with examples
│   ├── verification-internals.md      # how loom→Dafny works
│   ├── bidirectional-refinement.md    # the bidirectional discipline
│   ├── llm-operations.md              # distill/generate/summarize
│   ├── spec-quality.md                # using specq
│   └── adr/                           # architecture decision records
│       ├── 0001-rust-as-impl-language.md
│       ├── 0002-dafny-as-verifier.md
│       ├── 0003-python-as-target.md
│       └── 0004-no-actors-in-v0.md
│
├── .github/
│   └── workflows/
│       ├── ci.yml                     # build, test
│       ├── examples.yml               # verify all examples still work
│       └── docs.yml                   # publish docs site
│
└── tools/
    ├── install-dafny.sh               # bootstrap dafny on dev machine
    └── bench.sh                       # smoke/perf benchmarks