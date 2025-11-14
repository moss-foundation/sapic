import { beforeEach, describe, expect, it, vi } from "vitest";

import { ACTIVITYBAR_POSITION } from "@/constants/layout";
import { ActivityBarItemProps, useActivityBarStore } from "@/store/activityBar";
import { renderWithQueryClient } from "@/workbench/ui/components/ActivityBar/tests/test-utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { ActivityBarButton } from "../ActivityBarButton";

vi.mock("@/hooks/workbench/layout/useGetLayout", () => ({
  useGetLayout: vi.fn(() => ({
    data: {
      sidebarState: { visible: true },
      activitybarState: { activeContainerId: "test", position: ACTIVITYBAR_POSITION.DEFAULT },
    },
  })),
}));
vi.mock("@atlaskit/pragmatic-drag-and-drop/element/adapter", async (importOriginal) => {
  const originalModule = await (importOriginal() as Promise<
    typeof import("@atlaskit/pragmatic-drag-and-drop/element/adapter")
  >);
  return {
    ...originalModule,
    draggable: vi.fn(() => () => {}),
    dropTargetForElements: vi.fn(() => () => {}),
  };
});

vi.mock("@/store/activityBar");

const mockedDropTarget = vi.mocked(dropTargetForElements);
const mockedUseActivityBarStore = vi.mocked(useActivityBarStore);

describe("ActivityBarButton › dropTargetForElements", () => {
  const defaultProps: ActivityBarItemProps = {
    id: "test",
    icon: "Add",
    iconActive: "AddCircleActive",
    title: "Test Button",
    order: 1,
  };

  const setupMocks = () => {
    mockedUseActivityBarStore.mockReturnValue({
      position: ACTIVITYBAR_POSITION.DEFAULT,
      items: [],
      lastActiveContainerId: null,
      setPosition: vi.fn(),
      setItems: vi.fn(),
      updateFromWorkspaceState: vi.fn(),
      toWorkspaceState: vi.fn(),
      resetToDefaults: vi.fn(),
    });
  };

  beforeEach(() => {
    vi.clearAllMocks();
    setupMocks();
  });

  const renderComponent = () => renderWithQueryClient(<ActivityBarButton {...defaultProps} />);

  const getCanDropFunction = () => {
    renderComponent();
    const canDrop = mockedDropTarget.mock.calls[0][0].canDrop;
    expect(canDrop).toBeDefined();
    return canDrop;
  };

  describe("canDrop", () => {
    it("valid: should return true when source data type is ActivityBarButton", () => {
      const canDrop = getCanDropFunction();
      const result = canDrop!({ source: { data: { type: "ActivityBarButton" } } } as any);
      expect(result).toBe(true);
    });

    it("invalid: should return false when source data type is not ActivityBarButton", () => {
      const canDrop = getCanDropFunction();
      const result = canDrop!({ source: { data: { type: "SomeOtherType" } } } as any);
      expect(result).toBe(false);
    });

    it("invalid: should return false when source data is undefined", () => {
      const canDrop = getCanDropFunction();
      const result = canDrop!({ source: {} } as any);
      expect(result).toBe(false);
    });

    it("invalid: should return false when source is undefined", () => {
      const canDrop = getCanDropFunction();
      const result = canDrop!({} as any);
      expect(result).toBe(false);
    });
  });
});
