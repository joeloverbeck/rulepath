# Plan Mode Awareness

If plan mode is active:

- **Steps 1–6** proceed normally (all read-only).
- **Step 6** includes the initial findings report and any question-resolution rounds.
- **If questions exist**: present Step 6 findings in conversation → use `AskUserQuestion` for each question → incorporate resolved answers into the plan file. The conversational report + question resolutions are the decision record; the plan file is the condensed implementation reference.
- **After all questions are resolved**: write a condensed summary to the plan file, then call `ExitPlanMode`. If question resolution produced new findings, the plan file reflects the final resolved state, not the initial report.
- **After plan approval**: Steps 7–8 execute. The user's approval covers both question resolutions and overall changes — no separate gate.
- **Pre-Apply Verification** runs after `ExitPlanMode` approval, before Step 7.
- If there are no questions, proceed from the Step 6 findings report directly to writing the plan file and calling `ExitPlanMode`.
- If the `ExitPlanMode` result contains user comments, treat them as binding modifications.
- **Delegated resolution**: when a question is resolved via delegation ("you decide"), include the resolution rationale in the plan file alongside the resulting change. The `ExitPlanMode` approval then covers both.

**Plan file structure** (condensed action list for Step 7 execution — not a re-presentation of the conversational report):

- **Context**: which spec, why it's being reassessed, classification type (a/b/c/d).
- **Approved Changes**: compact action items grouped Issues Fixed / Improvements Applied / Additions Incorporated. Each item: severity tag + one-line summary + affected deliverable number or section. For question-resolved changes, append the resolution rationale in parentheses.
- **Critical Files**: paths to be modified.
- **Verification**: how to confirm the updated spec is correct after writing (which post-apply greps, which re-run commands).

The conversational report (Step 6) is the decision artifact — present it as a normal message, do not write it to the plan file. The plan file is a separate condensed reference for Steps 7–8.
