import { IDockviewPanelModel } from "../../dockview/dockviewPanelModel";
import { IContentRenderer, ITabRenderer } from "../../dockview/types";

export class DockviewPanelModelMock implements IDockviewPanelModel {
  constructor(
    readonly contentComponent: string,
    readonly content: IContentRenderer,
    readonly tabComponent: string,
    readonly tab: ITabRenderer
  ) {
    //
  }

  init(): void {
    //
  }

  updateParentGroup(): void {
    //
  }

  update(): void {
    //
  }

  layout(): void {
    //
  }

  dispose(): void {
    //
  }
}
