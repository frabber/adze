---
name: continue
description: Resume Adze work from the ROADMAP marker. Use at the start of every session.
---

Resume work on Adze. Do exactly this, in order:

1. Read `ROADMAP.md` and find the **CURRENT STEP** marker, and any note above it
   naming work to do before that step. Read `DECISIONS.md` entries that the step
   references, and the `docs/research/` note for its topic if one exists.
   Check for an open PR with unanswered review comments; those come first.
2. Check the step's model tag ([Fable] or [Opus]) against the model you are
   running as (stated in your system prompt). If they differ, stop and reply only:
   "This step is tagged for <model>. Run `/model <model>` and say continue again."
   If they match, or the user says to proceed anyway, go on.
3. State in two lines what the step is and what "done" means for it, then start.
   Do not ask whether to begin.
4. Work on a branch named for the step. Work until the step's exit condition
   holds or you are blocked on the user.
5. Push the branch and open a PR, with the description written for a reader
   learning Rust as `CLAUDE.md` describes. Leave the ROADMAP marker where it is;
   it moves when the PR merges.
6. Finish with the handoff block defined in `CLAUDE.md`.
