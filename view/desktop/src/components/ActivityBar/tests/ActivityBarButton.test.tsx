import { beforeEach, describe, expect, it, vi } from "vitest";

import { ACTIVITYBAR_POSITION } from "@/constants/layoutPositions";
import { useActivityBarStore } from "@/store/activityBar";
import { useAppResizableLayoutStore, type AppResizableLayoutStore } from "@/store/appResizableLayout";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { render } from "@testing-library/react";

import { ActivityBarButton } from "../ActivityBarButton";

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
vi.mock("@/store/appResizableLayout");

const mockedDropTarget = vi.mocked(dropTargetForElements);
const mockedUseActivityBarStore = vi.mocked(useActivityBarStore);
const mockedUseAppResizableLayoutStore = vi.mocked(useAppResizableLayoutStore);

describe("ActivityBarButton › dropTargetForElements", () => {
  const defaultProps = {
    id: "test",
    icon: "Add" as const,
    iconActive: "Add" as const,
    title: "Test Button",
    order: 1,
    isActive: false,
  };

  const setupMocks = () => {
    mockedUseActivityBarStore.mockReturnValue({
      position: ACTIVITYBAR_POSITION.DEFAULT,
      setActiveItem: vi.fn(),
    });

    mockedUseAppResizableLayoutStore.mockImplementation((selector: (state: AppResizableLayoutStore) => unknown) =>
      selector({
        sideBar: {
          visible: true,
          setVisible: vi.fn(),
          width: 200,
          minWidth: 100,
          maxWidth: 400,
          setWidth: vi.fn(),
        },
        sideBarPosition: "LEFT",
        setSideBarPosition: vi.fn(),
        bottomPane: {
          height: 200,
          setHeight: vi.fn(),
          visible: true,
          setVisible: vi.fn(),
          minHeight: 100,
          maxHeight: 400,
        },
        initialize: vi.fn(),
      })
    );
  };

  beforeEach(() => {
    vi.clearAllMocks();
    setupMocks();
  });

  const renderComponent = () => render(<ActivityBarButton {...defaultProps} />);

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
