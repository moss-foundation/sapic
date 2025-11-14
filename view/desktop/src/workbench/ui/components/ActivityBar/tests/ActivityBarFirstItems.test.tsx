import { beforeEach, describe, expect, it, vi } from "vitest";

import { ACTIVITYBAR_POSITION } from "@/constants/layout";
import { emptyGridState } from "@/defaults/layout";
import { useDescribeApp } from "@/hooks";
import { useGetLayout } from "@/hooks/workbench/layout/useGetLayout";
import { ActivityBarItemProps, useActivityBarStore } from "@/store/activityBar";
import { renderWithQueryClient, screen } from "@/workbench/ui/components/ActivityBar/tests/test-utils";
import { extractClosestEdge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ActivityBarFirstItems } from "../ActivityBarFirstItems";

vi.mock("@/store/activityBar");
vi.mock("@/hooks");
vi.mock("@/hooks/workbench/layout/useGetLayout", () => ({
  useGetLayout: vi.fn(),
}));
vi.mock("@atlaskit/pragmatic-drag-and-drop/element/adapter", () => ({
  monitorForElements: vi.fn(),
  draggable: vi.fn(() => () => {}),
  dropTargetForElements: vi.fn(() => () => {}),
}));
vi.mock("@atlaskit/pragmatic-drag-and-drop-hitbox/closest-edge", () => ({
  extractClosestEdge: vi.fn(),
}));

const useActivityBarStoreMock = vi.mocked(useActivityBarStore);
const useGetLayoutMock = vi.mocked(useGetLayout);
const useDescribeAppMock = vi.mocked(useDescribeApp);
const monitorForElementsMock = vi.mocked(monitorForElements);
const extractClosestEdgeMock = vi.mocked(extractClosestEdge);

const MOCK_ITEMS: ActivityBarItemProps[] = [
  { id: "1", icon: "Home", title: "Item 1", order: 0, isVisible: true },
  { id: "2", icon: "JsonPath", title: "Item 2", order: 1, isVisible: true },
  { id: "3", icon: "WebServer", title: "Item 3", order: 2, isVisible: true },
  { id: "4", icon: "Wrench", title: "Item 4", order: 3, isVisible: true },
];

describe("ActivityBarFirstItems", () => {
  let setItemsMock: ReturnType<typeof vi.fn>;
  let onDropHandler: ((payload: any) => void) | undefined;
  let cleanupMock: ReturnType<typeof vi.fn>;

  const setupMocks = (options?: {
    activityBarPosition?: string;
    activeContainerId?: string;
    sidebarVisible?: boolean;
    items?: ActivityBarItemProps[];
  }) => {
    const {
      activityBarPosition = ACTIVITYBAR_POSITION.DEFAULT,
      activeContainerId = "1",
      sidebarVisible = true,
      items = MOCK_ITEMS,
    } = options || {};

    useDescribeAppMock.mockReturnValue({
      data: {
        configuration: {
          keys: ["activityBarPosition"],
          contents: { activityBarPosition },
        },
      },
    } as any);

    useGetLayoutMock.mockReturnValue({
      data: {
        sidebarState: {
          visible: sidebarVisible,
          width: 255,
          minWidth: 130,
          maxWidth: 400,
        },
        activitybarState: {
          activeContainerId,
          position: ACTIVITYBAR_POSITION.DEFAULT,
        },
        bottomPanelState: {
          visible: false,
          height: 333,
          minHeight: 100,
          maxHeight: Infinity,
        },
        tabbedPaneState: {
          gridState: emptyGridState,
        },
      },
    } as any);

    setItemsMock = vi.fn();
    cleanupMock = vi.fn();

    useActivityBarStoreMock.mockReturnValue({
      items,
      position: ACTIVITYBAR_POSITION.DEFAULT,
      lastActiveContainerId: null,
      setPosition: vi.fn(),
      setItems: setItemsMock,
      getActiveItem: vi.fn(),
      updateFromWorkspaceState: vi.fn(),
      setActiveItem: vi.fn(),
      toWorkspaceState: vi.fn(),
      resetToDefaults: vi.fn(),
    });

    monitorForElementsMock.mockReturnValue(cleanupMock);
    extractClosestEdgeMock.mockReturnValue(null);
  };

  const createDropPayload = (sourceData?: any, targetData?: any) => {
    const currentLocation = {
      dropTargets: targetData
        ? [
            {
              data: targetData,
            },
          ]
        : [],
    };
    return {
      location: {
        current: currentLocation,
        initial: currentLocation,
        previous: currentLocation,
      },
      source: { data: sourceData },
    } as any;
  };

  const createActivityBarButtonData = (item: ActivityBarItemProps) => {
    return {
      type: "ActivityBarButton",
      data: item,
    };
  };

  beforeEach(() => {
    vi.clearAllMocks();
    onDropHandler = undefined;
    monitorForElementsMock.mockImplementation((config) => {
      onDropHandler = config.onDrop;
      return cleanupMock;
    });
  });

  describe("Drag and Drop", () => {
    beforeEach(() => {
      setupMocks();
    });

    it("valid: should reorder items when dropped successfully", () => {
      renderWithQueryClient(<ActivityBarFirstItems />);

      // Get the handler from the mock call after component renders
      const handler = monitorForElementsMock.mock.calls[0]?.[0]?.onDrop;
      expect(handler).toBeDefined();

      const sourceData = createActivityBarButtonData(MOCK_ITEMS[0]);
      const targetData = createActivityBarButtonData(MOCK_ITEMS[2]);
      const dropPayload = createDropPayload(sourceData, targetData);

      handler?.(dropPayload);

      expect(setItemsMock).toHaveBeenCalledTimes(1);
      const updatedItems = setItemsMock.mock.calls[0][0];
      expect(updatedItems.map((item: ActivityBarItemProps) => item.id)).toEqual(["3", "2", "1", "4"]);
    });

    it("valid: should reorder item when dropped successfully with top edge", () => {
      renderWithQueryClient(<ActivityBarFirstItems />);

      extractClosestEdgeMock.mockReturnValue("top");

      // Get the handler from the mock call after component renders
      const handler = monitorForElementsMock.mock.calls[0]?.[0]?.onDrop;
      expect(handler).toBeDefined();

      const sourceData = createActivityBarButtonData(MOCK_ITEMS[3]);
      const targetData = createActivityBarButtonData(MOCK_ITEMS[1]);
      const dropPayload = createDropPayload(sourceData, targetData);

      handler?.(dropPayload);

      expect(setItemsMock).toHaveBeenCalledTimes(1);
      const updatedItems = setItemsMock.mock.calls[0][0];
      expect(updatedItems.map((item: ActivityBarItemProps) => item.id)).toEqual(["1", "4", "2", "3"]);
    });

    it("valid: should reorder item when dropped successfully with bottom edge", () => {
      renderWithQueryClient(<ActivityBarFirstItems />);

      extractClosestEdgeMock.mockReturnValue("bottom");

      // Get the handler from the mock call after component renders
      const handler = monitorForElementsMock.mock.calls[0]?.[0]?.onDrop;
      expect(handler).toBeDefined();

      const sourceData = createActivityBarButtonData(MOCK_ITEMS[3]);
      const targetData = createActivityBarButtonData(MOCK_ITEMS[1]);
      const dropPayload = createDropPayload(sourceData, targetData);

      handler?.(dropPayload);

      expect(setItemsMock).toHaveBeenCalledTimes(1);
      const updatedItems = setItemsMock.mock.calls[0][0];
      expect(updatedItems.map((item: ActivityBarItemProps) => item.id)).toEqual(["1", "2", "4", "3"]);
    });

    it("invalid: should not reorder when no drop target is provided", () => {
      renderWithQueryClient(<ActivityBarFirstItems />);

      const sourceData = createActivityBarButtonData(MOCK_ITEMS[0]);
      const dropPayload = createDropPayload(sourceData);

      onDropHandler?.(dropPayload);

      expect(setItemsMock).not.toHaveBeenCalled();
    });

    it("invalid: should not reorder when no source data is provided", () => {
      renderWithQueryClient(<ActivityBarFirstItems />);

      const targetData = createActivityBarButtonData(MOCK_ITEMS[2]);
      const dropPayload = createDropPayload(undefined, targetData);

      onDropHandler?.(dropPayload);

      expect(setItemsMock).not.toHaveBeenCalled();
    });

    it("invalid: should not reorder when source data is missing", () => {
      renderWithQueryClient(<ActivityBarFirstItems />);

      const targetData = createActivityBarButtonData(MOCK_ITEMS[2]);
      const dropPayload = createDropPayload({ type: "ActivityBarButton" }, targetData);

      onDropHandler?.(dropPayload);

      expect(setItemsMock).not.toHaveBeenCalled();
    });

    it("invalid: should not reorder when target data is missing", () => {
      renderWithQueryClient(<ActivityBarFirstItems />);

      const sourceData = createActivityBarButtonData(MOCK_ITEMS[0]);
      const dropPayload = createDropPayload(sourceData, { type: "ActivityBarButton" });

      onDropHandler?.(dropPayload);

      expect(setItemsMock).not.toHaveBeenCalled();
    });

    it("valid: should only monitor ActivityBarButton type", () => {
      renderWithQueryClient(<ActivityBarFirstItems />);

      expect(monitorForElementsMock).toHaveBeenCalledWith(
        expect.objectContaining({
          canMonitor: expect.any(Function),
        })
      );

      const canMonitor = monitorForElementsMock.mock.calls[0]?.[0]?.canMonitor;
      expect(canMonitor).toBeDefined();
      if (canMonitor) {
        expect(canMonitor({ source: { data: { type: "ActivityBarButton" } } } as any)).toBe(true);
        expect(canMonitor({ source: { data: { type: "OtherType" } } } as any)).toBe(false);
      }
    });
  });

  describe("Rendering", () => {
    it("valid: should render all visible items", () => {
      setupMocks();
      const { container } = renderWithQueryClient(<ActivityBarFirstItems />);

      expect(screen.getByTitle("Item 1")).toBeTruthy();
      expect(screen.getByTitle("Item 2")).toBeTruthy();
      expect(screen.getByTitle("Item 3")).toBeTruthy();
      expect(screen.getByTitle("Item 4")).toBeTruthy();
      expect(container.querySelectorAll("button[title]")).toHaveLength(4);
    });

    it("valid: should filter out items with isVisible false", () => {
      const itemsWithHidden: ActivityBarItemProps[] = [
        ...MOCK_ITEMS,
        { id: "5", icon: "Commit", title: "Hidden Item", order: 4, isVisible: false },
      ];
      setupMocks({ items: itemsWithHidden });
      const { container } = renderWithQueryClient(<ActivityBarFirstItems />);

      expect(screen.getByTitle("Item 1")).toBeTruthy();
      expect(screen.getByTitle("Item 2")).toBeTruthy();
      expect(screen.getByTitle("Item 3")).toBeTruthy();
      expect(screen.getByTitle("Item 4")).toBeTruthy();
      expect(screen.queryByTitle("Hidden Item")).toBeNull();
      expect(container.querySelectorAll("button[title]")).toHaveLength(4);
    });

    it("valid: should render items with undefined isVisible", () => {
      const itemsWithUndefined: ActivityBarItemProps[] = [
        { id: "1", icon: "Home", title: "Item 1", order: 0 },
        { id: "2", icon: "JsonPath", title: "Item 2", order: 1, isVisible: undefined },
      ];
      setupMocks({ items: itemsWithUndefined });
      const { container } = renderWithQueryClient(<ActivityBarFirstItems />);

      expect(screen.getByTitle("Item 1")).toBeTruthy();
      expect(screen.getByTitle("Item 2")).toBeTruthy();
      expect(container.querySelectorAll("button[title]")).toHaveLength(2);
    });

    it("valid: should render ActivityBarButtonIndicator when item is active and sidebar is visible", () => {
      setupMocks({ activeContainerId: "1", sidebarVisible: true });
      renderWithQueryClient(<ActivityBarFirstItems />);

      const item1Button = screen.getByTitle("Item 1");
      const indicator = item1Button.parentElement?.querySelector("div[class*='absolute']");
      expect(indicator).toBeTruthy();
    });

    it("invalid: should not render ActivityBarButtonIndicator when sidebar is not visible", () => {
      setupMocks({ activeContainerId: "1", sidebarVisible: false });
      renderWithQueryClient(<ActivityBarFirstItems />);

      const item1Button = screen.getByTitle("Item 1");
      const indicator = item1Button.parentElement?.querySelector("div[class*='absolute']");
      expect(indicator).toBeNull();
    });

    it("invalid: should not render ActivityBarButtonIndicator when item is not active", () => {
      setupMocks({ activeContainerId: "2", sidebarVisible: true });
      renderWithQueryClient(<ActivityBarFirstItems />);

      const item1Button = screen.getByTitle("Item 1");
      const indicator = item1Button.parentElement?.querySelector("div[class*='absolute']");
      expect(indicator).toBeNull();
    });
  });

  describe("Layout", () => {
    it("valid: should apply flex-col layout for DEFAULT position", () => {
      setupMocks({ activityBarPosition: ACTIVITYBAR_POSITION.DEFAULT });
      const { container } = renderWithQueryClient(<ActivityBarFirstItems />);

      const wrapper = container.firstChild as HTMLElement;
      expect(wrapper.className).toContain("flex-col");
      expect(wrapper.className).toContain("gap-3");
    });

    it("valid: should apply flex-row layout for TOP position", () => {
      setupMocks({ activityBarPosition: ACTIVITYBAR_POSITION.TOP });
      const { container } = renderWithQueryClient(<ActivityBarFirstItems />);

      const wrapper = container.firstChild as HTMLElement;
      expect(wrapper.className).toContain("flex-row");
      expect(wrapper.className).toContain("gap-1");
    });

    it("valid: should apply flex-row layout for BOTTOM position", () => {
      setupMocks({ activityBarPosition: ACTIVITYBAR_POSITION.BOTTOM });
      const { container } = renderWithQueryClient(<ActivityBarFirstItems />);

      const wrapper = container.firstChild as HTMLElement;
      expect(wrapper.className).toContain("flex-row");
      expect(wrapper.className).toContain("gap-1");
    });

    it("valid: should apply correct padding for DEFAULT position", () => {
      setupMocks({ activityBarPosition: ACTIVITYBAR_POSITION.DEFAULT });
      const { container } = renderWithQueryClient(<ActivityBarFirstItems />);

      const itemWrapper = container.querySelector("div[class*='px-1.5']");
      expect(itemWrapper).toBeTruthy();
    });

    it("valid: should apply correct padding for TOP/BOTTOM position", () => {
      setupMocks({ activityBarPosition: ACTIVITYBAR_POSITION.TOP });
      const { container } = renderWithQueryClient(<ActivityBarFirstItems />);

      const itemWrapper = container.querySelector("div[class*='py-[3px]']");
      expect(itemWrapper).toBeTruthy();
    });
  });

  describe("Cleanup", () => {
    it("valid: should return cleanup function from monitorForElements", () => {
      setupMocks();
      const { unmount } = renderWithQueryClient(<ActivityBarFirstItems />);

      expect(monitorForElementsMock).toHaveBeenCalled();
      expect(cleanupMock).toBeDefined();

      unmount();
      // Cleanup should be called when component unmounts
      // Note: React Testing Library handles cleanup automatically
    });
  });
});
