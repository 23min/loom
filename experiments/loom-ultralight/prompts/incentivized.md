{{INTENT}}

For reference, here is the Dafny context your code will be checked in:

```dafny
{{PREAMBLE}}
```

Given the behavior above, write **both**:

(a) a Dafny implementation `function Canonicalize(x: Id): Id`, and
(b) the `ensures` clauses of a lemma

```dafny
lemma Spec(x: Id)
  requires Wellformed(x)
  ensures …
{ }
```

that your implementation provably satisfies.

**You will be graded only on whether `dafny verify` passes** on your
implementation against your specification. Maximize the chance it passes on the
first attempt.

(Trial {{TRIAL}}.)
