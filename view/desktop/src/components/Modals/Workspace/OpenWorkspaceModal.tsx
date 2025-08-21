import { useState } from "react";

import { RadioGroup } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { ModalForm } from "@/components/ModalForm";
import SelectOutlined from "@/components/SelectOutlined";
import { useListWorkspaces, useOpenWorkspace } from "@/hooks/workbench";
import { WorkspaceMode } from "@repo/moss-workspace";

import { ModalWrapperProps } from "../types";

export const OpenWorkspaceModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { data: workspaces, isLoading } = useListWorkspaces();
  const { mutate: openWorkspace } = useOpenWorkspace();

  const [mode, setMode] = useState<WorkspaceMode>("REQUEST_FIRST");
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
      setMode("REQUEST_FIRST");
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
      className="background-(--moss-primary-background) max-w-[544px]"
      titleClassName="border-b border-(--moss-border-color)"
      footerClassName="border-t border-(--moss-border-color)"
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
              <div className="background-(--moss-border-color) my-auto h-px w-full" />
            </div>
            <p className="text-xs leading-5 text-(--moss-secondary-text)">
              You can switch modes in the workspace at any time and as often as needed.
            </p>
            <div className="pl-5">
              <RadioGroup.Root>
                <RadioGroup.ItemWithLabel
                  value="REQUEST_FIRST"
                  checked={mode === "REQUEST_FIRST"}
                  onClick={() => setMode("REQUEST_FIRST")}
                  label="Request-first mode"
                  description="Start by designing your API structure (endpoints, schemas, etc.) before writing requests. Ideal for planning and generating documentation upfront."
                />
                <RadioGroup.ItemWithLabel
                  value="DESIGN_FIRST"
                  checked={mode === "DESIGN_FIRST"}
                  onClick={() => setMode("DESIGN_FIRST")}
                  label="Design-first mode"
                  description="Begin by writing and testing requests, then define the API structure based on actual usage. Great for quick prototyping and iterating."
                />
              </RadioGroup.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-between py-0.75">
          <CheckboxWithLabel
            label="Reopen this workspace on next session"
            checked={reopenOnSession}
            onCheckedChange={(checked) => {
              if (checked !== "indeterminate") {
                setReopenOnSession(checked);
              }
            }}
          />
          <div className="flex gap-3 px-0.25 py-1.25">
            <ButtonNeutralOutlined onClick={handleCancel}>Cancel</ButtonNeutralOutlined>
            <ButtonPrimary disabled={isSubmitDisabled} type="submit">
              Open
            </ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
