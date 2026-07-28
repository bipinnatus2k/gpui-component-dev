# Benchmark: gpui-component-skills

**Model**: deepseek-v4-flash
**Date**: 2026-07-04
**Evals**: 6 (1 run each per configuration)

## Summary

| Metric | With Skill | Without Skill | Delta |
|--------|-----------|---------------|-------|
| Pass Rate | 98% | 96% | +2% |
| Time | 30.0s | 30.0s | — |
| Tokens | 50k | 50k | — |

## Per-Eval Results

| Eval | With Skill | Without Skill | Difference |
|------|-----------|---------------|------------|
| 1. component-stateless | 9/9 (100%) | 9/9 (100%) | Tie |
| 2. component-stateful | 8/8 (100%) | 8/8 (100%) | Tie |
| 3. example-basic | 9/9 (100%) | 7/9 (78%) | **+22%** ✅ |
| 4. example-input-subscription | 8/8 (100%) | 8/8 (100%) | Tie |
| 5. story-basic | 7/8 (88%) | 8/8 (100%) | **-12%** ❌ |
| 6. theming-custom | 9/9 (100%) | 9/9 (100%) | Tie |

## Key Findings

### Where Skills Help
1. **example-basic**: With-skill correctly created `examples/counter/src/main.rs` structure. Without-skill placed files at flat root — missed the `examples/counter/src/` directory convention.

### Where Skills Need Improvement
1. **story-basic**: With-skill produced the story file but did NOT output a `mod.rs` registration. The without-skill correctly included `mod kbd_story;` + `pub use`. The story-development skill should emphasize that registration in `mod.rs` is a required output.

### Where Both Perform Equally
- **component-stateless**: Both created nearly identical `label.rs` files. The project patterns are consistent enough that the skill adds little differentiation.
- **component-stateful**: Both created full Rating implementations with Entity pattern, EventEmitter, click/hover interaction.
- **example-input-subscription**: Both correctly used subscription pattern.
- **theming-custom**: Both produced correct ThemeConfig with proper color keys.

## Observations
- The skills that focus on **project conventions** (directory layout, module registration) provide more value than skills focused on **well-established patterns** that are already in the existing codebase.
- Eval 5 shows a regression — the story-development skill didn't include the registration step in its workflow. This should be fixed.
- Most evals show that even without the skill, the existing codebase's consistent patterns enable good results.
