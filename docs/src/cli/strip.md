# kuva strip

Strip / jitter plot — individual points along a categorical axis.

**Input:** group label column + numeric value column, one observation per row.

| Flag | Default | Description |
|---|---|---|
| `--group-col <COL>` | `0` | Group label column |
| `--value-col <COL>` | `1` | Numeric value column |
| `--color <CSS>` | `steelblue` | Point color |
| `--point-size <PX>` | `4.0` | Point radius in pixels |
| `--swarm` | off | Beeswarm (non-overlapping) layout |
| `--center` | off | All points at group center (no spread) |

Default layout when neither `--swarm` nor `--center` is given: random jitter (±30 % of slot width).

```bash
kuva strip samples.tsv --group-col group --value-col expression

kuva strip samples.tsv --group-col group --value-col expression --swarm
```

---

*See also: [Shared flags](./index.md#shared-flags) — output, appearance, axes, log scale.*
