use hdk::prelude::*;
use hc_lib_ranking_index::*;


const LIKE_INDEX: RankingIndex = RankingIndex {
  name: "likes",
  index_interval: 5,
};

#[derive(Debug)]
pub struct RankEntryInput {
  pub ranking: i64,
  pub entry_hash: EntryHash,
}


pub fn create_entry_ranking(input: RankEntryInput) -> ExternResult<()> {
  LIKE_INDEX.create_entry_ranking(input.entry_hash, input.ranking, None)
}

pub fn delete_entry_ranking(input: RankEntryInput) -> ExternResult<()> {
  LIKE_INDEX.delete_entry_ranking(input.entry_hash, input.ranking)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRankingInput {
  pub direction: GetRankingDirection,
  pub entry_count: usize,
  pub cursor: Option<GetRankingCursor>,
}


pub fn get_entry_ranking_chunk(input: GetRankingInput) -> ExternResult<EntryRanking> {
  LIKE_INDEX.get_entry_ranking_chunk(input.direction, input.entry_count, input.cursor)
}


