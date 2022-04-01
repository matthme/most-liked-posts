<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
  import '@material/mwc-circular-progress';

  import { appWebsocketContext, appInfoContext } from './contexts';
  import CreatePost from './components/most_liked_posts/posts/CreatePost.svelte';
  import PostDetail from './components/most_liked_posts/posts/PostDetail.svelte';
  import PostDisplay from './components/most_liked_posts/posts/PostDisplay.svelte';
  import Tabs from './components/most_liked_posts/shared/Tabs.svelte';
  import type { PostWithLikes } from './types/most_liked_posts/posts';

  let appWebsocket: AppWebsocket | undefined;
  let appInfo: InstalledAppInfo | undefined;
  let loading: boolean = true;
  let entryHash: string | undefined;
  let allPosts: PostWithLikes[] = [];
  let mostPopularPosts: PostWithLikes[] = [];

  $: appWebsocket, appInfo, entryHash, loading;

  onMount(async () => {
    appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);

    appInfo = await appWebsocket.appInfo({
      installed_app_id: 'most-liked-posts2',
    });
    loading = false;
  });

  setContext(appWebsocketContext, {
    getAppWebsocket: () => appWebsocket,
  });

  setContext(appInfoContext, {
    getAppInfo: () => appInfo,
  });

  let tabs = ["Create Post", "All Posts", "Most Popular", "Custom Selection"];
  let activeTab = "Create Post";

  const getAllPosts = async (): Promise<void> =>  {

    const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'most_liked_posts')!;

    allPosts = await appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: 'posts',
      fn_name: 'get_all_posts_with_likes',
      payload: null,
      provenance: cellData.cell_id[1]
    });
  }


  const getMostPopularPosts = async (): Promise<void> => {

    const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'most_liked_posts')!;

    mostPopularPosts = await appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: 'posts',
      fn_name: 'get_n_most_popular',
      payload: 3,
      provenance: cellData.cell_id[1]
    });

  }

  const tabChange = async (e) => {
    activeTab = e.detail;

    if (activeTab === "All Posts") {
      await getAllPosts();
    }
    if (activeTab === "Most Popular") {
      await getMostPopularPosts();
    }
  }

</script>
<link href="https://fonts.googleapis.com/css?family=Material+Icons&display=block" rel="stylesheet">

<main style="display: flex; flex-direction: column; align-items: center; justify-content: center">
  {#if loading}

    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate />
    </div>

  {:else}

    <Tabs {tabs} {activeTab} on:tabChange={tabChange}></Tabs>

    {#if activeTab==="Create Post"}
      <CreatePost on:post-created="{e => entryHash = e.detail.entryHash}"></CreatePost>
    {:else if activeTab==="All Posts"}
      {#each allPosts as postWithLikes}
        <PostDisplay on:likes-updated={() => {setTimeout(getAllPosts, 500)} } {postWithLikes} {appInfo} {appWebsocket}></PostDisplay>
      {/each}
    {:else if activeTab==="Most Popular"}
      {#each mostPopularPosts as postWithLikes}
        <PostDisplay on:likes-updated={() => {setTimeout(getMostPopularPosts, 500)} } {postWithLikes} {appInfo} {appWebsocket}></PostDisplay>
      {/each}
    {:else}
      <div>Custom Selection</div>
    {/if}

  {/if}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
