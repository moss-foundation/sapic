import SelectOutlined from "@/components/SelectOutlined";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/constants/layoutPositions";
import { useActiveWorkspace } from "@/hooks";
import { useGetBottomPanel } from "@/hooks/sharedStorage/layout/bottomPanel/useGetBottomPanel";
import { useUpdateBottomPanel } from "@/hooks/sharedStorage/layout/bottomPanel/useUpdateBottomPanel";
import { useGetSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useGetSidebarPanel";
import { useUpdateSidebarPanel } from "@/hooks/sharedStorage/layout/sidebar/useUpdateSidebarPanel";
import { useActivityBarStore } from "@/store/activityBar";
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
  const { activeWorkspaceId, hasActiveWorkspace } = useActiveWorkspace();
  const { data: sideBar } = useGetSidebarPanel();
  const { mutate: updateSidebarPanel } = useUpdateSidebarPanel();

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
    updateSidebarPanel({ position: value, workspaceId: activeWorkspaceId });
  };

  return (
    <div>
      <h3 className="mb-2 font-medium">Sidebar Type</h3>
      <div className="w-[200px]">
        <SelectOutlined.Root
          value={sideBar?.position || SIDEBAR_POSITION.LEFT}
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
  const { data: sideBar } = useGetSidebarPanel();
  const { mutate: updateSidebarPanel } = useUpdateSidebarPanel();

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
    updateSidebarPanel({ visible: value === "visible", workspaceId: activeWorkspaceId });
  };

  return (
    <div>
      <h3 className="mb-2 font-medium">Sidebar Visibility</h3>
      <div className="w-[200px]">
        <SelectOutlined.Root
          value={sideBar?.visible ? "visible" : "hidden"}
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
  const { data: bottomPane } = useGetBottomPanel();
  const { mutate: updateBottomPanel } = useUpdateBottomPanel();

  const handleBottomPaneVisibilityChange = (value: string) => {
    const visibility = value === "visible";
    if (activeWorkspaceId && bottomPane) {
      updateBottomPanel({ visible: visibility, workspaceId: activeWorkspaceId });
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
          value={bottomPane?.visible ? "visible" : "hidden"}
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
  const { setPosition, position } = useActivityBarStore();

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
    setPosition(position);
  };

  return (
    <div>
      <h3 className="mb-2 font-medium">ActivityBar Position</h3>
      <div className="w-[200px]">
        <SelectOutlined.Root
          value={position || ACTIVITYBAR_POSITION.DEFAULT}
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
