import { writable } from "svelte/store";
import type { NodeResponseState } from "./types";

interface MapLocation {
    lat: number,
    lng: number,
    zoom: number,
}

export const nodes = writable<NodeResponseState>({
    error: "",
    response: [],
});

export const mapLocation = writable<MapLocation>({
    lat: 50.97,
    lng: 11.035,
    zoom: 11,
});
