<script lang="ts">

  import type { PostWithLikes } from "../../../types/most_liked_posts/posts";
  import '@material/mwc-icon';

  import { createEventDispatcher } from "svelte";

  let dispatch = createEventDispatcher();

  import type { InstalledCell } from '@holochain/client';
  import type { EntryHashB64 } from '@holochain-open-dev/core-types';

  export let postWithLikes: PostWithLikes;
  export let appInfo;
  export let appWebsocket;



  const addLike = async (entryHash: EntryHashB64) => {

    const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'most_liked_posts')!;

    await appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: 'posts',
      fn_name: 'add_like',
      payload: entryHash,
      provenance: cellData.cell_id[1]
    });
  }

  const removeLike = async (entryHash: EntryHashB64) => {

    const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'most_liked_posts')!;

    await appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: 'posts',
      fn_name: 'remove_like',
      payload: entryHash,
      provenance: cellData.cell_id[1]
    });
  }

</script>

<div class="post-container">

    <div class="title">
        <h3>{postWithLikes.post.title}</h3>
    </div>
    <div class="content">
        {postWithLikes.post.content}
    </div>
    <div class="post-footer">
      <mwc-icon on:click={() => {addLike(postWithLikes.entry_hash); dispatch("likes-updated")}}>expand_less</mwc-icon>
      {postWithLikes.likes}
      <mwc-icon on:click={() => {removeLike(postWithLikes.entry_hash); dispatch("likes-updated")}}>expand_more</mwc-icon>
    </div>

</div>



<style>
  .post-container {
    background-color: rgb(208, 223, 239);
    border-radius: 10px;
    display: flex;
    flex-direction: column;
    /* align-items: center; */
    width: 500px;
    /* text-align: center; */
    margin: 10px 0;
  }

  .post-footer {
    text-align: right;
    margin: 5px;
  }

  mwc-icon {
    cursor: pointer;
  }

  mwc-icon:hover {
    color: rgb(202, 202, 202);
  }
</style>