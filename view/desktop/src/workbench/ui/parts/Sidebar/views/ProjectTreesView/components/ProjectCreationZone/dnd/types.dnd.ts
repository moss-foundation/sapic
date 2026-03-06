import { PROJECT_CREATION_ZONE_TYPE } from "../constants";

export interface DropProjectCreationZoneData {
  type: typeof PROJECT_CREATION_ZONE_TYPE;
  [key: string | symbol]: unknown;
}
