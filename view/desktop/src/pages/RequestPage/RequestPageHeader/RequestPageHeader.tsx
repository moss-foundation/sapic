import { PageWrapper } from "@/components/PageView/PageWrapper";
import { useRenameEntryForm } from "@/hooks";
import { DockviewPanelApi } from "@/lib/moss-tabs/src";
import { StreamEntriesEvent } from "@repo/moss-collection";

import { EditableHeader } from "./EditableHeader";

interface RequestPageHeaderProps {
  node: StreamEntriesEvent;
  collectionId: string;
  api: DockviewPanelApi;
}

export const RequestPageHeader = ({ node, collectionId, api }: RequestPageHeaderProps) => {
  const { isRenamingEntry, setIsRenamingEntry, handleRenamingEntrySubmit, handleRenamingEntryCancel } =
    useRenameEntryForm(node, collectionId);

  return (
    <PageWrapper>
      <header className="flex flex-col gap-3">
        <EditableHeader
          title={node.name}
          isRenamingEntry={isRenamingEntry}
          setIsRenamingEntry={setIsRenamingEntry}
          handleRenamingEntrySubmit={handleRenamingEntrySubmit}
          handleRenamingEntryCancel={handleRenamingEntryCancel}
          editable
        />

        <div className="flex items-center gap-5">
          <div className="flex gap-[3px]">
            <span className="text-(--moss-shortcut-text)">Created</span> <span>March 31, 2025</span>
          </div>
          <div className="flex gap-[3px]">
            <span className="text-(--moss-shortcut-text)">Updated</span> <span>March 31, 2025</span>
          </div>
        </div>
      </header>
    </PageWrapper>
  );
};
