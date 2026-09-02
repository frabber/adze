---
name: continue
description: Resume Adze work from the ROADMAP marker. Use at the start of every session.
---

Resume work on Adze. Do exactly this, in order:

1. Read `ROADMAP.md` and find the **CURRENT STEP** marker. Read `DECISIONS.md`
   entries that the step references, and the `docs/research/` note for its topic
   if one exists.
2. Check the step's model tag ([Fable] or [Opus]) against the model you are
   running as (stated in your system prompt). If they differ, stop and reply only:
   "This step is tagged for <model>. Run `/model <model>` and say continue again."
   If they match, or the user says to proceed anyway, go on.
3. State in two lines what the step is and what "done" means for it, then start.
   Do not ask whether to begin.
4. Work until the step's exit condition holds or you are blocked on the user.
5. Finish with the handoff block defined in `CLAUDE.md`.
