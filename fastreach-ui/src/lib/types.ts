import type { GeoJsonObject } from "geojson";

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
