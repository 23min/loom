// model.dfy — loom self-host lowering: report writes are crash-safe.
//
// Property (atomic-crash-safety): a crash at any point during a report write leaves the
// destination either fully-old (or absent, if none existed) or fully-new — never a partial or
// torn report (C3). Subject: loom crates/loom/src/atomic.rs — `write_atomic` and its
// stage→commit protocol @ v0.1.0. Mirrored by reference; self-contained, NOT read from source
// at verify time (G1). Expected verdict: PROVED (category A).

// The observable content at the destination path. `Absent` and `Present` together model
// "fully-old or absent"; there is deliberately no `Partial`/`Torn` variant — the protocol must
// make torn states unrepresentable at the destination.
datatype DestState =
    Absent                    // no report has ever been committed here
  | Present(content: string)  // a fully-written report

// A point at which a crash can occur, in protocol order. Mirrors loom::atomic::write_atomic:
// stage() writes a SIBLING temp file (dest untouched), then Staged::commit() renames temp->dest,
// cleaning the temp up if the rename fails.
datatype Phase =
    BeforeStage    // nothing done yet
  | Staged         // temp written; rename not yet attempted
  | CommitFailed   // rename attempted and failed; temp cleaned up
  | Committed      // rename succeeded

// The destination's observable state at a given crash phase, given the starting state and the
// new content being written. The temp is a sibling, never the dest, so staging and a failed
// commit both leave the destination exactly at `start`; only a successful rename changes it, to
// the fully-new content, atomically.
function Observe(start: DestState, newContent: string, phase: Phase): DestState {
  match phase
  case BeforeStage  => start
  case Staged       => start
  case CommitFailed => start
  case Committed    => Present(newContent)
}

// "Fully-old or fully-new": the observed state is the untouched starting state (the prior
// report, or absence) or exactly the new content — never anything in between.
predicate FullyOldOrNew(observed: DestState, start: DestState, newContent: string) {
  observed == start || observed == Present(newContent)
}

// The property: at EVERY crash phase — i.e. no matter when the process dies — the destination
// is fully-old/absent or fully-new. Never torn.
lemma NoTornReportAtAnyCrashPoint(start: DestState, newContent: string)
  ensures forall phase: Phase :: FullyOldOrNew(Observe(start, newContent, phase), start, newContent)
  // faithfulness spot-checks — the corners of the protocol:
  ensures Observe(start, newContent, BeforeStage) == start          // crash before staging
  ensures Observe(start, newContent, Staged) == start               // crash after staging
  ensures Observe(start, newContent, CommitFailed) == start         // rename failed, temp cleaned
  ensures Observe(start, newContent, Committed) == Present(newContent) // rename succeeded
{ }

// Corollary: crash-safety holds equally whether or not a prior report existed — the "absent"
// case is not a special path. Pins that Absent is covered by the same `observed == start` arm.
lemma CrashSafeFromAbsentAndFromPresent(newContent: string, prior: string)
  ensures forall phase: Phase :: FullyOldOrNew(Observe(Absent, newContent, phase), Absent, newContent)
  ensures forall phase: Phase ::
    FullyOldOrNew(Observe(Present(prior), newContent, phase), Present(prior), newContent)
{
  NoTornReportAtAnyCrashPoint(Absent, newContent);
  NoTornReportAtAnyCrashPoint(Present(prior), newContent);
}
