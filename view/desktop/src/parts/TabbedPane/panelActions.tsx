import React from "react";
import ReactDOM from "react-dom";

import { Scrollbar } from "@/components/Scrollbar";
import { DockviewApi, IDockviewPanel } from "@repo/moss-tabs";

const PanelAction = (props: { panels: string[]; api: DockviewApi; activePanel?: string; panelId: string }) => {
  const onClick = () => {
    props.api.getPanel(props.panelId)?.focus();
  };

  React.useEffect(() => {
    const panel = props.api.getPanel(props.panelId);
    if (panel) {
      const disposable = panel.api.onDidVisibilityChange((event) => {
        setVisible(event.isVisible);
      });
      setVisible(panel.api.isVisible);

      return () => {
        disposable.dispose();
      };
    }
  }, [props.api, props.panelId]);

  const [panel, setPanel] = React.useState<IDockviewPanel | undefined>(undefined);

  React.useEffect(() => {
    const list = [
      props.api.onDidLayoutFromJSON(() => {
        setPanel(props.api.getPanel(props.panelId));
      }),
    ];

    if (panel) {
      const disposable = panel.api.onDidVisibilityChange((event) => {
        setVisible(event.isVisible);
      });
      setVisible(panel.api.isVisible);

      list.push(disposable);
    }

    setPanel(props.api.getPanel(props.panelId));

    return () => {
      list.forEach((l) => l.dispose());
    };
  }, [props.api, props.panelId]);

  const [visible, setVisible] = React.useState<boolean>(true);

  const [isPopupOpen, setIsPopupOpen] = React.useState(false);

  const togglePopup = () => {
    setIsPopupOpen(!isPopupOpen);
  };

  return (
    <div className="button-action select-none">
      <div className="flex">
        <button
          className={props.activePanel === props.panelId ? "demo-button selected" : "demo-button"}
          onClick={onClick}
        >
          {props.panelId}
        </button>
      </div>
      <div className="flex">
        <button
          className="demo-icon-button"
          onClick={() => {
            const panel = props.api.getPanel(props.panelId);
            if (panel) {
              props.api.addFloatingGroup(panel);
            }
          }}
        >
          <span className="material-symbols-outlined">ad_group</span>
        </button>
        <button
          className="demo-icon-button"
          onClick={() => {
            const panel = props.api.getPanel(props.panelId);
            if (panel) {
              props.api.addPopoutGroup(panel);
            }
          }}
        >
          <span className="material-symbols-outlined">open_in_new</span>
        </button>
        <button
          className="demo-icon-button"
          onClick={() => {
            const panel = props.api.getPanel(props.panelId);
            panel?.api.close();
          }}
        >
          <span className="material-symbols-outlined">close</span>
        </button>
        <button title="Panel visiblity cannot be edited manually." disabled={true} className="demo-icon-button">
          <span className="material-symbols-outlined">{visible ? "visibility" : "visibility_off"}</span>
        </button>
        <div>
          <button className="demo-icon-button" onClick={togglePopup}>
            <span className="material-symbols-outlined">edit</span>
          </button>
          {isPopupOpen && panel && <TitleEditPopup panel={panel} onClose={togglePopup} />}
        </div>
      </div>
    </div>
  );
};

const TitleEditPopup: React.FC<{ panel: IDockviewPanel; onClose: () => void }> = ({ panel, onClose }) => {
  const [title, setTitle] = React.useState<string>(panel.title ?? "");

  const onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setTitle(event.target.value);
  };

  const onClick = () => {
    panel.setTitle(title);
    onClose();
  };

  return ReactDOM.createPortal(
    <div className="fixed inset-0 z-[9999] flex items-center justify-center bg-black/50">
      <div className="rounded-lg bg-black p-5 shadow-lg">
        <div>
          <span className="!text-white">Edit Panel Title</span>
        </div>
        <input className="bg-white !text-black select-text" value={title} onChange={onChange} />
        <div className="button-group">
          <button className="panel-builder-button" onClick={onClick}>
            Edit
          </button>
          <button className="panel-builder-button" onClick={onClose}>
            Close
          </button>
        </div>
      </div>
    </div>,
    document.body
  );
};

export const PanelActions = (props: { panels: string[]; api: DockviewApi; activePanel?: string }) => {
  return (
    <div className="action-container select-none">
      <Scrollbar>
        <div className="flex items-center gap-2">
          {props.panels.map((id, index) => {
            return <PanelAction key={`panel-${id}-${index}`} {...props} panelId={id} />;
          })}
        </div>
      </Scrollbar>
    </div>
  );
};
