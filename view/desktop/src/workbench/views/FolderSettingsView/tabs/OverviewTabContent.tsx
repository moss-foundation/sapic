import { IDockviewPanelProps } from "moss-tabs";

import { useProjectsTrees } from "@/workbench/adapters/tanstackQuery/project";
import { PageContent } from "@/workbench/ui/components";
import { ProjectTreeNode } from "@/workbench/ui/components/ProjectTree/types";
import { ResourceKind } from "@repo/moss-project";

import { findNodeInProject } from "../utils";

export interface FolderSettingsParams {
  projectId: string;
  node: ProjectTreeNode;
  iconType: ResourceKind;
}

export const OverviewTabContent = ({ params }: IDockviewPanelProps<FolderSettingsParams>) => {
  const { projectsTrees } = useProjectsTrees();
  const project = projectsTrees?.find((col) => col.id === params?.projectId);
  const node = project ? findNodeInProject(project, params?.node?.id) : undefined;

  if (!node || !params?.projectId) {
    return (
      <div className="p-4">
        <p className="text-(--moss-secondary-foreground)">No folder data available</p>
      </div>
    );
  }

  return (
    <PageContent className="flex flex-col gap-1">
      <div className="border-(--moss-border) rounded-lg border p-4">
        <h3 className="text-(--moss-primary-foreground) mb-3 text-lg font-semibold">Folder Information</h3>

        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          <div>
            <label className="text-(--moss-secondary-foreground) mb-1 block text-sm font-medium">Folder Name</label>
            <div className="background-(--moss-secondary-background) border-(--moss-border) text-(--moss-primary-foreground) rounded border px-3 py-2">
              {node.name}
            </div>
          </div>

          <div>
            <label className="text-(--moss-secondary-foreground) mb-1 block text-sm font-medium">Folder ID</label>
            <div className="background-(--moss-secondary-background) border-(--moss-border) text-(--moss-primary-foreground) rounded border px-3 py-2 font-mono text-sm">
              {node.id}
            </div>
          </div>

          <div>
            <label className="text-(--moss-secondary-foreground) mb-1 block text-sm font-medium">Project ID</label>
            <div className="background-(--moss-secondary-background) border-(--moss-border) text-(--moss-primary-foreground) rounded border px-3 py-2 font-mono text-sm">
              {params?.projectId}
            </div>
          </div>

          <div>
            <label className="text-(--moss-secondary-foreground) mb-1 block text-sm font-medium">Folder Type</label>
            <div className="background-(--moss-secondary-background) border-(--moss-border) text-(--moss-primary-foreground) rounded border px-3 py-2">
              {node.kind} ({node.class})
            </div>
          </div>

          <div>
            <label className="text-(--moss-secondary-foreground) mb-1 block text-sm font-medium">Order</label>
            <div className="background-(--moss-secondary-background) border-(--moss-border) text-(--moss-primary-foreground) rounded border px-3 py-2">
              {node.order ?? "Not set"}
            </div>
          </div>

          <div>
            <label className="text-(--moss-secondary-foreground) mb-1 block text-sm font-medium">Expanded</label>
            <div className="background-(--moss-secondary-background) border-(--moss-border) text-(--moss-primary-foreground) rounded border px-3 py-2">
              {node.expanded ? "Yes" : "No"}
            </div>
          </div>
        </div>
      </div>

      <div className="border-(--moss-border) rounded-lg border p-4">
        <h3 className="text-(--moss-primary-foreground) mb-3 text-lg font-semibold">Child Nodes</h3>

        <div className="space-y-2">
          <div className="text-(--moss-secondary-foreground) text-sm">
            Total child nodes: {node.childNodes?.length || 0}
          </div>

          {node.childNodes && node.childNodes.length > 0 ? (
            <div className="background-(--moss-secondary-background) rounded p-3">
              <div className="space-y-2">
                {node.childNodes.map((child, index) => (
                  <div
                    key={child.id}
                    className="background-(--moss-primary-background) border-(--moss-border) flex items-center justify-between rounded border p-2"
                  >
                    <div className="flex items-center gap-2">
                      <span className="text-(--moss-secondary-foreground) font-mono text-xs">#{index + 1}</span>
                      <span className="text-(--moss-primary-foreground) font-medium">{child.name}</span>
                      <span className="background-(--moss-secondary-background) text-(--moss-secondary-foreground) rounded px-2 py-1 text-xs">
                        {child.kind}
                      </span>
                    </div>
                    <span className="text-(--moss-secondary-foreground) font-mono text-xs">{child.id}</span>
                  </div>
                ))}
              </div>
            </div>
          ) : (
            <div className="text-(--moss-secondary-foreground) italic">No child nodes</div>
          )}
        </div>
      </div>

      <div className="border-(--moss-border) rounded-lg border p-4">
        <h3 className="text-(--moss-primary-foreground) mb-3 text-lg font-semibold">Debug Information</h3>

        <div className="background-(--moss-secondary-background) border-(--moss-border) rounded border p-3">
          <pre className="text-(--moss-secondary-foreground) overflow-auto text-sm">
            {JSON.stringify({ node, projectId: params?.projectId, iconType: params?.iconType }, null, 2)}
          </pre>
        </div>
      </div>
    </PageContent>
  );
};
