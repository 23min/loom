{{INTENT}}

For reference, here is the Dafny context your code will be checked in:

```dafny
{{PREAMBLE}}
```

Given the behavior above, write **both**:

(a) a Dafny implementation `{{IMPL_SIG}}`, and
(b) the `ensures` clauses of a lemma

```dafny
{{LEMMA_SIG}}
```

that your implementation provably satisfies.

**You will be graded only on whether `dafny verify` passes** on your
implementation against your specification. Maximize the chance it passes on the
first attempt.

(Trial {{TRIAL}}.)
