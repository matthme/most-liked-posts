import { Orchestrator } from "@holochain/tryorama";

import posts_entry_def_0 from './most_liked_posts/posts/entry_def_0';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
posts_entry_def_0(orchestrator);
orchestrator.run();



