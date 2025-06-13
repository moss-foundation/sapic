import { beforeEach, describe, expect, it, vi } from "vitest";

import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActivityBarStore } from "@/store/activityBar";
import { AppResizableLayoutStore, useAppResizableLayoutStore } from "@/store/appResizableLayout";
import { render } from "@testing-library/react";

import { ActivityBar } from "../ActivityBar";

vi.mock("@/store/activityBar");
vi.mock("@/store/appResizableLayout");

const mockUseActivityBarStore = vi.mocked(useActivityBarStore);
const mockUseAppResizableLayoutStore = vi.mocked(useAppResizableLayoutStore);

describe("ActivityBar", () => {
  const setup = (
    activityBarPosition: keyof typeof ACTIVITYBAR_POSITION = "DEFAULT",
    sidebarPositionValue: keyof typeof SIDEBAR_POSITION = "LEFT"
  ) => {
    mockUseActivityBarStore.mockReturnValue({
      position: ACTIVITYBAR_POSITION[activityBarPosition],
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

    return render(<ActivityBar />);
  };

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("valid: should be hidden when position is HIDDEN", () => {
    const { container } = setup("HIDDEN");
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("hidden");
  });

  it("valid: should have correct classes for TOP position", () => {
    const { container } = setup("TOP");
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("w-full");
    expect(activityBar.getAttribute("class")).toContain("border-b");
  });

  it("valid: should have correct classes for BOTTOM position", () => {
    const { container } = setup("BOTTOM");
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("w-full");
    expect(activityBar.getAttribute("class")).toContain("border-t");
  });

  it("valid: should have correct classes for DEFAULT position", () => {
    const { container } = setup("DEFAULT");
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("h-full");
    expect(activityBar.getAttribute("class")).toContain("flex-col");
  });

  it("valid: should have correct classes for DEFAULT position when sidebar is on the right", () => {
    const { container } = setup("DEFAULT", "RIGHT");
    const activityBar = container.firstChild as HTMLElement;

    expect(activityBar).not.toBeNull();
    expect(activityBar.getAttribute("class")).toContain("h-full");
    expect(activityBar.getAttribute("class")).toContain("flex-col");
    expect(activityBar.getAttribute("class")).toContain("border-l");
  });
});
