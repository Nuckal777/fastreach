import type { GeoJsonObject } from "geojson";

interface DatasetInfo {
  name: string;
  nodes: number;
  edges: number;
  from: string;
  to: string;
}

interface IndexedDataset {
  name: string;
  nodes: number;
  edges: number;
  from: Date;
  to: Date;
  index: number;
}

interface DatasetResponse {
  datasets: DatasetInfo[];
}

interface DatasetResponseState {
  error: string;
  response: DatasetResponse;
}

interface IndexedDatasetResponseState {
  error: string;
  datasets: IndexedDataset[];
}

interface IsochroneConfiguration {
  nodes: Node[];
  start: Date;
  minutes: number;
}

interface IsochroneCall {
  request: IsochroneRequest;
  response: IsochroneResponse;
  name: string;
  lat: number;
  lng: number;
}

interface IsochroneRequest {
  id: string;
  minutes: number;
  start: Date;
}

interface IsochroneResponse {
  area: number;
  diameter: number;
  geometry: GeoJsonObject;
}

type IsochroneCallHandler = (res: IsochroneCall) => void;

interface MapLocation {
  lat: number;
  lng: number;
  zoom: number;
}

interface Node {
  name: string;
  coords: number[];
  id: string;
}

type NodeResponse = Node[];

interface NodeResponseState {
  response: NodeResponse;
  error: string;
}

enum FilterState {
  Empty,
  Match,
  Ambiguous,
}

export {
  FilterState,
  type DatasetInfo,
  type IndexedDataset,
  type DatasetResponse,
  type DatasetResponseState,
  type IndexedDatasetResponseState,
  type IsochroneConfiguration,
  type IsochroneCall,
  type IsochroneRequest,
  type IsochroneResponse,
  type IsochroneCallHandler,
  type MapLocation,
  type Node,
  type NodeResponse,
  type NodeResponseState,
};
