import { useActiveWorkspace } from "@/hooks";
import { useGetLayout, useUpdateLayout } from "@/workbench/adapters";
import { ACTIVITYBAR_POSITION, SIDEBAR_POSITION } from "@/workbench/domains/layout";
import SelectOutlined from "@/workbench/ui/components/SelectOutlined";
import { MenuItemProps } from "@/workbench/utils/renderActionMenuItem";

import { Section } from "../Section";

export const AppLayoutSection = () => {
  return (
    <Section title="App Layout">
      <div className="flex flex-col gap-4">
        <SideBarPositionSection />
        <ActivityBarPositionSection />
      </div>
    </Section>
  );
};

const SideBarPositionSection = () => {
  const { activeWorkspaceId } = useActiveWorkspace();
  const { data: layout } = useGetLayout();
  const { mutate: updateLayout } = useUpdateLayout();

  const sideBarPosition = layout?.sidebarState.position || SIDEBAR_POSITION.LEFT;

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
    if (!activeWorkspaceId) return;

    updateLayout({
      layout: { sidebarState: { position: value } },
      workspaceId: activeWorkspaceId,
    });
  };

  return (
    <div>
      <h3 className="mb-2 font-medium">Sidebar Position</h3>
      <div className="w-[200px]">
        <SelectOutlined.Root value={sideBarPosition} onValueChange={handleSidebarTypeChange}>
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

const ActivityBarPositionSection = () => {
  const { activeWorkspaceId } = useActiveWorkspace();

  const { data: layout } = useGetLayout();
  //TODO later we should handle the JsonValue differently
  const activityBarPosition = layout?.activitybarState.position || ACTIVITYBAR_POSITION.DEFAULT;

  const { mutate: updateLayout } = useUpdateLayout();

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

  const handleActivityBarPositionChange = (position: ACTIVITYBAR_POSITION) => {
    if (!activeWorkspaceId) return;

    updateLayout({
      layout: { activitybarState: { position: position } },
      workspaceId: activeWorkspaceId,
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
