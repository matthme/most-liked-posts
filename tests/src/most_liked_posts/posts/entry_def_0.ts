
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../../utils';

export default (orchestrator: Orchestrator<any>) =>  {
  
  orchestrator.registerScenario("entry_def_0 CRUD tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player]: Player[] = await s.players([config, config]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player]);

    const alice = alice_happ.cells.find(cell => cell.cellRole.includes('/most_liked_posts.dna')) as Cell;
    const bob = bob_happ.cells.find(cell => cell.cellRole.includes('/most_liked_posts.dna')) as Cell;

    const entryContents = {
  "title": "what relationship laugh",
  "content": "always trying to save the planet? Hey, you know how I'm, like. God destroys dinosaurs."
};

    // Alice creates a entry_def_0
    const create_output = await alice.call(
        "posts",
        "create_entry_def_0",
        entryContents
    );
    t.ok(create_output.headerHash);
    t.ok(create_output.entryHash);

    await sleep(200);
    
    // Bob gets the created entry_def_0
    const entry = await bob.call("posts", "get_entry_def_0", create_output.entryHash);
    t.deepEqual(entry, entryContents);
    
    
    // Alice updates the entry_def_0
    const update_output = await alice.call(
      "posts",
      "update_entry_def_0",
      {
        originalHeaderHash: create_output.headerHash,
        updatedEntryDef0: {
          "title": "I'll go than",
  "content": "It's nice to play a character that has a soulful, dependent, close relationship. Do you need a manager? No matter how you travel, it's still you going."
}
      }
    );
    t.ok(update_output.headerHash);
    t.ok(update_output.entryHash);
    await sleep(200);

      
    
    // Alice delete the entry_def_0
    await alice.call(
      "posts",
      "delete_entry_def_0",
      create_output.headerHash
    );
    await sleep(200);

    
    // Bob tries to get the deleted entry_def_0, but he doesn't get it because it has been deleted
    const deletedEntry = await bob.call("posts", "get_entry_def_0", create_output.entryHash);
    t.notOk(deletedEntry);
      
  });

}
