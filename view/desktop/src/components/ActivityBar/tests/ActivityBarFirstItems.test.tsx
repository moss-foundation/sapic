import { beforeEach, describe, expect, it, vi } from "vitest";

import { ActivityBarItem, useActivityBarStore } from "@/store/activityBar";
import { AppResizableLayoutStore, useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { render } from "@testing-library/react";

import { ActivityBarFirstItems } from "../ActivityBarFirstItems";

vi.mock("@/store/activityBar");
vi.mock("@/store/appResizableLayout");
vi.mock("@atlaskit/pragmatic-drag-and-drop/element/adapter", () => ({
  monitorForElements: vi.fn(),
  draggable: vi.fn(() => () => {}),
  dropTargetForElements: vi.fn(() => () => {}),
}));
vi.mock("@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge", () => ({
  extractClosestEdge: vi.fn(),
}));

const useActivityBarStoreMock = vi.mocked(useActivityBarStore);
const useAppResizableLayoutStoreMock = vi.mocked(useAppResizableLayoutStore);
const monitorForElementsMock = vi.mocked(monitorForElements);
const extractClosestEdgeMock = vi.mocked(extractClosestEdge);

const MOCK_ITEMS: ActivityBarItem[] = [
  { id: "1", order: 1, icon: "Home", iconActive: "Home", title: "Home", isActive: true, isVisible: true },
  { id: "2", order: 2, icon: "Home", iconActive: "Home", title: "Home", isActive: false, isVisible: true },
  { id: "3", order: 3, icon: "Home", iconActive: "Home", title: "Home", isActive: false, isVisible: true },
  { id: "4", order: 4, icon: "Home", iconActive: "Home", title: "Home", isActive: false, isVisible: true },
];

describe("ActivityBarFirstItems", () => {
  let setItemsMock: ReturnType<typeof vi.fn>;
  let onDropHandler: ((payload: any) => void) | undefined;

  const createMockAppResizableLayoutStore = (): AppResizableLayoutStore => ({
    sideBarPosition: "LEFT",
    setSideBarPosition: vi.fn(),
    initialize: vi.fn(),
    sideBar: {
      minWidth: 130,
      maxWidth: 400,
      width: 255,
      visible: true,
      setWidth: vi.fn(),
      setVisible: vi.fn(),
    },
    bottomPane: {
      minHeight: 100,
      maxHeight: Infinity,
      height: 333,
      visible: false,
      setHeight: vi.fn(),
      setVisible: vi.fn(),
    },
  });

  const setupMocks = () => {
    setItemsMock = vi.fn();

    useActivityBarStoreMock.mockReturnValue({
      items: MOCK_ITEMS,
      position: "DEFAULT",
      lastActiveContainerId: null,
      setPosition: vi.fn(),
      setItems: setItemsMock,
      getActiveItem: vi.fn(),
      updateFromWorkspaceState: vi.fn(),
      setActiveItem: vi.fn(),
      toWorkspaceState: vi.fn(),
      resetToDefaults: vi.fn(),
    });

    useAppResizableLayoutStoreMock.mockImplementation((selector?: (state: AppResizableLayoutStore) => any) => {
      const mockState = createMockAppResizableLayoutStore();
      return selector ? selector(mockState) : mockState;
    });

    monitorForElementsMock.mockImplementation(({ onDrop: onDropCallback }) => {
      onDropHandler = onDropCallback;
      return () => {};
    });

    extractClosestEdgeMock.mockReturnValue(null);
  };

  const createDropPayload = (sourceData?: any, targetData?: any) => {
    return {
      location: {
        current: {
          dropTargets: targetData
            ? [
                {
                  data: targetData,
                },
              ]
            : [],
        },
      },
      source: { data: sourceData },
    };
  };

  const createActivityBarButtonData = (item: ActivityBarItem, edge?: Edge) => {
    return {
      type: "ActivityBarButton",
      data: item,
      edge,
    };
  };

  beforeEach(() => {
    vi.clearAllMocks();
    setupMocks();
  });

  it("valid: should reorder items when dropped successfully", () => {
    render(<ActivityBarFirstItems />);

    const sourceData = createActivityBarButtonData(MOCK_ITEMS[0]);
    const targetData = createActivityBarButtonData(MOCK_ITEMS[2]);
    const dropPayload = createDropPayload(sourceData, targetData);

    onDropHandler?.(dropPayload);

    expect(setItemsMock).toHaveBeenCalledTimes(1);
    const updatedItems = setItemsMock.mock.calls[0][0];
    expect(updatedItems.map((item: ActivityBarItem) => item.id)).toEqual(["3", "2", "1", "4"]);
  });

  it("valid: should reorder item when dropped successfully with top edge", () => {
    render(<ActivityBarFirstItems />);

    extractClosestEdgeMock.mockReturnValue("top");

    const sourceData = createActivityBarButtonData(MOCK_ITEMS[3]);
    const targetData = createActivityBarButtonData(MOCK_ITEMS[1], "top");
    const dropPayload = createDropPayload(sourceData, targetData);

    onDropHandler?.(dropPayload);

    expect(setItemsMock).toHaveBeenCalledTimes(1);
    const updatedItems = setItemsMock.mock.calls[0][0];
    expect(updatedItems.map((item: ActivityBarItem) => item.id)).toEqual(["1", "4", "2", "3"]);
  });

  it("valid: should reorder item when dropped successfully with bottom edge", () => {
    render(<ActivityBarFirstItems />);

    extractClosestEdgeMock.mockReturnValue("bottom");

    const sourceData = createActivityBarButtonData(MOCK_ITEMS[3]);
    const targetData = createActivityBarButtonData(MOCK_ITEMS[1]);
    const dropPayload = createDropPayload(sourceData, targetData);

    onDropHandler?.(dropPayload);

    expect(setItemsMock).toHaveBeenCalledTimes(1);
    const updatedItems = setItemsMock.mock.calls[0][0];
    expect(updatedItems.map((item: ActivityBarItem) => item.id)).toEqual(["1", "2", "4", "3"]);
  });

  it("invalid: should not reorder when no drop target is provided", () => {
    render(<ActivityBarFirstItems />);

    const sourceData = createActivityBarButtonData(MOCK_ITEMS[0]);
    const dropPayload = createDropPayload(sourceData);

    onDropHandler?.(dropPayload);

    expect(setItemsMock).not.toHaveBeenCalled();
  });

  it("invalid: should not reorder when no source data is provided", () => {
    render(<ActivityBarFirstItems />);

    const targetData = createActivityBarButtonData(MOCK_ITEMS[2]);
    const dropPayload = createDropPayload(undefined, targetData);

    onDropHandler?.(dropPayload);

    expect(setItemsMock).not.toHaveBeenCalled();
  });

  it("invalid: should not reorder when drop targets array is empty", () => {
    render(<ActivityBarFirstItems />);

    const sourceData = createActivityBarButtonData(MOCK_ITEMS[0]);
    const dropPayload = createDropPayload(sourceData);

    onDropHandler?.(dropPayload);

    expect(setItemsMock).not.toHaveBeenCalled();
  });
});
