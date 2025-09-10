import { useState } from "react";

import { PageWrapper } from "@/components/PageView/PageWrapper";
import { useRenameEntryForm } from "@/hooks";
import { DockviewPanelApi } from "@/lib/moss-tabs/src";
import { MossToggle } from "@/lib/ui";
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

  const [isEnabled, setIsEnabled] = useState(false);
  return (
    <PageWrapper>
      <header className="flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <EditableHeader
            title={node.name}
            isRenamingEntry={isRenamingEntry}
            setIsRenamingEntry={setIsRenamingEntry}
            handleRenamingEntrySubmit={handleRenamingEntrySubmit}
            handleRenamingEntryCancel={handleRenamingEntryCancel}
            editable
          />
          <div>
            <div className="flex items-center gap-2">
              <span>{isEnabled ? "Enabled" : "Disabled"}</span>
              <MossToggle checked={isEnabled} onCheckedChange={setIsEnabled} />
            </div>
          </div>
        </div>

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
