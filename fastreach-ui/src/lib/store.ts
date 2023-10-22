import { writable } from "svelte/store";
import type { NodeResponseState } from "./types";

export const nodes = writable<NodeResponseState>({
    error: "",
    response: [],
});
