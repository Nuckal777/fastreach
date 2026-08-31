import { writable } from "svelte/store";
import type { MapLocation } from "./types";

export const mapLocation = writable<MapLocation>({
  lat: 50.97,
  lng: 11.035,
  zoom: 11,
});
