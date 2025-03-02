import React from "react";
import { vi } from "vitest";

/**
 * useful utility type to erase readonly signatures for testing purposes
 *
 * @see https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-4.html#readonly-mapped-type-modifiers-and-readonly-arrays
 */
export type Writable<T> = T extends object ? { -readonly [K in keyof T]: Writable<T[K]> } : T;

export function setMockRefElement(node: Partial<HTMLElement>) {
  const mockRef = {
    get current() {
      return node;
    },
    set current(_value) {
      //noop
    },
  };

  return vi.spyOn(React, "useRef").mockReturnValueOnce(mockRef);
}

export function createOffsetDragOverEvent(params: { clientX: number; clientY: number }): Event {
  const event = new Event("dragover", {
    bubbles: true,
    cancelable: true,
  });
  Object.defineProperty(event, "clientX", { get: () => params.clientX });
  Object.defineProperty(event, "clientY", { get: () => params.clientY });
  return event;
}

export function exhaustMicrotaskQueue(): Promise<void> {
  return new Promise<void>((resolve) => resolve());
}

export const mockGetBoundingClientRect = ({
  left,
  top,
  height,
  width,
}: {
  left: number;
  top: number;
  height: number;
  width: number;
}) => {
  const result = {
    left,
    top,
    height,
    width,
    right: left + width,
    bottom: top + height,
    x: left,
    y: top,
  };
  return {
    ...result,
    toJSON: () => result,
  };
};
