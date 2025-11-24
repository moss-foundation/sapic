import type { OpenInTarget } from "@repo/ipc";

export enum OpenInTargetEnum {
  NEW_WINDOW = "NEW_WINDOW",
  CURRENT_WINDOW = "CURRENT_WINDOW",
}

const _ensureAllValuesForOpenInTarget: {
  [K in OpenInTarget]: OpenInTargetEnum;
} = {
  NEW_WINDOW: OpenInTargetEnum.NEW_WINDOW,
  CURRENT_WINDOW: OpenInTargetEnum.CURRENT_WINDOW,
};
void _ensureAllValuesForOpenInTarget;
