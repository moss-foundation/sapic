import { useContext, useState } from "react";

import { Icon, MossDropdown, ToggleButton } from "@/lib/ui";
import Select from "@/lib/ui/Select";
import { cn } from "@/utils";
import { useRenameResourceForm } from "@/workbench/hooks";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";

import { EndpointViewContext } from "../../EndpointViewContext";
import { EditableHeader } from "./EditableHeader";

export const EndpointViewHeader = () => {
  const { resourceDescription, projectId, resource } = useContext(EndpointViewContext);

  const { isRenamingResource, setIsRenamingResource, handleRenamingResourceSubmit, handleRenamingResourceCancel } =
    useRenameResourceForm(resource, projectId);

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
            title={resourceDescription.name}
            isRenamingResource={isRenamingResource}
            setIsRenamingResource={setIsRenamingResource}
            handleRenamingResourceSubmit={handleRenamingResourceSubmit}
            handleRenamingResourceCancel={handleRenamingResourceCancel}
            editable
          />
          <div className="flex items-center gap-2">
            <ToggleButton checked={isEnabled} onCheckedChange={setIsEnabled} />
            <Select.Root value={selectedValue} onValueChange={setSelectedValue}>
              <Select.Trigger
                placeholder="Select an option"
                childrenLeftSide={
                  <span
                    className={cn("size-1.5 rounded-full", {
                      "background-(--moss-blue-4)": selectedValue === "Released",
                      "background-(--moss-orange-5)": selectedValue === "Draft",
                      "background-(--moss-error)": selectedValue === "Archived",
                      "hidden": selectedValue === "Some very long name",
                    })}
                  />
                }
              />

              <Select.Content align="end">
                {options?.map((option) => (
                  <Select.Item key={option.value} value={option.value}>
                    {option.label}
                  </Select.Item>
                ))}
              </Select.Content>
            </Select.Root>

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
            <span className="text-(--moss-primary-descriptionForeground)">Created</span> <span>March 31, 2025</span>
          </div>
          <div className="flex gap-[3px]">
            <span className="text-(--moss-primary-descriptionForeground)">Updated</span> <span>March 31, 2025</span>
          </div>
        </div>
      </header>
    </PageWrapper>
  );
};
