import { IDockviewPanelProps } from "moss-tabs";

import { PageContent } from "@/components";
import { ProjectTreeNode } from "@/components/ProjectTree/types";
import { useProjectsTrees } from "@/hooks";
import { EntryKind } from "@repo/moss-project";

import { findNodeInProject } from "../utils";

export interface FolderSettingsParams {
  projectId: string;
  node: ProjectTreeNode;
  iconType: EntryKind;
}

export const OverviewTabContent = ({ params }: IDockviewPanelProps<FolderSettingsParams>) => {
  const { projectsTrees: projectsTrees } = useProjectsTrees();
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
      <div className="rounded-lg border border-(--moss-border) p-4">
        <h3 className="mb-3 text-lg font-semibold text-(--moss-primary-foreground)">Folder Information</h3>

        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-foreground)">Folder Name</label>
            <div className="background-(--moss-secondary-background) rounded border border-(--moss-border) px-3 py-2 text-(--moss-primary-foreground)">
              {node.name}
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-foreground)">Folder ID</label>
            <div className="background-(--moss-secondary-background) rounded border border-(--moss-border) px-3 py-2 font-mono text-sm text-(--moss-primary-foreground)">
              {node.id}
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-foreground)">Project ID</label>
            <div className="background-(--moss-secondary-background) rounded border border-(--moss-border) px-3 py-2 font-mono text-sm text-(--moss-primary-foreground)">
              {params?.projectId}
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-foreground)">Folder Type</label>
            <div className="background-(--moss-secondary-background) rounded border border-(--moss-border) px-3 py-2 text-(--moss-primary-foreground)">
              {node.kind} ({node.class})
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-foreground)">Order</label>
            <div className="background-(--moss-secondary-background) rounded border border-(--moss-border) px-3 py-2 text-(--moss-primary-foreground)">
              {node.order ?? "Not set"}
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-foreground)">Expanded</label>
            <div className="background-(--moss-secondary-background) rounded border border-(--moss-border) px-3 py-2 text-(--moss-primary-foreground)">
              {node.expanded ? "Yes" : "No"}
            </div>
          </div>
        </div>
      </div>

      <div className="rounded-lg border border-(--moss-border) p-4">
        <h3 className="mb-3 text-lg font-semibold text-(--moss-primary-foreground)">Child Nodes</h3>

        <div className="space-y-2">
          <div className="text-sm text-(--moss-secondary-foreground)">
            Total child nodes: {node.childNodes?.length || 0}
          </div>

          {node.childNodes && node.childNodes.length > 0 ? (
            <div className="background-(--moss-secondary-background) rounded p-3">
              <div className="space-y-2">
                {node.childNodes.map((child, index) => (
                  <div
                    key={child.id}
                    className="background-(--moss-primary-background) flex items-center justify-between rounded border border-(--moss-border) p-2"
                  >
                    <div className="flex items-center gap-2">
                      <span className="font-mono text-xs text-(--moss-secondary-foreground)">#{index + 1}</span>
                      <span className="font-medium text-(--moss-primary-foreground)">{child.name}</span>
                      <span className="background-(--moss-secondary-background) rounded px-2 py-1 text-xs text-(--moss-secondary-foreground)">
                        {child.kind}
                      </span>
                    </div>
                    <span className="font-mono text-xs text-(--moss-secondary-foreground)">{child.id}</span>
                  </div>
                ))}
              </div>
            </div>
          ) : (
            <div className="text-(--moss-secondary-foreground) italic">No child nodes</div>
          )}
        </div>
      </div>

      <div className="rounded-lg border border-(--moss-border) p-4">
        <h3 className="mb-3 text-lg font-semibold text-(--moss-primary-foreground)">Debug Information</h3>

        <div className="background-(--moss-secondary-background) rounded border border-(--moss-border) p-3">
          <pre className="overflow-auto text-sm text-(--moss-secondary-foreground)">
            {JSON.stringify({ node, projectId: params?.projectId, iconType: params?.iconType }, null, 2)}
          </pre>
        </div>
      </div>
    </PageContent>
  );
};
