<script lang="ts">
  import { onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import '@material/mwc-icon';
  import type { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
  import { appInfoContext, appWebsocketContext } from '../../../contexts';
  import type { Post } from '../../../types/most_liked_posts/posts';
  import '@type-craft/title/title-detail';
  import '@type-craft/content/content-detail';
  import '@holochain-open-dev/utils/copiable-hash';
  import '@type-craft/date-time/date-time-detail';

  export let entryHash: string;

  let appInfo = getContext(appInfoContext).getAppInfo();
  let appWebsocket = getContext(appWebsocketContext).getAppWebsocket();

  let post: Post | undefined;
  let entryRanking: number = 0;

  $: post;
  $: entryRanking;

  onMount(async () => {
    const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'most_liked_posts')!;

    post = await appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: 'posts',
      fn_name: 'get_post',
      payload: entryHash,
      provenance: cellData.cell_id[1]
    });

  });

</script>

{#if post}
  <div class="post-container">
    <div class="title">
      <h3>{post.title}</h3>
    </div>
    <div class="content">
      {post.content}
    </div>
    <div class="post-footer">
      <mwc-icon>expand_more</mwc-icon>
      {entryRanking}
      <mwc-icon>expand_less</mwc-icon>
    </div>
  </div>
{:else}
  <div style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{/if}



<style>
  .post-container {
    background-color: rgb(208, 223, 239);
    border-radius: 10px;
    display: flex;
    flex-direction: column;
    /* align-items: center; */
    max-width: 500px;
    /* text-align: center; */
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