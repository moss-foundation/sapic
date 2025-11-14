import { beforeEach, describe, expect, it, vi } from "vitest";

import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layout";
import { emptyGridState } from "@/defaults/layout";
import { useDescribeApp } from "@/hooks/app/useDescribeApp";
import { useGetLayout } from "@/hooks/workbench/layout/useGetLayout";
import { useActivityBarStore } from "@/store/activityBar";
import { renderWithQueryClient } from "@/workbench/ui/components/ActivityBar/tests/test-utils";
import { ActivitybarPosition, SidebarPosition } from "@repo/moss-workspace";

import { ActivityBar } from "../ActivityBar";

vi.mock("@/store/activityBar");
vi.mock("@/hooks");
vi.mock("@/hooks/app/useDescribeApp");
vi.mock("@/hooks/workbench/layout/useGetLayout", () => ({
  useGetLayout: vi.fn(),
}));

const mockUseActivityBarStore = vi.mocked(useActivityBarStore);
const mockUseGetLayout = vi.mocked(useGetLayout);
const mockUseDescribeApp = vi.mocked(useDescribeApp);

describe("ActivityBar", () => {
  beforeEach(() => {
    vi.clearAllMocks();

    // Set up default mocks
    mockUseActivityBarStore.mockReturnValue({
      items: [],
      setItems: vi.fn(),
    });

    mockUseGetLayout.mockReturnValue({
      data: {
        sidebarState: {
          visible: true,
          width: 255,
          minWidth: 130,
          maxWidth: 400,
        },
        activitybarState: {
          activeContainerId: "1",
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
  });

  const setup = (
    activityBarPosition: ActivitybarPosition = ACTIVITYBAR_POSITION.DEFAULT,
    sideBarPositionValue: SidebarPosition = SIDEBAR_POSITION.LEFT
  ) => {
    mockUseActivityBarStore.mockReturnValue({
      items: [],
      setItems: vi.fn(),
    });

    mockUseDescribeApp.mockReturnValue({
      data: {
        configuration: {
          keys: ["activityBarPosition", "sideBarPosition"],
          contents: {
            activityBarPosition,
            sideBarPosition: sideBarPositionValue,
          },
        },
      },
    } as any);

    return renderWithQueryClient(<ActivityBar />);
  };

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
