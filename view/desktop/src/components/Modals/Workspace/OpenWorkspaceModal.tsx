import { useState } from "react";

import { Icon, Modal, RadioGroup } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import Select from "@/components/Select";
import { useGetWorkspaces } from "@/hooks/workspaces/useGetWorkspaces";
import { useOpenWorkspace } from "@/hooks/workspaces/useOpenWorkspace";

import { ModalWrapperProps } from "../types";

export const OpenWorkspaceModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { data: workspaces } = useGetWorkspaces();

  const [mode, setMode] = useState<"RequestFirstMode" | "DesignFirstMode">("RequestFirstMode");
  const [selectedWorkspace, setSelectedWorkspace] = useState<string | undefined>(undefined);
  const [openAutomatically, setOpenAutomatically] = useState<boolean>(true);

  const { mutate: openWorkspace } = useOpenWorkspace();

  const handleSubmit = () => {
    if (selectedWorkspace) {
      openWorkspace(selectedWorkspace);
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
    <Modal
      title="Open Workspace"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      content={
        <div className="flex flex-col gap-4">
          <div className="flex items-center gap-x-2 py-1.5">
            <div>Name:</div>

            <Select.Root onValueChange={setSelectedWorkspace} value={selectedWorkspace}>
              <Select.Trigger className="flex w-56 justify-between">
                <Select.Value placeholder="Select workspace" />
                <Icon icon="ChevronDown" />
              </Select.Trigger>

              <Select.Content className="z-50" position="popper">
                <Select.Viewport>
                  {workspaces?.map((workspace) => (
                    <Select.Item value={workspace.name} key={workspace.name}>
                      <Select.ItemText>{workspace.name}</Select.ItemText>
                    </Select.Item>
                  ))}
                </Select.Viewport>
              </Select.Content>
            </Select.Root>
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
