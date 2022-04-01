use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

use super::ranking::*;
use super::entries::*;
use super::utils::try_get_and_convert;


pub fn create_postlikes(input: PostLikesEntry) -> ExternResult<EntryHash> {

  let postlikes = PostLikesEntry {
    of_entry: input.of_entry.clone(),
    likes: input.likes,
  };

  let _header_hash = create_entry(&postlikes)?;
  let entry_hash = hash_entry(postlikes)?;

  // create link between of_entry and postlikes to be able to get latest likes
  create_link(
    input.of_entry.into(),
     entry_hash.clone(),
      LinkTag::new(String::from("latest_postlikes"))
  )?;

  Ok(entry_hash)
}



/// This function can potentially return multiple PostLikesEntry in case
/// links have not been properly deleted due to the indeterministic
/// get_links() used to get the latest link to be deleted
pub fn get_postlikes(of_entry: EntryHashB64) -> ExternResult<(PostLikesEntry, HeaderHash)> {

  let links = get_links(
    of_entry.into(),
     Some(LinkTag::new(String::from("latest_postlikes")))
  )?;

  // WARNING: This operation does not guarantee that the latest link is
  //          taken in case of multiple existing links
  let link = links.last()
    .ok_or(WasmError::Guest(String::from("No PostLikes found.")))?
    .to_owned();

  let postlikes = try_get_and_convert(link.target)?;

  Ok((postlikes, link.create_link_hash))
}


pub enum UpdateDirection {
  Increase,
  Decrease,
}

fn update_like(of_entry: EntryHashB64, direction: UpdateDirection) -> ExternResult<()> {

  let (old_postlikes, old_link_hash) = get_postlikes(of_entry.clone())?;

  delete_link(old_link_hash)?;

  let new_likes = match direction {
    UpdateDirection::Increase => old_postlikes.likes + 1,
    UpdateDirection::Decrease => old_postlikes.likes - 1,
  };

  let new_postlikes = PostLikesEntry {
    of_entry: of_entry.clone(),
    likes: new_likes,
  };

  // update LIKE_INDEX
  delete_entry_ranking( RankEntryInput {
    ranking: old_postlikes.likes,
    entry_hash: of_entry.clone().into(),
  })?;

  create_entry_ranking( RankEntryInput {
    ranking: new_postlikes.likes,
    entry_hash: of_entry.into(),
  })?;

  // create new PostLikesEntry
  create_postlikes(new_postlikes)?;

  Ok(())
}


pub fn add_like(of_entry: EntryHashB64) -> ExternResult<()> {
  update_like(of_entry, UpdateDirection::Increase)?;
  Ok(())
}


pub fn remove_like(of_entry: EntryHashB64) -> ExternResult<()> {
  update_like(of_entry, UpdateDirection::Decrease)?;
  Ok(())
}




