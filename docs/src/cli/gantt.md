# kuva gantt

Gantt chart — horizontal task bars with optional group/phase headers, progress fills, milestone diamonds, and a "now" reference line.

**Input:** one row per task with label, start, and end columns. Optional columns for group, progress, and milestone flag.

| Flag | Default | Description |
|---|---|---|
| `--label-col <COL>` | `0` | Task name column |
| `--start-col <COL>` | `1` | Task start value column |
| `--end-col <COL>` | `2` | Task end value column |
| `--group-col <COL>` | — | Group/phase column; tasks with the same value are grouped together |
| `--progress-col <COL>` | — | Completion fraction column (values `0`–`1`; values `> 1` are divided by 100) |
| `--milestone-col <COL>` | — | Milestone flag column; `1`, `true`, or `yes` marks a task as a milestone diamond |
| `--now <F>` | — | Draw a dashed vertical "now" line at this x value |
| `--bar-height <F>` | `0.6` | Bar height as a fraction of row height |
| `--color <CSS>` | `steelblue` | Default bar color when no group column is supplied |
| `--no-labels` | off | Hide task and milestone labels |

```bash
# Minimal: label, start, end
kuva gantt schedule.tsv --label-col task --start-col week_start --end-col week_end

# With groups and progress
kuva gantt plan.tsv \
    --label-col task --start-col start --end-col end \
    --group-col phase --progress-col pct_done \
    --now 8 --title "Q3 Roadmap"

# With milestone flag and output to file
kuva gantt milestones.tsv \
    --label-col name --start-col start --end-col end \
    --group-col phase --milestone-col is_milestone \
    --now 12 --title "Release Plan" -o release.svg
```

**Example TSV format (with all optional columns):**

```
phase       task            start   end     progress    milestone
Planning    Requirements    0       2       1.0         0
Planning    Architecture    1       3       0.8         0
Planning    Sign-off        3       3       0           1
Execution   Core build      3       9       0.5         0
Execution   Code freeze     11      11      0           1
Launch      Testing         10      13      0           0
Launch      Public launch   14      14      0           1
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes.*
