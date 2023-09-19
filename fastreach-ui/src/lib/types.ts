import type { GeoJsonObject } from "geojson";

interface IsochroneResponse {
    area: number;
    diameter: number;
    geometry: GeoJsonObject;
}

type IsochroneResponseHandler = (res: IsochroneResponse) => void;

export type { IsochroneResponse, IsochroneResponseHandler };
