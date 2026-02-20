import { STATUS_BAR_BUTTON_DND_TYPE } from "../constants";
import { StatusBarItem } from "../types";

export interface StatusBarButtonDragData {
  type: typeof STATUS_BAR_BUTTON_DND_TYPE;
  data: StatusBarItem;
  [key: string]: unknown;
}
