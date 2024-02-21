import type { GeoJsonObject } from "geojson";

interface IsochroneConfiguration {
    nodes: Node[],
    start: Date,
    minutes: number,
}

interface IsochroneCall {
    request: IsochroneRequest;
    response: IsochroneResponse;
    name: string,
    lat: number,
    lng: number,
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

interface Node {
    name: string,
    coords: number[],
    id: string,
}

type NodeResponse = Node[];

interface NodeResponseState {
    response: NodeResponse
    error: string,
}

export type {
    IsochroneConfiguration,
    IsochroneCall,
    IsochroneRequest,
    IsochroneResponse,
    IsochroneCallHandler,
    Node,
    NodeResponse,
    NodeResponseState,
};
