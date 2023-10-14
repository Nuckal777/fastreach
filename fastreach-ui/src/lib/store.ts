import { writable } from "svelte/store";
import type { NodeResponse } from "./types";

export const nodes = writable<NodeResponse>([]);
