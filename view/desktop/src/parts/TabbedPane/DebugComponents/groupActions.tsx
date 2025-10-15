import { DockviewApi, DockviewGroupLocation, IDockviewGroupPanel } from "moss-tabs";
import React from "react";

import { Scrollbar } from "@/lib/ui/Scrollbar";

const GroupAction = (props: { groupId: string; groups: string[]; api: DockviewApi; activeGroup?: string }) => {
  const onClick = () => {
    props.api?.getGroup(props.groupId)?.focus();
  };

  const isActive = props.activeGroup === props.groupId;

  const [group, setGroup] = React.useState<IDockviewGroupPanel | undefined>(undefined);

  React.useEffect(() => {
    const disposable = props.api.onDidLayoutFromJSON(() => {
      const group = props.api.getGroup(props.groupId);
      setGroup(group);
    });

    const group = props.api.getGroup(props.groupId);
    setGroup(group);

    return () => {
      disposable.dispose();
    };
  }, [props.api, props.groupId]);

  const [location, setLocation] = React.useState<DockviewGroupLocation | null>(null);
  const [isMaximized, setIsMaximized] = React.useState<boolean>(false);
  const [isVisible, setIsVisible] = React.useState<boolean>(true);

  React.useEffect(() => {
    if (!group) {
      setLocation(null);
      return;
    }

    const disposable = group.api.onDidLocationChange((event) => {
      setLocation(event.location);
    });

    const disposable2 = props.api.onDidMaximizedGroupChange(() => {
      setIsMaximized(group.api.isMaximized());
    });

    const disposable3 = group.api.onDidVisibilityChange(() => {
      setIsVisible(group.api.isVisible);
    });

    setLocation(group.api.location);
    setIsMaximized(group.api.isMaximized());
    setIsVisible(group.api.isVisible);

    return () => {
      disposable.dispose();
      disposable2.dispose();
      disposable3.dispose();
    };
  }, [group, props.api]);

  return (
    <div className="button-action select-none">
      <div className="flex">
        <button onClick={onClick} className={isActive ? "demo-button selected" : "demo-button"}>
          {props.groupId}
        </button>
      </div>
      <div className="flex">
        {/* Floating groups are disabled in our Dockview config */}
        {/* <button
          title="Add Floating Group"
          className={location?.type === "floating" ? "demo-icon-button selected" : "demo-icon-button"}
          onClick={() => {
            if (group) {
              // @ts-expect-error The types mismatch are also present in the original Dockview repo. Since we don't use floating groups, we can ignore it.
              props.api.addFloatingGroup(group, {
                width: 400,
                height: 300,
                x: 50,
                y: 50,
                position: {
                  bottom: 50,
                  right: 50,
                },
              });
            }
          }}
        >
          <span className="material-symbols-outlined">ad_group</span>
        </button> */}

        {/* Popouts are disabled in our Dockview config */}
        {/* <button
          title="Add Popout Group"
          className={location?.type === "popout" ? "demo-icon-button selected" : "demo-icon-button"}
          onClick={() => {
            if (group) {
              // @ts-expect-error The types mismatch are also present in the original Dockview repo. Since we don't use popouts, we can ignore it.
              props.api.addPopoutGroup(group);
            }
          }}
        >
          <span className="material-symbols-outlined">open_in_new</span>
        </button> */}
        <button
          title="Maximize Group"
          className={isMaximized ? "demo-icon-button selected" : "demo-icon-button"}
          onClick={() => {
            if (group) {
              if (group.api.isMaximized()) {
                group.api.exitMaximized();
              } else {
                group.api.maximize();
              }
            }
          }}
        >
          <span className="material-symbols-outlined">fullscreen</span>
        </button>
        <button
          title="Toggle Group Visibility"
          className="demo-icon-button"
          onClick={() => {
            if (group) {
              if (group.api.isVisible) {
                group.api.setVisible(false);
              } else {
                group.api.setVisible(true);
              }
            }
          }}
        >
          <span className="material-symbols-outlined">{isVisible ? "visibility" : "visibility_off"}</span>
        </button>
        <button
          title="Close Group"
          className="demo-icon-button"
          onClick={() => {
            const panel = props.api?.getGroup(props.groupId);
            panel?.api.close();
          }}
        >
          <span className="material-symbols-outlined">close</span>
        </button>
      </div>
    </div>
  );
};

export const GroupActions = (props: { groups: string[]; api: DockviewApi; activeGroup?: string }) => {
  return (
    <div className="action-container select-none">
      <Scrollbar>
        <div className="flex items-center gap-2">
          {props.groups.map((groupId, index) => {
            return <GroupAction key={`group-${groupId}-${index}`} {...props} groupId={groupId} />;
          })}
        </div>
      </Scrollbar>
    </div>
  );
};
