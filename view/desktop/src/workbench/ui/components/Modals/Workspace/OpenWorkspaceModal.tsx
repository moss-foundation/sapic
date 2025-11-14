import { useState } from "react";

import { useListWorkspaces, useOpenWorkspace } from "@/hooks/workbench";
import { Button } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import { RadioGroup } from "@/workbench/ui/components";
import { ModalForm } from "@/workbench/ui/components/ModalForm";
import SelectOutlined from "@/workbench/ui/components/SelectOutlined";
import { WorkspaceMode } from "@repo/moss-workspace";

import { ModalWrapperProps } from "../types";

export const OpenWorkspaceModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { data: workspaces, isLoading } = useListWorkspaces();
  const { mutate: openWorkspace } = useOpenWorkspace();

  const [mode, setMode] = useState<WorkspaceMode>("LIVE");
  const [selectedWorkspace, setSelectedWorkspace] = useState<string>("");
  const [reopenOnSession, setReopenOnSession] = useState(true);

  const handleSubmit = () => {
    if (!selectedWorkspace) return;

    openWorkspace(selectedWorkspace, {
      onSuccess: () => {
        closeModal();
        resetForm();
      },
      onError: (error) => {
        console.error("Failed to open workspace:", error.message);
      },
    });
  };

  const handleCancel = () => {
    closeModal();
    resetForm();
  };

  const resetForm = () => {
    setTimeout(() => {
      setSelectedWorkspace("");
      setMode("LIVE");
      setReopenOnSession(true);
    }, 200);
  };

  const isSubmitDisabled = !selectedWorkspace || isLoading;

  return (
    <ModalForm
      title="Open Workspace"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      className="background-(--moss-primary-background) max-w-136"
      titleClassName="border-b border-(--moss-border)"
      footerClassName="border-t border-(--moss-border)"
      content={
        <div className="flex flex-col gap-4">
          <div className="flex items-center gap-x-2 py-1.5">
            <div>Name:</div>
            <SelectOutlined.Root onValueChange={setSelectedWorkspace} value={selectedWorkspace}>
              <SelectOutlined.Trigger placeholder="Select workspace" />
              <SelectOutlined.Content>
                {workspaces?.map((workspace) => (
                  <SelectOutlined.Item value={workspace.id} key={workspace.id}>
                    {workspace.name}
                  </SelectOutlined.Item>
                ))}
              </SelectOutlined.Content>
            </SelectOutlined.Root>
          </div>

          <div>
            <div className="flex gap-2">
              <span>Mode</span>
              <div className="background-(--moss-border) my-auto h-px w-full" />
            </div>
            <p className="text-(--moss-secondary-foreground) text-xs leading-5">
              You can switch modes in the workspace at any time and as often as needed.
            </p>
            <div className="pl-5">
              <RadioGroup.Root>
                <RadioGroup.ItemWithLabel
                  value="LIVE"
                  checked={mode === "LIVE"}
                  onClick={() => setMode("LIVE")}
                  label="Live mode"
                  description="Start by designing your API structure (endpoints, schemas, etc.) before writing requests. Ideal for planning and generating documentation upfront."
                />
                <RadioGroup.ItemWithLabel
                  value="DESIGN"
                  checked={mode === "DESIGN"}
                  onClick={() => setMode("DESIGN")}
                  label="Design mode"
                  description="Begin by writing and testing requests, then define the API structure based on actual usage. Great for quick prototyping and iterating."
                />
              </RadioGroup.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="py-0.75 flex items-center justify-between">
          <CheckboxWithLabel
            label="Reopen this workspace on next session"
            checked={reopenOnSession}
            onCheckedChange={(checked) => {
              if (checked !== "indeterminate") {
                setReopenOnSession(checked);
              }
            }}
          />
          <div className="px-0.25 py-1.25 flex gap-3">
            <Button intent="outlined" onClick={handleCancel}>
              Cancel
            </Button>
            <Button intent="primary" disabled={isSubmitDisabled} type="submit">
              Open
            </Button>
          </div>
        </div>
      }
    />
  );
};
