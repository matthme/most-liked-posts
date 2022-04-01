import type { HeaderHashB64, EntryHashB64 } from '@holochain-open-dev/core-types';

export interface Post {
  title: string,
  content: string,
  author: string,
  timestamp: number,
}

export interface PostWithLikes {
  post: Post,
  likes: number,
  entry_hash: EntryHashB64,
}

export interface NewPostInput {
  title: string,
  content: string,
}

export interface NewPostOutput {
  headerHash: HeaderHashB64,
  entryHash: EntryHashB64,
}
export enum GetRankingDirection {
  Ascendent,
  Descendent,
}

export interface GetRankingCursor {
  fromRanking: number,
}

export interface GetRankingInput {
  direction: GetRankingDirection,
  entryCount: number,
  cursor: GetRankingCursor | undefined,
}

