import { useState } from "react";

import { RadioGroup } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { ModalForm } from "@/components/ModalForm";
import SelectOutlined from "@/components/SelectOutlined";
import { useWorkspaceContext } from "@/context/WorkspaceContext";
import { useGetWorkspaces } from "@/hooks/workspaces/useGetWorkspaces";

import { ModalWrapperProps } from "../types";

export const OpenWorkspaceModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { data: workspaces } = useGetWorkspaces();

  const [mode, setMode] = useState<"RequestFirstMode" | "DesignFirstMode">("RequestFirstMode");
  const [selectedWorkspace, setSelectedWorkspace] = useState<string | undefined>(undefined);
  const [openAutomatically, setOpenAutomatically] = useState<boolean>(true);

  const { openAndSelectWorkspace } = useWorkspaceContext();

  const handleSubmit = () => {
    if (selectedWorkspace) {
      openAndSelectWorkspace(selectedWorkspace);
      closeModal();
      reset();
    }
  };

  const handleCancel = () => {
    closeModal();
    reset();
  };

  const reset = () => {
    setTimeout(() => {
      setSelectedWorkspace(undefined);
      setMode("RequestFirstMode");
      setOpenAutomatically(true);
    }, 200);
  };

  return (
    <ModalForm
      title="Open Workspace"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      className="background-(--moss-primary-background)"
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
                  <SelectOutlined.Item value={workspace.displayName} key={workspace.displayName}>
                    {workspace.displayName}
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
                  value="RequestFirstMode"
                  checked={mode === "RequestFirstMode"}
                  onClick={() => setMode("RequestFirstMode")}
                  label="Request-first mode"
                  description="Start by designing your API structure (endpoints, schemas, etc.) before writing requests. Ideal for
                    planning and generating documentation upfront."
                />

                <RadioGroup.ItemWithLabel
                  value="DesignFirstMode"
                  checked={mode === "DesignFirstMode"}
                  onClick={() => setMode("DesignFirstMode")}
                  label="Design-first mode"
                  description="Begin by writing and testing requests, then define the API structure based on actual usage. Great
                    for quick prototyping and iterating."
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
            checked={openAutomatically}
            onCheckedChange={(check) => {
              if (check !== "indeterminate") setOpenAutomatically(check);
            }}
          />

          <div className="flex gap-3 px-0.25 py-1.25">
            <ButtonNeutralOutlined onClick={handleCancel}>Close</ButtonNeutralOutlined>
            <ButtonPrimary disabled={!selectedWorkspace} type="submit">
              Open
            </ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
