import type { GeoJsonObject } from "geojson";

interface IsochroneCall {
    request: IsochroneRequest;
    response: IsochroneResponse;
}

interface IsochroneRequest {
    name: string;
    minutes: number;
    start: Date;
}

interface IsochroneResponse {
    area: number;
    diameter: number;
    geometry: GeoJsonObject;
}

type IsochroneCallHandler = (res: IsochroneCall) => void;

export type {
    IsochroneCall,
    IsochroneRequest,
    IsochroneResponse,
    IsochroneCallHandler,
};
