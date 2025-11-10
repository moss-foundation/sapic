import SelectOutlined from "@/components/SelectOutlined";
import { useActiveWorkspace } from "@/hooks";
import { useGetLayout } from "@/hooks/sharedStorage/layout/useGetLayout";
import { useUpdateLayout } from "@/hooks/sharedStorage/layout/useUpdateLayout";
import { MenuItemProps } from "@/utils/renderActionMenuItem";

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

        <SidebarVisibilitySection />
        <BottomPaneVisibilitySection />
      </div>
    </Section>
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
