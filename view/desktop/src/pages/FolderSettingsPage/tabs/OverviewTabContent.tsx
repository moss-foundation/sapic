import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { TreeCollectionNode } from "@/components/CollectionTree/types";
import { EntryKind } from "@repo/moss-collection";

export interface FolderSettingsParams {
  collectionId: string;
  node: TreeCollectionNode;
  iconType: EntryKind;
}

export const OverviewTabContent = ({ params }: IDockviewPanelProps<FolderSettingsParams>) => {
  const { collectionId, node, iconType } = params || {};

  if (!node || !collectionId) {
    return (
      <div className="p-4">
        <p className="text-(--moss-error-text)">No folder data available</p>
      </div>
    );
  }

  return (
    <div className="space-y-4 p-4">
      <div className="rounded-lg border border-(--moss-border-color) p-4">
        <h3 className="mb-3 text-lg font-semibold text-(--moss-primary-text)">Folder Information</h3>

        <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-text)">Folder Name</label>
            <div className="rounded border border-(--moss-border-color) bg-(--moss-input-background) px-3 py-2 text-(--moss-primary-text)">
              {node.name}
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-text)">Folder ID</label>
            <div className="rounded border border-(--moss-border-color) bg-(--moss-input-background) px-3 py-2 font-mono text-sm text-(--moss-primary-text)">
              {node.id}
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-text)">Collection ID</label>
            <div className="rounded border border-(--moss-border-color) bg-(--moss-input-background) px-3 py-2 font-mono text-sm text-(--moss-primary-text)">
              {collectionId}
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-text)">Folder Type</label>
            <div className="rounded border border-(--moss-border-color) bg-(--moss-input-background) px-3 py-2 text-(--moss-primary-text)">
              {node.kind} ({node.class})
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-text)">Order</label>
            <div className="rounded border border-(--moss-border-color) bg-(--moss-input-background) px-3 py-2 text-(--moss-primary-text)">
              {node.order ?? "Not set"}
            </div>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-(--moss-secondary-text)">Expanded</label>
            <div className="rounded border border-(--moss-border-color) bg-(--moss-input-background) px-3 py-2 text-(--moss-primary-text)">
              {node.expanded ? "Yes" : "No"}
            </div>
          </div>
        </div>
      </div>

      <div className="rounded-lg border border-(--moss-border-color) p-4">
        <h3 className="mb-3 text-lg font-semibold text-(--moss-primary-text)">Child Nodes</h3>

        <div className="space-y-2">
          <div className="text-sm text-(--moss-secondary-text)">Total child nodes: {node.childNodes?.length || 0}</div>

          {node.childNodes && node.childNodes.length > 0 ? (
            <div className="rounded bg-(--moss-secondary-background) p-3">
              <div className="space-y-2">
                {node.childNodes.map((child, index) => (
                  <div
                    key={child.id}
                    className="flex items-center justify-between rounded border border-(--moss-border-color) bg-(--moss-primary-background) p-2"
                  >
                    <div className="flex items-center gap-2">
                      <span className="font-mono text-xs text-(--moss-secondary-text)">#{index + 1}</span>
                      <span className="font-medium text-(--moss-primary-text)">{child.name}</span>
                      <span className="rounded bg-(--moss-tag-background) px-2 py-1 text-xs text-(--moss-secondary-text)">
                        {child.kind}
                      </span>
                    </div>
                    <span className="font-mono text-xs text-(--moss-secondary-text)">{child.id}</span>
                  </div>
                ))}
              </div>
            </div>
          ) : (
            <div className="text-(--moss-secondary-text) italic">No child nodes</div>
          )}
        </div>
      </div>

      <div className="rounded-lg border border-(--moss-border-color) p-4">
        <h3 className="mb-3 text-lg font-semibold text-(--moss-primary-text)">Debug Information</h3>

        <div className="rounded border border-(--moss-border-color) bg-(--moss-code-background) p-3">
          <pre className="overflow-auto text-sm text-(--moss-code-text)">
            {JSON.stringify({ node, collectionId, iconType }, null, 2)}
          </pre>
        </div>
      </div>
    </div>
  );
};
