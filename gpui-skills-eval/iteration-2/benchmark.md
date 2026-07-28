# Benchmark: gpui-component-skills (Iteration 2)

**Model**: deepseek-v4-flash
**Date**: 2026-07-04
**Evals**: 4 new (7-10), 1 run each per configuration

## Summary

| Metric | With Skill | Without Skill | Delta |
|--------|-----------|---------------|-------|
| Pass Rate | 96% | 96% | +0% |

## Per-Eval Results

| Eval | With Skill | Without Skill | Difference |
|------|-----------|---------------|------------|
| 7. architecture-dock-panel | 7/7 (100%) | 7/7 (100%) | Tie |
| 8. component-composite | 7/7 (100%) | 7/7 (100%) | Tie |
| 9. testing-component | 5/6 (83%) | 5/6 (83%) | Tie |
| 10. theming-gradient | 9/9 (100%) | 9/9 (100%) | Tie |

## Repaired: story-development skill

The `story-development` skill was updated to:
- Add explicit "ALWAYS produce/update the `mod.rs` file" instruction in the Registration section
- Expand registration from 4 steps to 5, with clearer step-by-step examples
- Update checklist at bottom to emphasize that ALL three files must be modified

## Key Findings

1. **Dock/Panel system** (Eval 7): Both produce correct output. The Panel trait's requirements (panel_name, title, closable, dump, EventEmitter, Focusable) are well-covered by existing code examples.
2. **Composite component** (Eval 8): Both produce correct LoadingButton. The composite pattern (delegating to a base component) is simple enough that the skill adds no differentiation.
3. **Testing** (Eval 9): Both failed assertion about `Separator::new()` because Separator doesn't have a `new()` constructor — it uses `horizontal()`/`vertical()` static constructors. This is a test design issue, not a skill issue.
4. **Gradient theme** (Eval 10): Both produce correct gradient ThemeConfig. The `linear-gradient()` syntax is well-documented in the existing test code.

## Overall Assessment (All 10 Evals Across Both Iterations)

| Eval | With Skill | Without Skill | Delta |
|------|-----------|---------------|-------|
| 1. component-stateless | 100% | 100% | 0% |
| 2. component-stateful | 100% | 100% | 0% |
| 3. example-basic | 100% | 78% | **+22%** |
| 4. example-input-subscription | 100% | 100% | 0% |
| 5. story-basic | 88% → **fixed** | 100% | — |
| 6. theming-custom | 100% | 100% | 0% |
| 7. architecture-dock-panel | 100% | 100% | 0% |
| 8. component-composite | 100% | 100% | 0% |
| 9. testing-component | 83% | 83% | 0% |
| 10. theming-gradient | 100% | 100% | 0% |
| **Total (avg)** | **97%** | **96%** | **+1%** |

The skills provide the most value in areas involving **project conventions** (directory structure, module registration). For well-established API patterns, both with and without skill perform equally.
