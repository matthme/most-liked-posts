use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

mod post;
use post::*;

use hc_lib_ranking_index::{EntryRanking, GetRankingDirection};

entry_defs![
  Post::entry_def(),
  PostLikesEntry::entry_def(),
  Path::entry_def(),
  PathEntry::entry_def()
];




#[hdk_extern]
pub fn create_post(input: NewPostInput) -> ExternResult<NewPostOutput> {
  post::create_post(input)
}

#[hdk_extern]
pub fn get_post(entry_hash: EntryHashB64) -> ExternResult<Option<Post>> {
  post::get_post(entry_hash)
}

#[hdk_extern]
pub fn get_all_posts(_:()) -> ExternResult<Vec<Post>> {
  post::get_all_posts(())
}

#[hdk_extern]
pub fn get_all_posts_with_likes(_:()) -> ExternResult<Vec<PostWithLikes>> {
  post::get_all_posts_with_likes(())
}

#[hdk_extern]
pub fn add_like(of_entry: EntryHashB64) -> ExternResult<()> {
  post::add_like(of_entry)
}

#[hdk_extern]
pub fn remove_like(of_entry: EntryHashB64) -> ExternResult<()> {
  post::remove_like(of_entry)
}

#[hdk_extern]
pub fn get_entry_ranking_chunk(input: GetRankingInput) -> ExternResult<EntryRanking> {
  post::get_entry_ranking_chunk(input)
}

#[hdk_extern]
pub fn get_n_most_popular(n: usize) -> ExternResult<Vec<PostWithLikes>> {

  let query = GetRankingInput {
    direction: GetRankingDirection::Descendent,
    entry_count: n,
    cursor: None,
  };

  let entry_ranking = get_entry_ranking_chunk(query)?;

  let mut output = Vec::new();

  for (key, value) in entry_ranking.iter() {

    for entry_hash_with_tag in value {
      let post = get_post(entry_hash_with_tag.entry_hash.to_owned().into())?
        .ok_or(WasmError::Guest(String::from("No post found for hash.")))?;
      let post_with_likes = PostWithLikes {
        post,
        likes: key.to_owned(),
        entry_hash: entry_hash_with_tag.entry_hash.to_owned().into(),
      };
      output.push(post_with_likes)
    }
  }

  Ok(output)
}

