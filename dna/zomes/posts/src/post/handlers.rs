use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

use super::try_get_and_convert;
use super::{ Post, PostLikesEntry };
use super::ranking::*;
use super::postlikes::*;

const ALL_POSTS: &str = "ALL_POSTS";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewPostInput {
  title: String,
  content: String,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewPostOutput {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}


pub fn create_post(input: NewPostInput) -> ExternResult<NewPostOutput> {

  let agent_pub_key = agent_info()?.agent_initial_pubkey;
  let timestamp = sys_time()?;

  let anchor = anchor(ALL_POSTS.into(), ALL_POSTS.into())?;

  let post = Post {
    title: input.title,
    content: input.content,
    author: agent_pub_key.into(),
    timestamp,
  };

  let header_hash = create_entry(&post)?;

  let entry_hash = hash_entry(&post)?;

  create_link(anchor, entry_hash.clone(), LinkTag::new(String::from("POST")))?;

  create_entry_ranking( RankEntryInput { ranking: 0, entry_hash: entry_hash.clone() })?;

  let _postlikes_entry_hash = create_postlikes(
    PostLikesEntry {
      of_entry: entry_hash.clone().into(),
      likes: 0,
    }
  )?;

  let output = NewPostOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


pub fn get_post(entry_hash: EntryHashB64) -> ExternResult<Option<Post>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let post: Post = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to Post.".into()))?;

      Ok(Some(post))
    }
  }
}


pub fn get_all_posts(_:()) -> ExternResult<Vec<Post>> {

  let anchor = anchor(ALL_POSTS.into(), ALL_POSTS.into())?;

  let links = get_links(anchor, Some(LinkTag::new(String::from("POST"))))?;
  let mut posts = Vec::new();

  for link in links {
    let post = try_get_and_convert(link.target)?;
    posts.push(post);
  }

  Ok(posts)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PostWithLikes {
  pub post: Post,
  pub likes: i64,
  pub entry_hash: EntryHashB64,
}

pub fn get_all_posts_with_likes(_:()) -> ExternResult<Vec<PostWithLikes>> {

  let all_posts = get_all_posts(())?;

  let mut posts_with_likes = Vec::new();

  for post in all_posts {

    let entry_hash = hash_entry(post.clone())?;
    let (post_likes_entry, _header_hash) = get_postlikes(entry_hash.clone().into())?;

    let post_with_likes = PostWithLikes {
      post,
      likes: post_likes_entry.likes,
      entry_hash: entry_hash.into(),
    };

    posts_with_likes.push(post_with_likes);
  }

  Ok(posts_with_likes)
}
