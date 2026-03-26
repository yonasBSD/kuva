# kuva heatmap

Color-encoded matrix heatmap.

**Input:** wide-format matrix — first column is the row label, remaining columns are numeric values. The header row (if present) supplies column labels.

```
gene    Sample_01  Sample_02  Sample_03 …
 TP53    0.25       -1.78       1.58     …
BRCA1   0.23        0.48       1.06     …
```

| Flag | Default | Description |
|---|---|---|
| `--colormap <NAME>` | `viridis` | Color map: `viridis`, `inferno`, `grayscale` |
| `--values` | off | Print numeric values in each cell |
| `--legend <LABEL>` | — | Show color bar with this label |

```bash
kuva heatmap heatmap.tsv

kuva heatmap heatmap.tsv --colormap inferno --values --legend "z-score"
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes, log scale.*
