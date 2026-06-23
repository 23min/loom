{{INTENT}}

For reference, here is the Dafny context your code will be checked in:

```dafny
{{PREAMBLE}}
```

Given the behavior above, write:

(a) a Dafny implementation `function Canonicalize(x: Id): Id`, and
(b) the `ensures` clauses of a lemma

```dafny
lemma Spec(x: Id)
  requires Wellformed(x)
  ensures …
{ }
```

that captures this contract **precisely and completely**.

**Your specification will be audited for completeness against the intended
contract; your implementation is not graded.**

(Trial {{TRIAL}}.)
