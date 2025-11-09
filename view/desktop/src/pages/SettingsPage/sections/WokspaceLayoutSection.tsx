import SelectOutlined from "@/components/SelectOutlined";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace, useDescribeApp } from "@/hooks";
import { useGetBottomPanel } from "@/hooks/sharedStorage/layout/bottomPanel/useGetBottomPanel";
import { useUpdateBottomPanel } from "@/hooks/sharedStorage/layout/bottomPanel/useUpdateBottomPanel";
import { useGetSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useGetSidebarPanel";
import { useUpdateSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useUpdateSidebarPanel";
import { useGetLayout } from "@/hooks/sharedStorage/layout/useGetLayout";
import { useUpdateLayout } from "@/hooks/sharedStorage/layout/useUpdateLayout";
import { useUpdateConfiguration } from "@/hooks/useUpdateConfiguration";
import { MenuItemProps } from "@/utils/renderActionMenuItem";
import { ActivitybarPosition } from "@repo/moss-workspace";

import { Section } from "../Section";

export const WorkspaceLayoutSection = () => {
  const { hasActiveWorkspace } = useActiveWorkspace();

  return (
    <Section title="Workspace Layout">
      <div className="flex flex-col gap-4">
        {!hasActiveWorkspace && (
          <div className="rounded-md bg-yellow-50 p-3 text-sm text-yellow-800">
            <p>
              Sidebar and panel settings are only available when a workspace is active. These settings are saved per
              workspace.
            </p>
          </div>
        )}

        <SidebarTypeSection />
        <SidebarVisibilitySection />
        <BottomPaneVisibilitySection />
        <ActivityBarPositionSection />
      </div>
    </Section>
  );
};

const SidebarTypeSection = () => {
  const { hasActiveWorkspace } = useActiveWorkspace();
  const { data: appState } = useDescribeApp();
  const { mutate: updateConfiguration } = useUpdateConfiguration();

  const sidebarTypeItems: MenuItemProps[] = [
    {
      id: "sidebar-left",
      type: "radio",
      label: "Left",
      value: SIDEBAR_POSITION.LEFT,
    },
    {
      id: "sidebar-right",
      type: "radio",
      label: "Right",
      value: SIDEBAR_POSITION.RIGHT,
    },
  ];

  const handleSidebarTypeChange = (value: SIDEBAR_POSITION) => {
    console.log({
      key: "sidebarPosition",
      value: value,
      target: "WORKSPACE",
    });
    updateConfiguration({
      key: "sidebarPosition",
      value: value,
      target: "WORKSPACE",
    });
  };

  return (
    <div>
      <h3 className="mb-2 font-medium">Sidebar Type</h3>
      <div className="w-[200px]">
        <SelectOutlined.Root
          value={(appState?.configuration.contents.sidebarPosition as SIDEBAR_POSITION) || SIDEBAR_POSITION.LEFT}
          onValueChange={handleSidebarTypeChange}
          disabled={!hasActiveWorkspace}
        >
          <SelectOutlined.Trigger />
          <SelectOutlined.Content>
            {sidebarTypeItems.map((item) => (
              <SelectOutlined.Item key={item.id} value={item.value!}>
                {item.label}
              </SelectOutlined.Item>
            ))}
          </SelectOutlined.Content>
        </SelectOutlined.Root>
      </div>
    </div>
  );
};

const SidebarVisibilitySection = () => {
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();
  const { data: layout } = useGetLayout();
  const { mutate: updateLayout } = useUpdateLayout();

  const visibilityItems: MenuItemProps[] = [
    {
      id: "visible",
      type: "radio",
      label: "Visible",
      value: "visible",
    },
    {
      id: "hidden",
      type: "radio",
      label: "Hidden",
      value: "hidden",
    },
  ];

  const handleSidebarVisibilityChange = (value: string) => {
    updateLayout({ layout: { sidebarState: { visible: value === "visible" } }, workspaceId: activeWorkspaceId });
  };

  return (
    <div>
      <h3 className="mb-2 font-medium">Sidebar Visibility</h3>
      <div className="w-[200px]">
        <SelectOutlined.Root
          value={layout?.sidebarState.visible ? "visible" : "hidden"}
          onValueChange={handleSidebarVisibilityChange}
          disabled={!hasActiveWorkspace}
        >
          <SelectOutlined.Trigger />
          <SelectOutlined.Content>
            {visibilityItems.map((item) => (
              <SelectOutlined.Item key={item.id} value={item.value!}>
                {item.label}
              </SelectOutlined.Item>
            ))}
          </SelectOutlined.Content>
        </SelectOutlined.Root>
      </div>
    </div>
  );
};

const BottomPaneVisibilitySection = () => {
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();
  const { data: layout } = useGetLayout();
  const { mutate: updateLayout } = useUpdateLayout();

  const handleBottomPaneVisibilityChange = (value: string) => {
    const visibility = value === "visible";
    if (activeWorkspaceId && layout?.bottomPanelState.visible) {
      updateLayout({ layout: { bottomPanelState: { visible: visibility } }, workspaceId: activeWorkspaceId });
    }
  };

  const visibilityItems: MenuItemProps[] = [
    {
      id: "visible",
      type: "radio",
      label: "Visible",
      value: "visible",
    },
    {
      id: "hidden",
      type: "radio",
      label: "Hidden",
      value: "hidden",
    },
  ];

  return (
    <div>
      <h3 className="mb-2 font-medium">Bottom Pane Visibility</h3>
      <div className="w-[200px]">
        <SelectOutlined.Root
          value={layout?.bottomPanelState.visible ? "visible" : "hidden"}
          onValueChange={handleBottomPaneVisibilityChange}
          disabled={!hasActiveWorkspace}
        >
          <SelectOutlined.Trigger />
          <SelectOutlined.Content>
            {visibilityItems.map((item) => {
              if (item.type === "separator") {
                return <SelectOutlined.Separator key={item.id} />;
              }

              return (
                <SelectOutlined.Item key={item.id} value={item.value!}>
                  {item.label}
                </SelectOutlined.Item>
              );
            })}
          </SelectOutlined.Content>
        </SelectOutlined.Root>
      </div>
    </div>
  );
};

const ActivityBarPositionSection = () => {
  const { data: appState } = useDescribeApp();
  //TODO later we should handle the JsonValue differently
  const activityBarPosition =
    (appState?.configuration.contents.activityBarPosition as ActivitybarPosition) || ACTIVITYBAR_POSITION.DEFAULT;

  const { mutate: updateConfiguration } = useUpdateConfiguration();

  const activityBarPositionItems: MenuItemProps[] = [
    {
      id: ACTIVITYBAR_POSITION.DEFAULT,
      type: "radio",
      label: "Default",
      value: ACTIVITYBAR_POSITION.DEFAULT,
    },
    {
      id: ACTIVITYBAR_POSITION.TOP,
      type: "radio",
      label: "Top",
      value: ACTIVITYBAR_POSITION.TOP,
    },
    {
      id: ACTIVITYBAR_POSITION.BOTTOM,
      type: "radio",
      label: "Bottom",
      value: ACTIVITYBAR_POSITION.BOTTOM,
    },
    {
      id: ACTIVITYBAR_POSITION.HIDDEN,
      type: "radio",
      label: "Hidden",
      value: ACTIVITYBAR_POSITION.HIDDEN,
    },
  ];

  const handleActivityBarPositionChange = (value: string) => {
    const position = value as ActivitybarPosition;
    console.log({
      key: "activityBarPosition",
      value: position,
      target: "WORKSPACE",
    });
    updateConfiguration({
      key: "activityBarPosition",
      value: position,
      target: "WORKSPACE",
    });
  };

  return (
    <div>
      <h3 className="mb-2 font-medium">ActivityBar Position</h3>
      <div className="w-[200px]">
        <SelectOutlined.Root
          value={activityBarPosition || ACTIVITYBAR_POSITION.DEFAULT}
          onValueChange={handleActivityBarPositionChange}
        >
          <SelectOutlined.Trigger />
          <SelectOutlined.Content>
            {activityBarPositionItems.map((item) => {
              if (item.type === "separator") {
                return <SelectOutlined.Separator key={item.id} />;
              }

              return (
                <SelectOutlined.Item key={item.id} value={item.value!}>
                  {item.label}
                </SelectOutlined.Item>
              );
            })}
          </SelectOutlined.Content>
        </SelectOutlined.Root>
      </div>
    </div>
  );
};
