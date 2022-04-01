<script lang="ts">
  import { createEventDispatcher, getContext } from 'svelte';
  import '@material/mwc-button';
  import type { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';

  import { appWebsocketContext, appInfoContext } from '../../../contexts';
  import type { NewPostInput } from '../../../types/most_liked_posts/posts';
  import '@type-craft/title/create-title';
  import '@type-craft/content/create-content';
  import '@type-craft/date-time/create-date-time';

  let appInfo = getContext(appInfoContext).getAppInfo();
  let appWebsocket = getContext(appWebsocketContext).getAppWebsocket();

  const dispatch = createEventDispatcher();

  let title: string | undefined;
  let content: string | undefined;
  let author: string | undefined;
  let timestamp: number | undefined;

  $: title, content, author, timestamp;

  async function createPost() {
    const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'most_liked_posts')!;


    const postInput: NewPostInput = {
      title: title!,
      content: content!,
    };


    const { entryHash } = await appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: 'posts',
      fn_name: 'create_post',
      payload: postInput,
      provenance: cellData.cell_id[1]
    });

    dispatch('post-created', { entryHash });
  }

</script>

<div style="display: flex; flex-direction: column; align-items: center;">
  <span style="font-size: 18px">Create Post</span>

  <input placeholder="Title"
      bind:value={title}
      on:change="{e => title = e.target.value}"
      style="margin-top: 16px"
    >

  <textarea bind:value={content} placeholder="Content" rows=10 cols=50
      on:change={event => content = event.target.value}
      style="margin-top: 16px"
    ></textarea>


  <mwc-button
    label="Create Post"
    disabled={!(title && content)}
    on:click="{() => {createPost(); content=""; title="";}}"
  ></mwc-button>
</div>


<style>
  textarea {
    max-width: 700px;
  }
</style>