# loom-completeness PoC — does visible-gap verification catch honest blind spots that tests miss?

> **Status:** design — not yet built, not yet scaffolded as an epic. The successor inquiry to E-0001/E-0002/E-0003.
> **Relationship to the ladder:** a second **PoC rung** alongside [`loom-ultralight.md`](loom-ultralight.md), *before* loom-light is built (see [`loom-light.md`](loom-light.md) §1). It builds **none of the loom-light engine** — just Dafny plus the existing Rust harness, re-pointed.
> **Relationship to the prior experiments:** loom-ultralight / E-0002 / E-0003 asked *"does the incentive make an LLM **weaken** the spec?"* — the **gaming** hypothesis. Three subjects under pre-registration returned **NO-GO** (`D-0002`, `D-0005`); the one positive (`D-0001`, canonicalize) was post-hoc and did not replicate. This PoC pursues the branch loom-ultralight §5 named — *"differentiator weaker than hoped; reconsider before building"* — by testing loom's actual top-line claim instead of the gaming differentiator.
> **Prior art:** the verify-against-{correct,buggy}-impl machinery is the same mutation-kill mechanism as MutDafny / IronSpec (`loom-light.md` §9). What is novel here is the **honest-completeness** framing — measuring spec *omission* under honest authorship against a realistic test baseline, not spec *weakening* under an adversary.

---

## 0. What this tests

**The claim under test (loom's real top line, from the README):** *a way to mechanically check that the code does what was claimed, and to surface the parts that were not checked rather than quietly absorb them* — and that this **helps a human ship more-trustworthy code even when nobody is gaming.**

**The reframe.** The gaming studies measured spec *weakness* under an adversary. The value question is about spec *completeness* under **honest** authorship, and the failure mode is not weakening but **omission**: an honest human+LLM team writes the umbrella, and the one claim that would have caught the bug never gets written — because author and model **share the blind spot**. Loom's gap report exists precisely to drag that blind spot into the light (category **(B) claimed-but-unproved** surfaces the honest bug; the **absence** of any (B) touching the bug is the omission; category **(C) proved-but-unclaimed** surfaces under-claiming). The claim maps onto loom's own three categories (`bidirectional-refinement.md` §2), which makes it mechanically testable.

**Hypothesis.** On realistically-subtle, *honest-mistake* bugs, an honestly-authored umbrella verified against the implementation makes the defect **visible** (a category-(B) failure) at a materially higher rate than a thorough, honestly-authored **test suite** does — *and* honest authors omit the load-bearing claim rarely enough that the umbrella is complete enough to be worth reading.

If loom's margin is real **and** the omission rate is low → the value story has legs; design the tractability and human-factors rungs. If the test arms catch everything loom catches, **or** honest authors systematically omit the catching claim → the value story is weak too, learned cheaply before building loom-light.

This is **not** a re-proof of "formal verification catches edge cases tests miss" (textbook; the easy misleading win). The loom-specific, genuinely-uncertain part is **whether an honestly-authored umbrella is complete enough that the defect lands in category (B) at all.**

### 0.1 The load-bearing design problem: the capability ceiling

On E-0003, opus wrote **30/30 complete specs in both arms** — the omission rate was ≈0 because the toy invariants were too easy to omit anything on. If that repeats, this PoC measures nothing: honest authors look perfectly complete in a regime where completeness is trivial. The whole design turns on defeating this, via a distinction that is easy to blur:

- **Z3 *difficulty*** (deep proofs, large state) → the tractability wall → **avoid** (it is the separate §9 problem).
- **Obligation *breadth*** (many independent claims, several **not obvious from the prose**) → elicits honest omission → **seek**.

The lever that makes omission appear *honestly* is **realistically incomplete prose intent.** Real intent never spells out every obligation; the umbrella's job is to *complete* it. So a subject must pair under-specified prose with a rich gold obligation set containing several properties an author must **infer**, not read off. The omission rate then measures which non-obvious obligations honest authors fail to infer — which is exactly loom's claimed value: catching the property the prose did not state and nobody thought to claim. (reallocate's prose made refs salient enough that both arms claimed them; these subjects must not.)

Whether the decidable regime can be made broad-and-incomplete enough to elicit omission *at all* is itself uncertain — so it is settled empirically, first, by the pilot (§3).

## 1. The burden split — and why the author cannot be faked

| | |
|---|---|
| **Hand-built, no LLM** (the stimulus + the trust root) | the subjects, the reference implementations, the **bug bank**, the **gold spec** oracle, the harness, and the committed **detection fixtures** (umbrella-with-a-known-hole + buggy impl → expect survived-bug) that unit-test the instrument offline |
| **Real author, LLM-as-proxy** (the thing under measurement) | the **umbrella** (arm L) and the two **test suites** (arms T1, T2), each authored from the **prose intent only**, blinded to the impl and the bug |
| **Automated** (the harness) | verify each umbrella / run each suite against {correct impl, buggy impl}; classify caught / missed / omitted / invalid; compute per-arm detection rates, the omission rate, and the marginal Venn; print the tables |
| **You** (the human) | install the toolchain, set the API key, run it once, read the tables |

**Why the authoring cannot be mocked.** The load-bearing question is *what an honest author omits*. If the umbrellas (or test suites) are hand-written, their completeness is whatever the author chose — and a hand-author who knows where the bug is specs straight to it. Mocking the author writes the answer you want and assumes away the disease loom treats: *if a human could reliably hand-author complete specs, there would be no loom.* This is the loom-ultralight burden split (§1) — the LLM authors; a small external check validates — load-bearing here too.

**Where mocking IS correct.** Everything except the measured run. Build and **fully validate the instrument offline** on the hand-built detection fixtures + recorded generations; only the single recorded authoring sample touches the live API; re-score forever with `--rescore` (no API). The detection fixtures prove the *machinery* detects omission/catching correctly; they do **not** answer "do real authors omit" — only the real sample does. Keep the two roles separate.

## 2. The subjects — designed for omission, not difficulty

Subjects follow a **recipe**, not an ad-hoc pick:

- **Decidable and per-clause easy** — every obligation is inside Z3's decidable regime so the verifier stays clean and the gap report is meaningful (category (A) dense; `bidirectional-refinement.md` §6). The PoC does **not** test tractability (§9).
- **Broad** — a rich gold obligation set (target: many independent obligations, not the handful reallocate had), so completeness is non-trivial even when each clause is easy.
- **Prose-incomplete** — the prose intent deliberately under-specifies, leaving several gold obligations to be *inferred*. This is the omission generator, and it is itself a craft to get right (§8).

Candidate subjects are faithful *models* of real aiwf invariants (the `reallocate` family, the status-transition FSM, the allocator), enriched for breadth. The existing `LOOM_SUBJECT` registry is the home for them.

## 3. Step 0 — the omission pilot (the gate)

Before any recorded run, a cheap pilot settles the §0.1 question with data rather than an armchair guess. **It is disjoint from the recorded run** and gates the whole PoC.

**Measures.** The honest-omission rate — the L arm only, small N, **primary model first** (opus is the hardest to make omit; if it omits, the regime works; if only weaker models do, that is itself informative). The omission rate = fraction of the gold obligation set the honest umbrella fails to entail (reusing E-0003's strength machinery).

**On.** 2–3 candidate recipe-subjects spanning the breadth / prose-incompleteness knobs, plus the floor/ceiling references — a trivial type-only umbrella (floor, ~all omitted) and the gold spec (ceiling, 0 omitted) — to fix the dynamic range.

**Decides two things.**
1. **Feasibility** — does honest omission appear in the decidable-broad regime at all? If even opus enumerates exhaustively on broad, prose-incomplete subjects, the toy regime cannot elicit the effect → escalate to harder subjects (accepting the tractability risk) or stop. The pilot thus *resolves* the regime fork rather than deferring it.
2. **Calibration** — the floor↔ceiling scale the omission-ceiling threshold (§6) anchors to.

**The safeguard — the pilot must not shop the subject.** Selecting the recorded subject by its omission rate is subject-shopping (selecting on the effect), exactly what E-0003's prereg forbids. So the pilot validates the **recipe**, never a winner:

- **Cleanest:** the pilot proves "broad + prose-incomplete reliably elicits omission" across the candidates; the recorded run uses a **fresh subject built to the same recipe**, committed before its outcomes are measured.
- **Cheaper alternative:** pre-commit to recording on **every** recipe-subject clearing a *structural* bar (decidable, gold-validated, ≥N obligations) — selecting on structure, never on the omission outcome.

Either way the pilot may set thresholds and prove the recipe *can* work; it may not pick the subject by its omission rate. Calibration ≠ recorded run.

## 4. The authored artifacts

### 4.1 The honest prompt (shared across all arms)

All three arms receive the **same (deliberately incomplete) prose intent** and are asked to **capture / test it precisely and completely** — no incentive clause, no grading game. The single difference is the artifact requested:

- **Arm L (loom umbrella):** "write the `ensures` clauses of a lemma `Spec(...)` that capture this contract precisely and completely." *(Native Dafny `ensures` is the PoC umbrella surface — a deliberate simplification; a real readable-umbrella surface might change what authors omit. §8.)*
- **Arm T1 (example tests):** "write a thorough suite of example/unit tests for this contract."
- **Arm T2 (property-based tests):** "write a thorough property-based test suite (randomized/quickcheck-style) for this contract."

T1 and T2 are matched to L on model and effort budget (token cap / wall-clock), so a loom win is not merely a loom *budget* win. Running both baselines locates loom's margin against the **realistic** competitor (T1, what people write) *and* the **strong** competitor (T2, the genuine rival to formal claims); the gap between T1 and T2 is itself informative.

### 4.2 The bug bank (the stimulus — the construct-validity crux)

Each bug is a pair `(C, B)`: the correct reference impl `C` and a buggy variant `B` = `C` with **one honest-mistake mutation**.

**Provenance.** Primary source is **LLM-generated buggy impls** — because in real loom the implementation *is* LLM-authored, so LLM-shaped mistakes are the target distribution — each validated as a genuine defect by the gold oracle. Anchored to real aiwf bug history where it exists; hand-planted bugs only to fill empty cells. The bank is **pre-registered** and the bug-planter is **independent of the umbrella/test author**.

**The class axis — a 2×2 that predicts where each method wins.** Report per-cell; a bank stacked toward any one cell predetermines the verdict:

|  | obvious-from-prose property | subtle / inferred property |
|---|---|---|
| **common input** | both catch (calibration) | **tests win** (one example pins it; the umbrella omits the property) |
| **rare input** | **loom wins** (tests do not sample it; the umbrella claims it → universal) | both miss (the genuinely-hard residual) |

Loom's value concentrates in *rare-input × obvious-property*; its blind spot in *common-input × subtle-property*; *rare × subtle* is the honest residual; *common × obvious* is calibration.

**The optimism bias you cannot design away — so frame around it.** A *planted* bug sits independently of the umbrella author's blind spots. The real failure is **correlated**: blind spot X → impl buggy on X → umbrella silent on X → bug survives. Independent planting breaks that correlation, so the detection comparison **overestimates** loom. Two consequences: (i) the decision rule is **asymmetric** (§6) — a NO-GO is strong, a GO is necessary-not-sufficient; (ii) the **omission rate is immune** to this bias (measured against the gold obligations, no bug needed), which is the main reason it, not the detection margin, is the primary outcome (§5).

### 4.3 The oracle (trust root)

The **gold spec** per subject (human-audited, deliberately small — loom-ultralight §3.2) is the arbiter of "real defect": a bug is *real* iff the gold spec rejects `B`. The gold spec is **not** under test; it decides ground truth. Containment, not elimination.

## 5. The measure

Both arm types reduce to the same `{correct, buggy}` discrimination, so the comparison is symmetric and fair.

**Arm L (umbrella `S`), reusing the certified validity gate (M-0012/M-0013):**
- **Validity gate:** `S` must verify against `C`; an `S` that rejects the correct impl is over-strong → **invalid**, excluded.
- **Caught:** `S` fails to verify against `B` → a category-(B) failure → the bug is **visible**.
- **Omitted:** `S` verifies against both `C` and `B` → `S` does not constrain the buggy property → **honest omission**.

**Arms T1/T2 (suite `TS`):**
- **Valid:** `TS` passes against `C` (no false failures); else fix/exclude.
- **Caught:** some test in `TS` fails against `B`. **Missed:** all pass against `B`.

**Outcomes (pre-registered).**
- **Primary — the honest-omission rate** (bug-free, bias-free; the cleanest measure of the thing most likely to kill loom: incompleteness). Computed as entailment of the gold obligation set under honest authorship.
- **Secondary — the detection margin** of L over `T1 ∪ T2`, reported **per-cell**, with the §4.2 optimism bias flagged. Includes the **marginal Venn**: bugs only L catches (loom's unique value), bugs only tests catch (loom's blind spots), bugs all/none catch (the residual).
- **Effort** per arm (authoring + triage tokens / wall-clock), surfaced not folded.

Z3 nondeterminism stays in an **inconclusive** bucket, never folded into caught/missed (the E-0003 trichotomy; G1).

## 6. What each outcome means — pre-register before running

The decision rule mirrors E-0003's two-dimension combine, applied to one honest arm:

- **GO** requires **low omission rate** (below the pilot-calibrated ceiling) **AND** a **positive, material detection margin** on the *rare-input × obvious-property* cell. Honest umbrellas are complete enough *and* surface blind spots thorough tests miss.
- **NO-GO if either fails:**
  - `T1 ∪ T2` catches everything L catches → no detection value over honest tests → NO-GO (as cheaply as the gaming NO-GO).
  - the omission rate exceeds the ceiling → honest authorship cannot produce complete-enough umbrellas → NO-GO by **incompleteness** (the honest analog of gaming; the most likely quiet failure).
- **L wins only on the test-shaped cell** → loom is re-proving "a claim beats a missing test," not earning its keep → reconsider the subject / bank.

**Asymmetry.** Because independent bug-planting is optimistic-for-loom (§4.2), the **GO bar is stringent** (loom must win clearly, since the real correlated setting is harder) and the **NO-GO bar lenient** (if loom can't win even here, stop). A GO is *necessary-not-sufficient*: it advances to a natural-bug test where one model authors both impl and umbrella, restoring the blind-spot correlation.

**Discipline.** Thresholds (omission ceiling, margin, materiality), the bank, the baseline protocol, and the decision rule are **frozen before the recorded run**, anchored to the pilot's floor↔ceiling calibration (on disjoint subjects), and the prereg commit is a **git-ancestor of the run commit** (`--check-prereg-ancestry`, the M-0002 integrity lesson). One recorded run; N fixed before it; no subject-shopping.

## 7. What is reused from E-0003 vs new

**Reused (why this is cheap):**
- the Dafny shell-out harness, the `{correct, buggy}` verify-and-classify loop, the killed/survived/inconclusive trichotomy;
- the **certified hybrid validity gate** (M-0012/M-0013, `D-0003`/`D-0004`) — the over-strong/over-claim instrument with its bounded residual;
- the **strength machinery** (entailment of an obligation set) — now repurposed as the omission rate;
- the subject registry, the gold-spec oracle pattern, the prereg-ancestry guard, the `--rescore` offline replay, the atomic-write / self-contained-artifact discipline (B2/C3/E3/G1).

**New:**
- a **Step 0 pilot** that gates the approach and calibrates thresholds;
- drop the **incentivized arm** — authorship is honest-only;
- swap the generic mutant bank for the curated, **2×2 honest-mistake bug bank** (stimulus, not measure);
- add the **two test-suite arms** (T1 example, T2 property-based) and a test runner alongside the verifier;
- the **omission rate** as primary outcome and the **marginal-Venn** reporting;
- the **detection fixtures** committed as offline regressions.

## 8. Threats to validity (honest)

- **Capability ceiling.** The central risk: frontier models may be too complete on any decidable subject. Mitigated by the §2 recipe (breadth + incomplete prose) and *tested directly* by the §3 pilot — which converts the threat into a gating decision rather than a buried assumption.
- **Optimism bias from independent planting.** The detection comparison overestimates loom (§4.2). Mitigated by the asymmetric decision rule and by leaning on the bias-free omission rate as primary.
- **Don't re-prove formal methods.** The *rare-input × obvious-property* cell risks demonstrating textbook "verification > tests." Mitigated by per-cell reporting and by centering the loom-specific signals (omission rate, the readable-umbrella margin), not the generic verification win.
- **Baseline fairness is load-bearing.** A weak test arm hands loom a hollow win (the E-0001 weak-comparator trap, inverted). Both arms are thorough, same model, matched effort; T2 (property-based) is the genuine rival to formal claims.
- **Pilot leak / subject-shopping.** Selecting the recorded subject by piloted omission would taint the result. Mitigated by the §3 safeguard (validate the recipe, record on a fresh/structurally-qualified subject).
- **Blinding.** The author must not see the impl or the bug, else it specs/tests straight to it. Independent bug-planting; authors see prose intent only.
- **Prose-incompleteness is a craft.** Too complete → no omission (ceiling); too sparse → the task is underspecified and authors guess → noise. The pilot calibrates this.
- **Umbrella surface.** Native Dafny `ensures` is a proxy for the (undecided) loom claims surface; a smaller readable surface might change what authors omit. Flagged, not resolved.
- **LLM-as-author proxy.** A real human+LLM *team* may claim differently than a solo LLM. The solo LLM is the cheap, N-scalable, fixturable proxy; the human-in-the-loop study is a later rung (§9).
- **Single regime.** Decidable toy models only — see §9.

## 9. What this deliberately does NOT prove

- **Tractability on real, messy code.** The decidable toy subjects keep the verifier clean; they say nothing about whether the gap report survives real stateful, multi-module code without drowning in category-(B) timeouts (`bidirectional-refinement.md` §6). That is the **separate, harder** unknown — best attacked by a feasibility smoke on *one real* aiwf component (the real `Canonicalize`, the real transition FSM), not a model of it. **Do not let clean toy results launder a tractability claim they did not earn** (the central E-0003 lesson).
- **The human-factors loop.** Whether a human actually *reads* the gap report, triages (B) into timeout/limitation/gap/failure correctly, and converts/rejects (C) correctly (`bidirectional-refinement.md` §5) — that needs humans, and is the rung after this one.
- **Net value vs cost.** Detection margin at *acceptable effort* is the real value-gate; this PoC records effort but a full cost verdict is downstream.

## 10. Next step

Not yet scaffolded as an epic. When it is, the sequence is: **(1)** run the Step 0 pilot to settle feasibility and calibrate the floor↔ceiling; **(2)** if feasible, freeze the subject recipe, the 2×2 bug bank, the baseline protocol, and the thresholds, and commit the pre-registration; **(3)** build the instrument offline against hand-built detection fixtures; **(4)** record the single run; **(5)** apply the frozen decision rule. The certified E-0003 harness is the foundation — this is a re-pointing, not a rebuild.
