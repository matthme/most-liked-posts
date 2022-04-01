import { Config, InstallAgentsHapps } from '@holochain/tryorama';
import path from 'path'
import { fileURLToPath } from "url";
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const mostLikedPostsDna = path.join(__dirname, "../../dna/workdir/most_liked_posts.dna");


export const config = Config.gen();

export const installation: InstallAgentsHapps = [
  // one agent
  [
    [
      mostLikedPostsDna, // contains this dna
    ]
  ]
];

export const sleep = (ms: number) => new Promise(resolve => setTimeout(() => resolve(null), ms));
