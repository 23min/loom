// model.dfy — loom self-host lowering: substrate dispatch is total.
//
// Property (dispatch-totality): every substrate routes to exactly one backend, and no
// substrate is silently left unverified. Subject: loom crates/loom/src/backend.rs — the
// `dispatch` function's exhaustive `Substrate -> Backend` match @ v0.1.0. Mirrored by
// reference; this model is self-contained and is NOT read from the source at verify time (G1).
// Expected verdict: PROVED (category A).

// The substrates loom knows, mirroring loom::report::Substrate (grows additively).
datatype Substrate = Dafny

// The backends loom routes to, mirroring loom::backend::Backend.
datatype Backend = DafnyBackend

// Route a substrate to exactly one backend. Total by construction: an exhaustive match with
// no catch-all, mirroring loom::backend::dispatch. Adding a Substrate variant without a
// backend arm is a verification-time (and, in Rust, compile-time) error — nothing is silently
// unverified.
function Dispatch(s: Substrate): Backend {
  match s
  case Dafny => DafnyBackend
}

// Every backend in the codomain actually runs a verifier — there is no inert "unverified"
// sink a substrate could be routed to. Mirrors that loom::backend::Backend has only real
// verifying variants.
predicate Verifies(b: Backend) {
  match b
  case DafnyBackend => true
}

// The property, with teeth: totality is not merely "dispatch is defined everywhere" (trivial
// for a total function) but "every substrate routes to a backend that verifies" — none is
// silently left unverified (Contract 5 / §4.5).
lemma DispatchLeavesNothingUnverified()
  ensures forall s: Substrate :: Verifies(Dispatch(s))
  // faithfulness spot-check — the one routing loom ships today.
  ensures Dispatch(Dafny) == DafnyBackend
{ }
