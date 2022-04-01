<script>
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let tabs;
    export let activeTab;
</script>

<div class="tabs">
    <ul>
        {#each tabs as tab}
            <li on:click={() => dispatch('tabChange', tab)}>
                <div class:active={tab===activeTab}>{tab}</div>
            </li>
        {/each}
    </ul>
</div>


<style>
    .tabs {
        font-size: larger;
        margin-bottom: 40px;
    }
    ul {
        display: flex;
        justify-content: center;
        padding: 0;
        list-style-type: none;
    }
    li {
        margin: 0 16px;
        cursor: pointer;
    }
    .active{
        border-bottom: 3px solid rgb(0, 0, 94);
    }

</style>


