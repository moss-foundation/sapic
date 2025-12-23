import React from "react";
import { createRoot } from "react-dom/client";

import { useCurrentWorkspace } from "@/hooks/workspace/derived/useCurrentWorkspace";
import { Scrollbar } from "@/lib/ui/Scrollbar";
import { useUpdateLayout } from "@/workbench/adapters";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { defaultConfig, nextId } from "./defaultLayout";
import { PanelBuilder } from "./panelBuilder";

let mount = document.querySelector(".popover-anchor") as HTMLElement | null;

if (!mount) {
  mount = document.createElement("div");
  mount.className = "popover-anchor";
  document.body.insertBefore(mount, document.body.firstChild);
}

const PopoverComponent = (props: { close: () => void; component: React.FC<{ close: () => void }> }) => {
  const ref = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
    const handler = (ev: MouseEvent) => {
      let target = ev.target as HTMLElement;

      while (target.parentElement) {
        if (target === ref.current) {
          return;
        }
        target = target.parentElement;
      }

      props.close();
    };

    window.addEventListener("mousedown", handler);

    return () => {
      window.removeEventListener("mousedown", handler);
    };
  }, []);

  return (
    <div className="absolute left-0 top-0 z-[9999] h-full w-full bg-amber-400">
      <div
        ref={ref}
        className="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform bg-black p-2.5 text-white"
      >
        <props.component close={props.close} />
      </div>
    </div>
  );
};

function usePopover() {
  return {
    open: (Component: React.FC<{ close: () => void }>) => {
      const el = document.createElement("div");
      mount!.appendChild(el);
      const root = createRoot(el);

      root.render(
        <PopoverComponent
          component={Component}
          close={() => {
            root.unmount();
            el.remove();
          }}
        />
      );
    },
  };
}

export const GridActions = () => {
  const { api, watermark, setWatermark } = useTabbedPaneStore();
  const { mutate: updateLayout } = useUpdateLayout();

  const { currentWorkspaceId } = useCurrentWorkspace();

  const hasCustomWatermark = watermark;
  const toggleCustomWatermark = () => {
    setWatermark(!watermark);
  };

  const onClear = () => {
    api?.clear();
  };

  const gridState = useTabbedPaneStore((state) => state.gridState);
  const onLoad = () => {
    if (gridState) {
      try {
        api?.fromJSON(gridState);
      } catch (err) {
        console.error("failed to load saved state", err);
      }
    }
  };

  const onSave = () => {
    if (api && currentWorkspaceId) {
      updateLayout({ layout: { tabbedPaneState: { gridState: api.toJSON() } }, workspaceId: currentWorkspaceId });
    }
  };

  const onReset = () => {
    if (api) {
      try {
        api.clear();
        defaultConfig(api);
      } catch (err) {
        console.error("failed to reset state to default", err);
      }
    }
  };

  const popover = usePopover();

  const onAddPanel = (options?: { advanced?: boolean; type?: string }) => {
    const panelType = options?.type;
    if (panelType && api?.getPanel(panelType) !== undefined) {
      api.getPanel(panelType)?.focus();
      return;
    }

    if (options?.advanced) {
      popover.open(({ close }) => {
        return <PanelBuilder api={api!} done={close} />;
      });
    } else {
      api?.addPanel({
        id: panelType && panelType !== "nested" ? panelType : `id_${Date.now().toString()}`,
        component: options?.type ?? "Default",
        title: options?.type ?? `Tab ${nextId()}`,
        renderer: "onlyWhenVisible",
      });
    }
  };

  const onAddGroup = () => {
    api?.addGroup();
  };

  const [gap, setGap] = React.useState(0);

  return (
    <div className="action-container select-none">
      <Scrollbar>
        <div className="flex h-10 items-center gap-2">
          <span className="grow" />
          <div className="button-group">
            <button className="text-button" onClick={() => onAddPanel()}>
              Add Panel
            </button>
            <button className="demo-icon-button !rounded" onClick={() => onAddPanel({ advanced: true })}>
              <span className="material-symbols-outlined">tune</span>
            </button>
          </div>
          <button className="text-button" onClick={() => onAddPanel({ type: "nested" })}>
            Add Nested Panel
          </button>
          <button className="text-button" onClick={onAddGroup}>
            Add Group
          </button>
          <span className="button-action">
            <button
              className={hasCustomWatermark ? "demo-button selected !rounded" : "demo-button !rounded"}
              onClick={toggleCustomWatermark}
            >
              Use Custom Watermark
            </button>
          </span>
          <button className="text-button" onClick={onClear}>
            Clear
          </button>
          <span className="flex-grow" />
          <button className="text-button" onClick={onLoad}>
            Load State
          </button>
          <button className="text-button" onClick={onSave}>
            Save State
          </button>
          <button className="text-button" onClick={onReset}>
            Use Default State
          </button>
          <span className="grow" />
          <div className="flex items-center">
            <span className="pr-1">Grid Gap</span>
            <input
              className="w-10 text-center"
              type="number"
              min={0}
              max={99}
              step={1}
              value={gap}
              onChange={(event) => setGap(Number(event.target.value))}
            />
            <button className="text-button" onClick={() => setGap(0)}>
              Reset
            </button>
          </div>
        </div>
      </Scrollbar>
    </div>
  );
};
