# Compositional correctness: preserving global properties across the umbrella tree

> **Status:** draft
> **Audience:** anyone who wants to know how far Loom's guarantees reach *across* modules, not just within one — and where they currently stop.
> **Companions:** [`bidirectional-refinement.md`](bidirectional-refinement.md) (the gap report), [`containment-not-solution.md`](containment-not-solution.md) (the reliability frame), [`language-reference.md`](language-reference.md) §4.6/§7 and [`claims-reference.md`](claims-reference.md) §7 (the `summarizes` register), [`verification-internals.md`](verification-internals.md) §6 (limitations).
> **Worked example:** [`examples/05-composition/`](../examples/05-composition/).

---

## 1. The concern this addresses

A recurring worry about LLM coding agents is that they *quietly break architecture*: each local edit looks plausible, the tests stay green, but a global property of the system silently degrades until it fails in production. Tests do not catch this because tests are local and example-based; nothing in the ordinary workflow forces the system's *cross-cutting* invariants to be re-established after every change.

This is the same problem Loom's premise names — "you still read the output and hope" ([`README.md`](../README.md)) — but raised one level. Loom's per-module verification is a strong answer to *within-module* correctness. The question this document answers is: **how well does Loom preserve correctness at the level of the whole system — the "global" aspect, the architecture?**

The honest answer is that "architecture" splits into three distinct concerns, and Loom scores very differently on each:

| Concern | What it means | Loom's coverage |
|---|---|---|
| **Within-module** | a property of one umbrella's operations | **Strong** — this is what the umbrella + gap report are for |
| **Cross-module / compositional** | a system property that *follows from* what several modules prove | **Designed for, not yet mechanized** (the `summarizes` register; post-v0) |
| **Structural** | constraints on the shape of the system itself — layering, dependency direction, "every operation that touches X also does Y" | **Largely unexpressible** in v0's claim language |

The rest of this document makes each row precise, works a concrete two-umbrella example, sketches how cross-module discharge would be mechanized, and states plainly what remains a gap.

---

## 2. What Loom already provides

### 2.1 Within a module

For a single umbrella, Loom does exactly what example-based testing cannot: it pins invariants in a small, named, human-readable artifact and makes any erosion *loud*. The `proves` register states universal properties; the verifier discharges them; the **gap report** ([`bidirectional-refinement.md`](bidirectional-refinement.md)) reports anything it cannot establish as a first-class, visible gap rather than a silent absence. Two mechanisms target the *quiet* aspect specifically:

- **Uncovered ground is output, not silence.** A property the verifier cannot establish is a category-(B) entry, not a blank.
- **Erosion over time is mechanical.** `bidirectional-refinement.md` §7(ii) tracks category-(A) shrinking / category-(B) growing across revisions — a direct signal that an agent's edits are degrading guarantees.

The standing limitation, even here: protection is bounded by **claim coverage**. A property no umbrella ever claimed can still be broken quietly; `specq` defends the *strength* of the claims that exist, but neither tool invents an invariant nobody wrote down.

### 2.2 The umbrella tree and the verifiability gradient

Loom's architecture is explicitly hierarchical. From the architecture paper (§4.5, "filesystem as architecture"):

> Recursive composition is natural: umbrellas can have umbrellas, with a *verifiability gradient* — claims become more formal as one descends the tree and more prose-like as one ascends. The top of the tree might assert qualitative properties that the compiler cannot directly verify; the immediate-children umbrellas decompose those into more specific claims that lower-level modules prove mechanically. Bidirectional refinement bridges the gradient: lower-level proofs are summarized upward and matched against higher-level claims, with gaps reported.

This is, on paper, a clean answer to the "global architecture" concern: the system-level invariant lives at the top of the tree, is decomposed downward into checkable child claims, and the children's proofs are composed back upward, with gaps surfaced at each boundary. The filesystem geometry *is* the architecture — sibling proximity encodes coupling, tree distance encodes loose coupling — and it is inspectable by browsing.

### 2.3 The `summarizes` register

The surface mechanism for the upward composition is the `summarizes` register ([`language-reference.md`](language-reference.md) §4.6, [`claims-reference.md`](claims-reference.md) §7). A parent umbrella states what it guarantees *in terms of* what its children prove:

```loom
summarizes {
  customer_funds_safe:
    requires { ledger.conservation, ledger.no_overdrafts, audit.tamper_evident }
    means { for-all customer, balance(customer) reflects all completed transfers }
}
```

The intended semantics, quoting `claims-reference.md` §7:

> The verifier checks that the `means` clause follows from the conjunction of `requires` claims, treating child claims as assumed.

That sentence is the whole compositional discipline in one line — and §3 unpacks exactly what "treating child claims as assumed" must mean for it to be sound.

---

## 3. The compositional discipline, precisely

Bidirectional refinement already defines the discipline within a boundary: *obligations flow down, evidence flows up, the difference is the gap report.* Composition applies the same shape *across* a boundary, with one new rule and one new soundness condition.

**The discharge rule.** A parent claim `P.means` is discharged when it is a logical consequence of the conjunction of the child claims named in `P.requires`, with those child claims taken as **assumptions** — the parent does *not* re-prove them. This is what makes composition tractable: a system with hundreds of leaf proofs is verified once at the leaves, and each parent only proves the (usually small) implication from its children's guarantees to its own.

**The soundness condition.** Assuming a child claim is sound **only if that child claim is itself category-(A) in the child's own gap report.** If `audit.records_the_entry` is unproved in `audit`, then any parent that assumes it is building on sand. Therefore:

> A cross-umbrella discharge is no stronger than the weakest child claim it depends on. A category-(B) child claim **propagates upward**: the parent claim that requires it degrades to category-(B), with the child's gap as its reason.

This is the load-bearing rule, and it is where [`containment-not-solution.md`](containment-not-solution.md) becomes directly relevant. §3 of that essay insists a trust root must be *external* — "the worker cannot game the checker." The upward summary is produced, in the LLM workflow, by the `loom summarize` operation, which is **worker-mediated**. So the discipline must be explicit:

- `summarize` may *draft* the parent's `summarizes` register (propose the `requires`/`means`).
- The **verifier**, not the summary, must discharge `means` from the assumed child claims.
- Each assumed child claim must independently be category-(A).

If a global guarantee's only link from children to parent is an LLM summary a human signs off on, that is precisely the essay's "independence is illusory" failure (§5, competence mode). Mechanized discharge plus the category-(A) gate is what keeps the global story from collapsing back into "trust the worker."

---

## 4. A worked two-umbrella example

The files are in [`examples/05-composition/`](../examples/05-composition/). The domain is the canonical one from the architecture paper: money and an audit log.

### 4.1 Child A — `ledger.lm`

A money-moving core. The two claims the parent will lean on:

```loom
proves {
  conservation:
    for-all from: Account, to: Account, amount: PositiveAmount,
      from.open and to.open and from.balance >= amount =>
        let (from', to') = transfer(from, to, amount);
        from.balance + to.balance = from'.balance + to'.balance

  no_overdrafts:
    for-all from: Account, to: Account, amount: PositiveAmount,
      from.open and to.open and from.balance >= amount =>
        let (from', _) = transfer(from, to, amount);
        from'.balance >= 0
}
```

Both auto-discharge: `conservation` is `(b - a) + (c + a) = b + c`; `no_overdrafts` follows from the precondition `from.balance >= amount`. **Category-(A), 2/2.**

### 4.2 Child B — `audit.lm`

An append-only log. `Log :: List<Entry>`; `record` appends one entry. Three claims:

```loom
proves {
  append_only:
    for-all log: Log, e: Entry, x: Entry,
      log.contains(x) => record(log, e).contains(x)

  records_the_entry:
    for-all log: Log, e: Entry,
      record(log, e).contains(e)

  length_grows_by_one:
    for-all log: Log, e: Entry,
      record(log, e).length = log.length + 1
}
```

All three are facts about sequence append (`x in s ==> x in s+[e]`, `e in s+[e]`, `|s+[e]| = |s|+1`), which Z3 discharges from the sequence theory without hints. **Category-(A), 3/3.**

### 4.3 Parent — `bank.lm`

One system operation = one ledger transfer + one audit record:

```loom
import ledger from "./ledger.lm"
import audit from "./audit.lm"

module bank {
  knows {
    System :: { from: ledger.Account, to: ledger.Account, log: audit.Log }
  }

  relates {
    execute_transfer(s: System, amount: ledger.PositiveAmount) -> System
      requires { s.from.open, s.to.open, s.from.balance >= amount }
      ensures {
        let (from', to') = ledger.transfer(s.from, s.to, amount);
        result.from = from',
        result.to = to',
        result.log = audit.record(s.log, {from: s.from.id, to: s.to.id, amount: amount}),
      }
  }

  // ... does { ... } binds the body to ledger.transfer + audit.record ...

  summarizes {
    // (A) follows from the children, children assumed
    funds_conserved_and_audited:
      requires { ledger.conservation, audit.records_the_entry, audit.length_grows_by_one }
      means {
        for-all s: System, amount: ledger.PositiveAmount when
          s.from.open and s.to.open and s.from.balance >= amount,
          let s' = execute_transfer(s, amount);
          (s.from.balance + s.to.balance = s'.from.balance + s'.to.balance)
          and (s'.log.length = s.log.length + 1)
          and s'.log.contains({from: s.from.id, to: s.to.id, amount: amount})
      }

    // (B) the architectural invariant — a known gap
    gap audit_is_complete: when single_operation
  }
}
```

### 4.4 The composition that discharges

`funds_conserved_and_audited` is a system-level claim that the parent does **not** re-derive from first principles. Its `means` follows immediately once the three child claims are in scope:

- the balance equation **is** `ledger.conservation`, applied to the accounts `execute_transfer` feeds into `ledger.transfer`;
- `s'.log.contains(...)` **is** `audit.records_the_entry`;
- `s'.log.length = s.log.length + 1` **is** `audit.length_grows_by_one`.

The parent proves only the trivial glue: that `execute_transfer` routes its arguments into those two child operations unchanged. That is the entire value of composition — the leaves did the hard work; the parent assembles it.

### 4.5 The cross-umbrella gap report

```
Cross-Umbrella Gap Report for examples/05-composition/bank.lm
Verified against Dafny 4.4.0 with Z3 4.13
Children assumed:  ledger (A: 2/2)   audit (A: 3/3)

Summary:
  Claimed and proved (A):     2     system_conserves_money, funds_conserved_and_audited
  Claimed and unproved (B):   1     audit_is_complete
  Proved but not claimed (C): 0

(A) funds_conserved_and_audited
    requires: ledger.conservation, audit.records_the_entry, audit.length_grows_by_one
    All three child claims are category-(A) in their own umbrellas.
    means discharged assuming the child claims.      SMT 0.3s

(B) audit_is_complete
    Status: explicit gap  (gap audit_is_complete: when single_operation)
    Reason: verifier limitation — no trace/history model in v0
            (verification-internals §6.3). The property quantifies over the
            sequence of ALL operations ever applied to a System; v0 models
            single pure operations only.
    This is the architectural invariant most relevant to silent ledger/audit
    drift. Tracked for post-v0 (trace model).
```

The header line is the soundness condition made visible: the (A) result is annotated with the child status it rests on. Were `audit` to report, say, `records_the_entry` as (B), this report would show `funds_conserved_and_audited` degraded to (B) with reason "child claim audit.records_the_entry unproved" — gap propagation, surfaced rather than hidden.

### 4.6 The gap that does *not* discharge — and why it matters most

`audit_is_complete` is the property you actually want against a quietly-breaking agent: *the ledger and the audit log can never drift apart — every transfer that ever happened has a matching entry.* It is exactly the kind of cross-cutting invariant the video worries about. And v0 cannot establish it, for a reason worth stating precisely:

- It is a property of the **trace** of operations — the whole history of calls to the system — not of a single pure `execute_transfer`. Loom v0 has no mutable state and no history model ([`verification-internals.md`](verification-internals.md) §6.3; the live runtime view is deferred per `PLAN.md`). The strongest thing v0 can say is the *single-operation* shadow of it, which is already covered by `funds_conserved_and_audited`.
- More deeply: even setting traces aside, completeness is a claim about **every operation in the system** — including sibling operations not yet written. Loom has no construct that quantifies over the set of all operations across the tree. A future `payout` operation could update a balance without calling `audit.record`, and no claim in this umbrella would fail. (See §6.2.)

So `audit_is_complete` is admitted as an explicit `gap`, restricted to `single_operation`. **This is the design working as intended, not failing.** The architectural risk is not eliminated — it is *relocated* into a named, visible artifact at the top of the tree, where a reviewer is forced to see it, rather than diffused silently across the codebase. That relocation is the actual deliverable (§7).

---

## 5. How cross-umbrella discharge would be mechanized (Dafny)

Mechanizing §3's discharge rule on the Dafny backend is small and standard — it reuses the `proves`-to-lemma encoding already in [`verification-internals.md`](verification-internals.md) §2.3.

**Child claims become assumed lemmas.** Each child `proves`/usable claim the parent depends on is emitted into the parent's translation as a lemma *declaration whose body is not re-checked here* — Dafny's `{:axiom}` attribute:

```dafny
// Imported from the children — VERIFIED THERE, assumed here.
lemma {:axiom} ledger_conservation(from: Account, to: Account, amount: PositiveAmount)
  requires from.open && to.open && from.balance >= amount
  ensures var r := ledger_transfer(from, to, amount);
          from.balance + to.balance == r._0.balance + r._1.balance

lemma {:axiom} audit_records_the_entry(log: seq<Entry>, e: Entry)
  ensures e in audit_record(log, e)

lemma {:axiom} audit_length_grows_by_one(log: seq<Entry>, e: Entry)
  ensures |audit_record(log, e)| == |log| + 1
```

**The parent `means` becomes a lemma that invokes them.** Calling an assumed lemma brings its `ensures` into the SMT context; Z3 then discharges the parent's `ensures` from those facts plus `execute_transfer`'s body:

```dafny
lemma funds_conserved_and_audited(s: System, amount: PositiveAmount)
  requires s.from.open && s.to.open && s.from.balance >= amount
  ensures
    var s' := execute_transfer(s, amount);
    s.from.balance + s.to.balance == s'.from.balance + s'.to.balance
    && |s'.log| == |s.log| + 1
    && (Entry(s.from.id, s.to.id, amount) in s'.log)
{
  ledger_conservation(s.from, s.to, amount);
  var e := Entry(s.from.id, s.to.id, amount);
  audit_records_the_entry(s.log, e);
  audit_length_grows_by_one(s.log, e);
  // Z3 discharges the parent ensures from the assumed facts.
}
```

**The `{:axiom}` is the trust root, and it must be gated.** `{:axiom}` tells Dafny "trust this, don't check it." That is sound **only** when the lemma was independently verified *without* `{:axiom}` during the child's own compilation. So the translator must emit an assumed child lemma only after confirming the corresponding child claim is category-(A) in the child's gap report; otherwise it emits no axiom and the parent claim is reported (B) by propagation (§3). The source map ([`verification-internals.md`](verification-internals.md) §4.2) extends naturally: a parent `means` failure maps back to the parent's `summarizes` entry, and a propagation failure carries a pointer to the offending child claim.

What this does **not** require: any change to the AST, the source-map structure, or the orchestrator. It is a new emission mode in `loom-compile-dafny` plus a cross-umbrella step in `loom-verify` that orders children before parents (a topological sort of the import graph) and threads each child's gap-report status into the parent's translation.

**The acyclicity assumption.** This works for a tree (or DAG) of umbrellas, verified leaves-first. Mutually-recursive umbrella claims (A assumes B while B assumes A) are out of scope and should be a `loom check` error, not silently admitted — circular assumption is unsound.

---

## 6. What is genuinely not covered

### 6.1 Cross-umbrella discharge is post-v0

The mechanism in §5 is **not in v0.** This is stated in [`bidirectional-refinement.md`](bidirectional-refinement.md) §9 ("Cross-umbrella gap reports … Computing effective guarantees through `summarizes` relations is a future direction") and in [`language-reference.md`](language-reference.md) §7.3. In v0:

- each umbrella verifies **independently** against its own `does`;
- `loom summarize` can draft a parent's `summarizes` register, but nothing **discharges** `means` from the children;
- the consequence is sharp: **in v0 every module can verify green while the system-level property is only LLM-summarized, not proved.** That is precisely the failure shape the concern describes — except that, even unmechanized, a `summarizes` entry gives the global claim a *named home and a visible "(B) not yet discharged" status*, instead of nowhere.

This is the single highest-leverage gap to close relative to the concern, because the concern is *inherently* cross-cutting. §8 proposes it as the next scope increment.

### 6.2 Structural architecture has no claim form

The five registers (`knows`, `relates`, `shows`, `does`, `proves`) are behavioral and data-oriented. They constrain *what operations compute*. They do not constrain the *shape of the system*:

- "the payments module may never import the UI module";
- "there are no cycles in the dependency graph";
- "**every** operation that mutates a balance also appends an audit entry" — a universal over the *set of operations*, which is what `audit_is_complete` really needs.

The architecture paper encodes structure *implicitly* via the filesystem (sibling/tree geometry, `uses` blocks), but there is no register whose claims an agent's edit could *violate and thereby fail verification*. An agent can satisfy every behavioral claim in every umbrella and still route a call across a forbidden boundary, or add a balance-mutating operation that skips the audit, and nothing fails. In the common practitioner sense of the phrase, this is the part of "agents quietly break architecture" that Loom does not yet touch. It is flagged as a candidate ADR in §8; the design space (an `enforces`/architecture register? import-graph constraints checked by `loom check`? capability tracking, already sketched post-v0?) is open.

### 6.3 Quiet erosion across revisions: partial

Within a module, category-(A)-shrink tracking (§2.1) is a real erosion signal. Across modules, the equivalent — tracking whether a parent's effective guarantee weakened because a child claim moved A→B between revisions — depends on the cross-umbrella report of §6.1 and is therefore also post-v0. `bidirectional-refinement.md` §9 lists "gap-rate trending" and "differential gap reports" as the relevant future items.

---

## 7. The reframe: containment, not solution

It would be a mistake to read §6 as "Loom fails at global correctness." [`containment-not-solution.md`](containment-not-solution.md) argues the goal is not to *solve* (prevent every architectural break) but to *contain* — to make residual risk **locatable**: concentrated in small named artifacts, explicitly reviewed, rather than diffuse and silent.

By that standard the design is sound even where mechanization is incomplete. `audit_is_complete` is not proved, but it is not *invisible* either: it has a name, a home at the top of the tree, an explicit `gap` marker, and a stated reason. A reviewer is forced to confront "we do not yet guarantee ledger/audit non-drift across traces" — which is exactly the question that, in an ordinary codebase, no one is forced to ask until an incident.

The useful test from that essay (§7) applies directly: *after this system is in place, where would the next failure most likely come from, and would I see it before it caused damage?* For the example: the next failure comes from a new balance-mutating operation that skips the audit (§6.2). Would Loom-as-designed see it? Today, **no** — there is no structural claim to violate. That honest "no" is the most valuable output of this analysis, and it names the work in §8.

---

## 8. Proposed next steps

In rough priority order, relative to the concern:

1. **Mechanize `summarizes` discharge for the acyclic case** (§5). This is the bounded, high-leverage step: it turns "designed-for" into "demonstrated" on the cross-module axis, and it is mostly a new emission mode plus a leaves-first verification order. Candidate ADR; depends on nothing in v0 changing.
2. **Promote [`examples/05-composition/`](../examples/05-composition/) to a verifying example** once (1) lands — the first end-to-end demonstration that a parent invariant is *proved* from child claims, with gap propagation exercised.
3. **Open an ADR on structural/architectural constraints** (§6.2): is there an `enforces` register, an import-graph check in `loom check`, or does capability tracking (already a post-v0 item) subsume it? This is the genuinely unscoped gap and deserves a design discussion, not a quick patch.
4. **Cross-umbrella gap trending** (§6.3) — after (1), track A→B movement of child claims as a system-level erosion signal.

---

## 9. References

- [`bidirectional-refinement.md`](bidirectional-refinement.md) — the gap report; §9 lists cross-umbrella composition as future work.
- [`containment-not-solution.md`](containment-not-solution.md) — trust roots, independence, locatability.
- [`language-reference.md`](language-reference.md) §4.6, §7 — the `summarizes` register and module composition.
- [`claims-reference.md`](claims-reference.md) §7 — the meaning of `summarizes`.
- [`verification-internals.md`](verification-internals.md) §2.3 (lemma encoding), §4.2 (source map), §6.3 (no mutable state).
- [`docs/research/verifiable-umbrella-paper-v2.md`](research/verifiable-umbrella-paper-v2.md) §4.5 — the umbrella tree and verifiability gradient.
- [`examples/05-composition/`](../examples/05-composition/) — the worked example.
