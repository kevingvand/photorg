use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

// Beta(PRIOR_WINS, PRIOR_LOSSES) prior — currently Beta(1,1), a uniform distribution.
// PRIOR_WINS must be > 0 and PRIOR_WINS < PRIOR_STRENGTH to keep scores in (0, 1).
const PRIOR_STRENGTH: f32 = 2.0;
const PRIOR_WINS: f32 = 1.0;
const PRIOR_LOSSES: f32 = PRIOR_STRENGTH - PRIOR_WINS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonWinner {
    ImageA,
    ImageB,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PairwiseComparison {
    pub image_a: String,
    pub image_b: String,
    pub winner: ComparisonWinner,
    pub confidence: f32,
    pub timestamp_ms: u64,
}

impl PairwiseComparison {
    pub fn new(
        image_a: impl Into<String>,
        image_b: impl Into<String>,
        winner: ComparisonWinner,
        confidence: f32,
        timestamp_ms: u64,
    ) -> Self {
        Self {
            image_a: image_a.into(),
            image_b: image_b.into(),
            winner,
            confidence: sanitize_confidence(confidence),
            timestamp_ms,
        }
    }

    fn winner_id(&self) -> &str {
        match self.winner {
            ComparisonWinner::ImageA => &self.image_a,
            ComparisonWinner::ImageB => &self.image_b,
        }
    }

    fn loser_id(&self) -> &str {
        match self.winner {
            ComparisonWinner::ImageA => &self.image_b,
            ComparisonWinner::ImageB => &self.image_a,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankedImage {
    pub image_id: String,
    pub score: f32,
    /// How much direct evidence backs this score (0 = none, approaches 1 as comparisons accumulate).
    /// Distinct from `PairwiseComparison.confidence`, which is user-provided per-vote certainty.
    pub evidence_strength: f32,
    pub wins: f32,
    pub losses: f32,
    /// Comparisons that contributed nonzero weight. Zero-confidence comparisons are excluded.
    pub effective_comparison_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PairwiseMatch {
    pub image_a: String,
    pub image_b: String,
}

#[derive(Debug, Default, Clone)]
struct ImageStats {
    wins: f32,
    losses: f32,
    comparison_count: u32,
}

pub fn rank_images(image_ids: &[String], comparisons: &[PairwiseComparison]) -> Vec<RankedImage> {
    let stats = build_stats(image_ids, comparisons);
    let mut ranked = stats
        .into_iter()
        .map(|(image_id, stats)| RankedImage {
            image_id,
            score: posterior_win_probability(stats.wins, stats.losses),
            evidence_strength: evidence_confidence(stats.wins, stats.losses),
            wins: stats.wins,
            losses: stats.losses,
            effective_comparison_count: stats.comparison_count,
        })
        .collect::<Vec<_>>();

    ranked.sort_by(compare_ranked_images);
    ranked
}

pub fn rank_known_images(comparisons: &[PairwiseComparison]) -> Vec<RankedImage> {
    let image_ids = discovered_image_ids(comparisons);
    rank_images(&image_ids, comparisons)
}

pub fn generate_pairwise_matches(image_ids: &[String]) -> Vec<PairwiseMatch> {
    let unique_ids = image_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let mut matches = Vec::new();
    for start_index in 0..unique_ids.len() {
        for end_index in (start_index + 1)..unique_ids.len() {
            matches.push(PairwiseMatch {
                image_a: unique_ids[start_index].clone(),
                image_b: unique_ids[end_index].clone(),
            });
        }
    }

    matches
}

fn build_stats(
    image_ids: &[String],
    comparisons: &[PairwiseComparison],
) -> BTreeMap<String, ImageStats> {
    let mut stats: BTreeMap<String, ImageStats> = BTreeMap::new();

    for image_id in image_ids {
        stats.entry(image_id.clone()).or_default();
    }

    for comparison in comparisons {
        let weight = sanitize_confidence(comparison.confidence);
        if weight == 0.0 {
            continue;
        }

        stats.entry(comparison.image_a.clone()).or_default();
        stats.entry(comparison.image_b.clone()).or_default();

        let winner = comparison.winner_id().to_owned();
        let loser = comparison.loser_id().to_owned();

        let winner_stats = stats.get_mut(&winner).expect("winner entry must exist");
        winner_stats.wins += weight;
        winner_stats.comparison_count += 1;

        let loser_stats = stats.get_mut(&loser).expect("loser entry must exist");
        loser_stats.losses += weight;
        loser_stats.comparison_count += 1;
    }

    stats
}

fn discovered_image_ids(comparisons: &[PairwiseComparison]) -> Vec<String> {
    let mut image_ids = BTreeSet::new();
    for comparison in comparisons {
        image_ids.insert(comparison.image_a.clone());
        image_ids.insert(comparison.image_b.clone());
    }
    image_ids.into_iter().collect()
}

fn posterior_win_probability(wins: f32, losses: f32) -> f32 {
    (wins + PRIOR_WINS) / (wins + losses + PRIOR_WINS + PRIOR_LOSSES)
}

fn evidence_confidence(wins: f32, losses: f32) -> f32 {
    let evidence = wins + losses;
    if evidence == 0.0 {
        0.0
    } else {
        evidence / (evidence + PRIOR_STRENGTH)
    }
}

fn sanitize_confidence(confidence: f32) -> f32 {
    if confidence.is_finite() {
        confidence.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

fn compare_ranked_images(left: &RankedImage, right: &RankedImage) -> Ordering {
    right
        .score
        .total_cmp(&left.score)
        .then_with(|| right.evidence_strength.total_cmp(&left.evidence_strength))
        .then_with(|| right.effective_comparison_count.cmp(&left.effective_comparison_count))
        .then_with(|| left.image_id.cmp(&right.image_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn comparison(
        image_a: &str,
        image_b: &str,
        winner: ComparisonWinner,
        confidence: f32,
        timestamp_ms: u64,
    ) -> PairwiseComparison {
        PairwiseComparison::new(image_a, image_b, winner, confidence, timestamp_ms)
    }

    #[test]
    fn confidence_updates_when_new_comparison_arrives() {
        let images = vec!["alpha".to_string(), "beta".to_string()];
        let first = vec![comparison("alpha", "beta", ComparisonWinner::ImageA, 1.0, 1)];
        let second = vec![
            comparison("alpha", "beta", ComparisonWinner::ImageA, 1.0, 1),
            comparison("alpha", "beta", ComparisonWinner::ImageA, 1.0, 2),
        ];

        let first_rank = rank_images(&images, &first);
        let second_rank = rank_images(&images, &second);

        assert!(second_rank[0].evidence_strength > first_rank[0].evidence_strength);
        assert!(second_rank[0].score > first_rank[0].score);
    }

    #[test]
    fn cycles_remain_deterministic_without_transitive_inference() {
        let images = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
        let comparisons = vec![
            comparison("alpha", "beta", ComparisonWinner::ImageA, 1.0, 1),
            comparison("beta", "gamma", ComparisonWinner::ImageA, 1.0, 2),
            comparison("gamma", "alpha", ComparisonWinner::ImageA, 1.0, 3),
        ];

        let ranked = rank_images(&images, &comparisons);

        assert_eq!(ranked.iter().map(|image| image.image_id.as_str()).collect::<Vec<_>>(), vec!["alpha", "beta", "gamma"]);
        assert_eq!(ranked[0].score, ranked[1].score);
        assert_eq!(ranked[1].score, ranked[2].score);
    }

    #[test]
    fn ranking_is_independent_of_comparison_order() {
        let images = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
        let comparisons = vec![
            comparison("alpha", "beta", ComparisonWinner::ImageA, 1.0, 1),
            comparison("alpha", "gamma", ComparisonWinner::ImageA, 1.0, 2),
            comparison("beta", "gamma", ComparisonWinner::ImageA, 1.0, 3),
        ];
        let expected = rank_images(&images, &comparisons);

        let permutations = vec![
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ];

        for permutation in permutations {
            let reordered = permutation
                .into_iter()
                .map(|index| comparisons[index].clone())
                .collect::<Vec<_>>();
            let ranked = rank_images(&images, &reordered);

            assert_eq!(ranked, expected);
        }
    }

    #[test]
    fn weighted_comparison_contributes_fractional_evidence() {
        let images = vec!["alpha".to_string(), "beta".to_string()];

        let half_weight = vec![comparison("alpha", "beta", ComparisonWinner::ImageA, 0.5, 1)];
        let full_weight = vec![comparison("alpha", "beta", ComparisonWinner::ImageA, 1.0, 1)];

        let half = rank_images(&images, &half_weight);
        let full = rank_images(&images, &full_weight);

        assert!(full[0].score > half[0].score);
        assert!(full[0].evidence_strength > half[0].evidence_strength);
    }

    #[test]
    fn rank_images_includes_images_with_zero_comparisons() {
        let images = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
        let comparisons = vec![comparison("alpha", "beta", ComparisonWinner::ImageA, 1.0, 1)];

        let ranked = rank_images(&images, &comparisons);

        assert_eq!(ranked.len(), 3);
        assert!(ranked.iter().any(|image| image.image_id == "gamma"));
    }

    #[test]
    fn rank_known_images_excludes_images_with_zero_comparisons() {
        let comparisons = vec![comparison("alpha", "beta", ComparisonWinner::ImageA, 1.0, 1)];

        let ranked = rank_known_images(&comparisons);

        assert_eq!(ranked.len(), 2);
        assert!(!ranked.iter().any(|image| image.image_id == "gamma"));
    }

    #[test]
    fn deserialized_comparison_with_out_of_range_confidence_is_handled_safely() {
        let json = r#"{"image_a":"alpha","image_b":"beta","winner":"image_a","confidence":99.9,"timestamp_ms":1}"#;
        let decoded: PairwiseComparison =
            serde_json::from_str(json).expect("deserialization should succeed");

        let ranked = rank_known_images(&[decoded]);

        assert!(ranked[0].score <= 1.0);
        assert!(ranked[0].score >= 0.0);
    }

    #[test]
    fn generate_pairwise_matches_deduplicates_input_and_covers_all_pairs() {
        let images = vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
            "alpha".to_string(), // duplicate
        ];

        let matches = generate_pairwise_matches(&images);

        assert_eq!(matches.len(), 3); // C(3,2) = 3 unique pairs
    }

    #[test]
    fn images_with_equal_score_are_ordered_by_id() {
        let images = vec!["zeta".to_string(), "alpha".to_string()];
        let ranked = rank_images(&images, &[]);

        assert_eq!(ranked[0].image_id, "alpha");
        assert_eq!(ranked[1].image_id, "zeta");
    }

    #[test]
    fn comparisons_round_trip_through_json() {
        let comparison = comparison("alpha", "beta", ComparisonWinner::ImageB, 0.75, 42);
        let json = serde_json::to_string(&comparison).expect("comparison should serialize");
        let decoded: PairwiseComparison =
            serde_json::from_str(&json).expect("comparison should deserialize");

        assert_eq!(decoded, comparison);
    }

    #[test]
    fn ten_round_tournament_over_fifty_images_returns_valid_ranking() {
        let images = (0..50).map(|index| format!("image-{index:02}")).collect::<Vec<_>>();
        let mut comparisons = Vec::new();

        for round_index in 0..10 {
            for index in 0..(images.len() - 1) {
                comparisons.push(comparison(
                    &images[index],
                    &images[index + 1],
                    ComparisonWinner::ImageA,
                    1.0,
                    (round_index * 100 + index) as u64,
                ));
            }
        }

        let ranked = rank_images(&images, &comparisons);

        assert_eq!(ranked.len(), images.len());
        assert!(ranked.windows(2).all(|pair| pair[0].score >= pair[1].score));
        assert_eq!(ranked.first().map(|image| image.image_id.as_str()), Some("image-00"));
        assert_eq!(ranked.last().map(|image| image.image_id.as_str()), Some("image-49"));
    }
}
