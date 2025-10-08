import { useContext, useState } from "react";

import { PageWrapper } from "@/components/PageView/PageWrapper";
import { useRenameEntryForm } from "@/hooks";
import { Icon, MossDropdown, MossToggle } from "@/lib/ui";
import MossSelect from "@/lib/ui/MossSelect";
import { cn } from "@/utils";

import { EndpointPageContext } from "../../EndpointPageContext";
import { EditableHeader } from "./EditableHeader";

export const EndpointPageHeader = () => {
  const { entryDescription: entry, projectId, entry: node } = useContext(EndpointPageContext);

  const { isRenamingEntry, setIsRenamingEntry, handleRenamingEntrySubmit, handleRenamingEntryCancel } =
    useRenameEntryForm(node, projectId);

  const [isEnabled, setIsEnabled] = useState(false);

  const [selectedValue, setSelectedValue] = useState("Released");

  const options = [
    { label: "All", value: "All" },
    { label: "Released", value: "Released" },
    { label: "Draft", value: "Draft" },
    { label: "Archived", value: "Archived" },
    { label: "Some very long name", value: "Some very long name" },
  ];

  return (
    <PageWrapper>
      <header className="flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <EditableHeader
            icon="Http"
            title={entry.name}
            isRenamingEntry={isRenamingEntry}
            setIsRenamingEntry={setIsRenamingEntry}
            handleRenamingEntrySubmit={handleRenamingEntrySubmit}
            handleRenamingEntryCancel={handleRenamingEntryCancel}
            editable
          />
          <div className="flex items-center gap-2">
            <MossToggle
              checked={isEnabled}
              onCheckedChange={setIsEnabled}
              labelLeft={isEnabled ? "Enabled" : "Disabled"}
            />
            <MossSelect.Root value={selectedValue} onValueChange={setSelectedValue}>
              <MossSelect.Trigger
                placeholder={"Placeholder"}
                childrenLeftSide={
                  <span
                    className={cn("size-1.5 rounded-full", {
                      "background-(--moss-primary)": selectedValue === "Released",
                      "background-(--moss-orange-5)": selectedValue === "Draft",
                      "background-(--moss-error)": selectedValue === "Archived",
                      "hidden": selectedValue === "Some very long name",
                    })}
                  />
                }
              />

              <MossSelect.Content align="end">
                {options?.map((option) => (
                  <MossSelect.Item key={option.value} value={option.value}>
                    {option.label}
                  </MossSelect.Item>
                ))}
              </MossSelect.Content>
            </MossSelect.Root>

            <MossDropdown.Root>
              <MossDropdown.Trigger>
                <Icon icon="MoreHorizontal" />
              </MossDropdown.Trigger>
              <MossDropdown.Portal>
                <MossDropdown.Content>
                  <MossDropdown.Item>Item 1</MossDropdown.Item>
                  <MossDropdown.Item>Item 2</MossDropdown.Item>
                  <MossDropdown.Item>Item 3</MossDropdown.Item>
                </MossDropdown.Content>
              </MossDropdown.Portal>
            </MossDropdown.Root>
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
