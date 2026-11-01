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
| agreement | `src/agreement.spi` | all engines on a seeded corpus (lib `keyed_rng`) |

The syntax (`src/syntax.spi`) has intersection and complement, and counted repetition is one node (`[a-z]{1,1000}`
builds 1001 states, it is never unrolled). Nullability comes with typed evidence (`src/nullable.spi`).

```powershell
pwsh scripts/init.ps1    # links deps/spiral to a sibling spiral checkout, cloning it when missing
pwsh build.ps1           # bench/bench.spi -> Rust, cargo build, bench/report.html, then the Spiral test cells
```

## Honest limitations

- "Certified" means checked per automaton by a second engine, not proved: there is no Lean checker yet.
- Benchmarks are ratios inside one CI job against an A/A arm; there is no comparison with the Rust `regex` crate or
  rebar yet, and no claim of speed.
- No captures, lookaround or Unicode categories. The XSD pattern front end is not written yet.
- One maintainer, with AI agents writing most of the code.
