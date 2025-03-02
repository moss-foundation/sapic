import { beforeEach, describe, expect, test, vi } from "vitest";

import { fromPartial } from "@total-typescript/shoehorn";

import { DockviewPanelModelMock } from "../__mocks__/mockDockviewPanelModel";
import { DockviewComponent } from "../../dockview/dockviewComponent";
import { DockviewGroupPanel } from "../../dockview/dockviewGroupPanel";
import { GroupOptions } from "../../dockview/dockviewGroupPanelModel";
import { DockviewPanel } from "../../dockview/dockviewPanel";
import { IContentRenderer, ITabRenderer } from "../../dockview/types";
import { OverlayRenderContainer } from "../../overlay/overlayRenderContainer";

describe("dockviewGroupPanel", () => {
  test("default minimum/maximium width/height", () => {
    const accessor = fromPartial<DockviewComponent>({
      onDidActivePanelChange: vi.fn(),
      onDidAddPanel: vi.fn(),
      onDidRemovePanel: vi.fn(),
      options: {},
    });
    const options = fromPartial<GroupOptions>({});
    const cut = new DockviewGroupPanel(accessor, "test_id", options);

    expect(cut.minimumWidth).toBe(100);
    expect(cut.minimumHeight).toBe(100);
    expect(cut.maximumHeight).toBe(Number.MAX_SAFE_INTEGER);
    expect(cut.maximumWidth).toBe(Number.MAX_SAFE_INTEGER);
  });

  test("group constraints", () => {
    const accessor = fromPartial<DockviewComponent>({
      onDidActivePanelChange: vi.fn(),
      onDidAddPanel: vi.fn(),
      onDidRemovePanel: vi.fn(),
      doSetGroupActive: vi.fn(),
      overlayRenderContainer: fromPartial<OverlayRenderContainer>({
        attach: vi.fn(),
        detatch: vi.fn(),
      }),
      options: {},
    });
    const options = fromPartial<GroupOptions>({});
    const cut = new DockviewGroupPanel(accessor, "test_id", options);

    cut.api.setConstraints({
      minimumHeight: 10,
      maximumHeight: 100,
      minimumWidth: 20,
      maximumWidth: 200,
    });

    // initial constraints

    expect(cut.minimumWidth).toBe(20);
    expect(cut.minimumHeight).toBe(10);
    expect(cut.maximumHeight).toBe(100);
    expect(cut.maximumWidth).toBe(200);

    const panelModel = new DockviewPanelModelMock(
      "content_component",
      fromPartial<IContentRenderer>({
        element: document.createElement("div"),
      }),
      "tab_component",
      fromPartial<ITabRenderer>({
        element: document.createElement("div"),
      })
    );

    const panel = new DockviewPanel("panel_id", "component_id", undefined, accessor, accessor.api, cut, panelModel, {
      renderer: "onlyWhenVisible",
      minimumWidth: 21,
      minimumHeight: 11,
      maximumHeight: 101,
      maximumWidth: 201,
    });

    cut.model.openPanel(panel);

    // active panel constraints

    expect(cut.minimumWidth).toBe(21);
    expect(cut.minimumHeight).toBe(11);
    expect(cut.maximumHeight).toBe(101);
    expect(cut.maximumWidth).toBe(201);

    const panel2 = new DockviewPanel("panel_id", "component_id", undefined, accessor, accessor.api, cut, panelModel, {
      renderer: "onlyWhenVisible",
      minimumWidth: 22,
      minimumHeight: 12,
      maximumHeight: 102,
      maximumWidth: 202,
    });

    cut.model.openPanel(panel2);

    // active panel constraints

    expect(cut.minimumWidth).toBe(22);
    expect(cut.minimumHeight).toBe(12);
    expect(cut.maximumHeight).toBe(102);
    expect(cut.maximumWidth).toBe(202);

    const panel3 = new DockviewPanel("panel_id", "component_id", undefined, accessor, accessor.api, cut, panelModel, {
      renderer: "onlyWhenVisible",
    });

    cut.model.openPanel(panel3);

    // active panel without specified constraints so falls back to group constraints

    expect(cut.minimumWidth).toBe(20);
    expect(cut.minimumHeight).toBe(10);
    expect(cut.maximumHeight).toBe(100);
    expect(cut.maximumWidth).toBe(200);
  });
});
