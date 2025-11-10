import { beforeEach, describe, expect, it, vi } from "vitest";

import { renderWithQueryClient } from "@/components/ActivityBar/tests/test-utils";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutStates";
import { useGetSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useGetSidebarPanel";
import { useActivityBarStore } from "@/store/activityBar";
import { AppResizableLayoutStore, useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { ActivitybarPosition, SidebarPosition } from "@repo/moss-workspace";

import { ActivityBar } from "../ActivityBar";

vi.mock("@/store/activityBar");
vi.mock("@/store/appResizableLayout");
vi.mock("@/hooks/sharedStorage/layout/sidebar/useGetSidebarPanel");

const mockUseActivityBarStore = vi.mocked(useActivityBarStore);
const mockUseAppResizableLayoutStore = vi.mocked(useAppResizableLayoutStore);
const mockUseGetSidebarPanel = vi.mocked(useGetSidebarPanel);

describe("ActivityBar", () => {
  const setup = (
    activityBarPosition: ActivitybarPosition = ACTIVITYBAR_POSITION.DEFAULT,
    sidebarPositionValue: SidebarPosition = SIDEBAR_POSITION.LEFT
  ) => {
    mockUseActivityBarStore.mockReturnValue({
      position: activityBarPosition,
      items: [],
      lastActiveContainerId: null,
      setPosition: vi.fn(),
      setItems: vi.fn(),
      getActiveItem: vi.fn(),
      updateFromWorkspaceState: vi.fn(),
      setActiveItem: vi.fn(),
      toWorkspaceState: vi.fn(),
      resetToDefaults: vi.fn(),
    });

    mockUseAppResizableLayoutStore.mockImplementation((selector?: (state: AppResizableLayoutStore) => any) => {
      const mockState: AppResizableLayoutStore = {
        sideBarPosition: sidebarPositionValue,
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
      };

      if (selector) {
        return selector(mockState);
      }
      return mockState;
    });

    mockUseGetSidebarPanel.mockReturnValue({
      data: { position: sidebarPositionValue, size: 255, visible: true, minWidth: 130, maxWidth: 400 },
      isLoading: false,
      isError: false,
      error: null,
      isPending: false,
      isSuccess: true,
      status: "success",
      dataUpdatedAt: Date.now(),
      errorUpdatedAt: 0,
      failureCount: 0,
      failureReason: null,
      fetchStatus: "idle",
      isFetching: false,
      isFetched: true,
      isFetchedAfterMount: true,
      isInitialLoading: false,
      isPaused: false,
      isPlaceholderData: false,
      isRefetching: false,
      isStale: false,
      refetch: vi.fn(),
    } as any);

    return renderWithQueryClient(<ActivityBar />);
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("valid: should be hidden when position is HIDDEN", () => {
    const { container } = setup(ACTIVITYBAR_POSITION.HIDDEN);
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("hidden");
  });

  it("valid: should have correct classes for TOP position", () => {
    const { container } = setup(ACTIVITYBAR_POSITION.TOP);
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("w-full");
    expect(activityBar.getAttribute("class")).toContain("border-b");
  });

  it("valid: should have correct classes for BOTTOM position", () => {
    const { container } = setup(ACTIVITYBAR_POSITION.BOTTOM);
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("w-full");
    expect(activityBar.getAttribute("class")).toContain("border-t");
  });

  it("valid: should have correct classes for DEFAULT position", () => {
    const { container } = setup(ACTIVITYBAR_POSITION.DEFAULT);
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("h-full");
    expect(activityBar.getAttribute("class")).toContain("flex-col");
  });

  it("valid: should have correct classes for DEFAULT position when sidebar is on the right", () => {
    const { container } = setup(ACTIVITYBAR_POSITION.DEFAULT, SIDEBAR_POSITION.RIGHT);
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("h-full");
    expect(activityBar.getAttribute("class")).toContain("flex-col");
    expect(activityBar.getAttribute("class")).toContain("border-l");
  });
});
