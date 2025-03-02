import { beforeEach, describe, expect, test, vi } from "vitest";

import { fireEvent } from "@testing-library/dom";
import { fromPartial } from "@total-typescript/shoehorn";

import { ContentContainer } from "../../../../dockview/components/panel/content";
import { DockviewComponent } from "../../../../dockview/dockviewComponent";
import { DockviewGroupPanelModel } from "../../../../dockview/dockviewGroupPanelModel";
import { IDockviewPanel } from "../../../../dockview/dockviewPanel";
import { IDockviewPanelModel } from "../../../../dockview/dockviewPanelModel";
import { GroupPanelPartInitParameters, IContentRenderer } from "../../../../dockview/types";
import { CompositeDisposable } from "../../../../lifecycle";
import { OverlayRenderContainer } from "../../../../overlay/overlayRenderContainer";
import { PanelUpdateEvent } from "../../../../panel/types";

class TestContentRenderer extends CompositeDisposable implements IContentRenderer {
  readonly element: HTMLElement;

  constructor(public id: string) {
    super();
    this.element = document.createElement("div");
    this.element.id = id;
  }

  init(parameters: GroupPanelPartInitParameters): void {
    //
  }

  layout(width: number, height: number): void {
    //
  }
  update(event: PanelUpdateEvent): void {
    //
  }

  toJSON(): object {
    return {};
  }

  focus(): void {
    //
  }
}

describe("contentContainer", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  test("basic focus test", async () => {
    let focus = 0;
    let blur = 0;

    const disposable = new CompositeDisposable();

    const overlayRenderContainer = new OverlayRenderContainer(
      document.createElement("div"),
      fromPartial<DockviewComponent>({})
    );

    const cut = new ContentContainer(
      fromPartial<DockviewComponent>({
        renderer: "onlyWhenVisible",
        overlayRenderContainer,
      }),
      fromPartial<DockviewGroupPanelModel>({
        renderContainer: overlayRenderContainer,
      })
    );

    disposable.addDisposables(
      cut.onDidFocus(() => {
        focus++;
      }),
      cut.onDidBlur(() => {
        blur++;
      })
    );

    const contentRenderer = new TestContentRenderer("id-1");

    const panel = fromPartial<IDockviewPanel>({
      view: {
        content: contentRenderer,
      },
      api: { renderer: "onlyWhenVisible" },
    });

    cut.openPanel(panel as IDockviewPanel);

    expect(focus).toBe(0);
    expect(blur).toBe(0);

    // container has focus within
    fireEvent.focus(contentRenderer.element);
    expect(focus).toBe(1);
    expect(blur).toBe(0);

    // container looses focus
    fireEvent.blur(contentRenderer.element);
    await vi.runAllTimersAsync();
    expect(focus).toBe(1);
    expect(blur).toBe(1);

    const contentRenderer2 = new TestContentRenderer("id-2");

    const panel2 = {
      view: {
        content: contentRenderer2,
      } as Partial<IDockviewPanelModel>,
      api: { renderer: "onlyWhenVisible" },
    } as Partial<IDockviewPanel>;

    cut.openPanel(panel2 as IDockviewPanel);
    // expect(focus).toBe(2);
    // expect(blur).toBe(1);

    // new panel recieves focus
    fireEvent.focus(contentRenderer2.element);
    expect(focus).toBe(2);
    expect(blur).toBe(1);

    // new panel looses focus
    fireEvent.blur(contentRenderer2.element);
    await vi.runAllTimersAsync();
    expect(focus).toBe(2);
    expect(blur).toBe(2);

    disposable.dispose();
  });

  test("that panels renderered as 'onlyWhenVisible' are removed when closed", () => {
    const overlayRenderContainer = fromPartial<OverlayRenderContainer>({
      detatch: vi.fn(),
    });

    const cut = new ContentContainer(
      fromPartial<DockviewComponent>({
        overlayRenderContainer,
      }),
      fromPartial<DockviewGroupPanelModel>({
        renderContainer: overlayRenderContainer,
      })
    );

    const panel1 = fromPartial<IDockviewPanel>({
      api: {
        renderer: "onlyWhenVisible",
      },
      view: { content: new TestContentRenderer("panel_1") },
    });

    const panel2 = fromPartial<IDockviewPanel>({
      api: {
        renderer: "onlyWhenVisible",
      },
      view: { content: new TestContentRenderer("panel_2") },
    });

    cut.openPanel(panel1);

    expect(panel1.view.content.element.parentElement).toBe(cut.element);
    expect(cut.element.childNodes.length).toBe(1);

    cut.openPanel(panel2);

    expect(panel1.view.content.element.parentElement).toBeNull();
    expect(panel2.view.content.element.parentElement).toBe(cut.element);
    expect(cut.element.childNodes.length).toBe(1);
  });
});
