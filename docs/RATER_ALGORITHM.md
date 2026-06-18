# Rater Algorithm

Photorg uses a deterministic, append-only pairwise ranking model for goal-based culling.

## Data model

Each stored comparison records:

- `image_a`
- `image_b`
- `winner` (`image_a` or `image_b`)
- `confidence` (`0.0..1.0`, user certainty / weight)
- `timestamp_ms` (Unix milliseconds)

Comparisons are never deleted. Replays use the full log.

## Scoring model

The v1 scorer is a simple Bayesian pairwise model using a Beta(1,1) prior (uniform):

- each direct win adds weighted evidence to the winner
- each direct loss adds weighted evidence to the loser
- `PairwiseComparison.confidence` is treated as a per-vote weight, not as an algorithm output
- the final `score` is the posterior win probability: `(wins + 1) / (wins + losses + 2)`
- `evidence_strength` (on `RankedImage`) measures how much direct evidence backs the score; it starts at 0 and approaches 1 as weighted comparisons accumulate

The priors ensure images with zero comparisons start at `score = 0.5`, neither ranked first nor last.

This keeps ranking deterministic, order-independent, and easy to audit.

## Tournament logic

- Tournament rounds are decomposed into direct pairwise matches.
- For multi-image rounds, all pairs in the presented set are generated deterministically.
- No random shuffles are used in ranking logic.

## Transitivity

No transitive wins are inferred.

- If `A > B` and `B > C`, the model does **not** assume `A > C`.
- Cycles such as `A > B > C > A` remain valid inputs.
- Cycles are resolved only by direct evidence and deterministic tie-breakers.

## Incomplete data

Missing comparisons are normal.

- Images with no direct comparisons remain at the prior baseline.
- More evidence raises confidence.
- Sparse sets are still rankable because the model does not require a fully connected graph.

## Reproducibility

The ranking output is deterministic:

1. score descending
2. evidence_strength descending
3. effective_comparison_count descending
4. image id ascending

Given the same comparisons, the same output is produced every time.

## JSON export

Comparisons are JSON-serializable for audit and portability. The export is the source of truth for replays.

