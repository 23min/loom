// model.dfy — loom self-host lowering: umbrella parsing is total.
//
// Property (parser-totality): every umbrella input yields either a parsed umbrella or a typed
// rejection — never a panic, never a silent misparse. Subject: loom crates/loom/src/umbrella.rs
// — the `parse` function @ v0.1.0. Mirrored by reference; self-contained, NOT read from source
// at verify time (G1). Expected verdict: PROVED (category A).
//
// Abstraction: the Rust parser scans lines for a `substrate:` prefix and collects the trimmed
// values. This model works over that collected sequence of declarations — one `Token` per
// `substrate:` line found — modeling the parse's *decision*, not the byte-level line scan. The
// value of each declaration is abstracted to whether it is empty, a substrate loom knows, or an
// unknown token.

// The abstracted value of one `substrate:` declaration.
datatype Token =
    Empty     // the value after `substrate:` trimmed to nothing
  | Known     // a substrate loom recognizes (e.g. "dafny")
  | Unknown   // a non-empty value loom does not recognize

// The total classification of a parse. Mirrors loom::umbrella::{Umbrella, ParseError}:
// Ok ~ Ok(Umbrella); the three errors ~ ParseError::{Missing, Duplicate, Unknown}.
datatype ParseResult = Ok | ErrMissing | ErrDuplicate | ErrUnknown

// Classify the parse from the sequence of `substrate:` declarations found, faithfully to
// loom::umbrella::parse: two-or-more declarations reject as Duplicate (the Rust returns on the
// second occurrence, before emptiness is ever checked); with exactly one, an empty value is
// Missing, an unknown value is Unknown, a known value is Ok; with none, Missing.
function Parse(decls: seq<Token>): ParseResult {
  if |decls| == 0 then ErrMissing
  else if |decls| >= 2 then ErrDuplicate
  else match decls[0]
    case Empty => ErrMissing
    case Unknown => ErrUnknown
    case Known => Ok
}

// The property: parsing is a total partition — every possible input maps to exactly one
// outcome (never a panic, never a silent misparse). A total Dafny function is total by
// construction; the lemma makes the claim explicit and pins the classification against
// regression, with faithfulness spot-checks proving each branch is reachable.
lemma ParseClassifiesEveryInput()
  ensures forall decls: seq<Token> :: Parse(decls) in {Ok, ErrMissing, ErrDuplicate, ErrUnknown}
  // faithfulness — each branch reachable, matching loom::umbrella::parse:
  ensures Parse([]) == ErrMissing            // no substrate: line
  ensures Parse([Empty]) == ErrMissing       // `substrate:` with an empty value
  ensures Parse([Unknown]) == ErrUnknown     // one unrecognized value
  ensures Parse([Known]) == Ok               // one recognized value
  ensures Parse([Known, Known]) == ErrDuplicate  // two declarations → duplicate
  ensures Parse([Empty, Empty]) == ErrDuplicate  // duplicate wins before emptiness is checked
{ }
