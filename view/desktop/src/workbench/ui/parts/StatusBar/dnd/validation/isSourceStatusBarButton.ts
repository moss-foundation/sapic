import { ElementDragPayload } from "@atlaskit/pragmatic-drag-and-drop/dist/types/internal-types";

import { STATUS_BAR_BUTTON_DND_TYPE } from "../../constants";

export const isSourceStatusBarButton = (source: ElementDragPayload) => {
  return source.data.type === STATUS_BAR_BUTTON_DND_TYPE;
};
