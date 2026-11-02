# regex

Regular expressions as typed automata, written in [Spiral](https://github.com/i574n/spiral) and compiled to Rust. Every
engine answers the same question, and the answers are checked against each other.

| engine | module | what it computes |
| --- | --- | --- |
| Brzozowski derivatives | `src/derivative.spi` | one derivative per code point, similarity-normalized (ACI alternation, right-nested sequence) |
| Antimirov partial derivatives | `src/partial.spi` | sets of terms, with the support bound \|PD(r)\| <= alphabetic width + 1 |
| eager DFA | `src/dfa.spi` | states = derivatives over the pattern's code-point classes, a state cap that refuses |
| minimal DFA | `src/minimal.spi` | the Myhill-Nerode quotient by partition refinement |
| certificates | `src/certificate.spi` | a distinguishing word per pair of minimal states, replayed on the derivative matcher; equivalence by product exploration |
| agreement | `src/agreement.spi` | all engines on a seeded corpus (lib `keyed_rng`) and on each shortest accepted word with its one-symbol mutations |
| XSD patterns | `src/xsd.spi` | the XSD dialect (implicit anchoring, literal `^`/`$`, class subtraction, blocks) with typed refusals |

The syntax (`src/syntax.spi`) has intersection and complement, and counted repetition is one node (`[a-z]{1,1000}`
builds 1001 states, it is never unrolled). Nullability comes with typed evidence (`src/nullable.spi`).

```powershell
pwsh scripts/init.ps1    # links deps/spiral to a sibling spiral checkout, cloning it when missing
pwsh build.ps1           # the Spiral test cells, then bench/bench.spi -> Rust -> cargo -> bench/report.html
```

## Honest limitations

- "Certified" means checked per automaton by a second engine, not proved: there is no Lean checker yet.
- Benchmarks (`bench/report.html` on the page) are ratios inside one job against the minimal DFA, with the same arm
  run twice as an A/A control: a table whose A/A median leaves 0.95-1.05 prints VOID. The Rust `regex` crate runs on
  the same inputs as an oracle (counts must match) and as a second baseline. Wall time, not CPU time; no rebar runs
  yet, and no claim of speed.
- `bench/bench.rs` is about 4,200 generated lines, over the 1,000-line rule: it stays uncommitted until the bench is
  split into crates.
- No captures, lookaround or Unicode categories. The XSD front end (`src/xsd.spi`) knows six Unicode blocks and ASCII `\d\s\w`, and has not been checked against a real schema corpus yet.
- One maintainer, with AI agents writing most of the code.
