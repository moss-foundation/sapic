import { useEffect, useState } from "react";

import { Button, Checkbox, Icon, Modal, Radio } from "@/components";
import Select from "@/components/Select";
import { useGetWorkspaces } from "@/hooks/useGetWorkspaces";
import { useOpenWorkspace } from "@/hooks/useOpenWorkspace";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { useWorkspaceStore } from "@/store/workspace";

import { ModalWrapperProps } from "../types";

export const OpenWorkspaceModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { data: workspaces } = useGetWorkspaces();

  const [mode, setMode] = useState<"RequestFirstMode" | "DesignFirstMode">("RequestFirstMode");
  const [selectedWorkspace, setSelectedWorkspace] = useState<string | undefined>(undefined);
  const [openAutomatically, setOpenAutomatically] = useState<boolean>(true);

  const { mutate: openWorkspace, data: currentWorkspace } = useOpenWorkspace();

  const { setWorkspace } = useWorkspaceStore();
  const { api } = useTabbedPaneStore();

  useEffect(() => {
    if (currentWorkspace?.path) {
      setWorkspace(currentWorkspace.path);
      const WelcomePanel = api?.getPanel("WelcomePage");
      if (WelcomePanel) {
        WelcomePanel.api.close();
      }
    }
  }, [currentWorkspace, setWorkspace]);

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
              <Radio.Root>
                <div className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,min-content)] items-center gap-x-2">
                  <Radio.Item
                    value="RequestFirstMode"
                    id="RequestFirstModeRadioOpenWorkspace"
                    checked={mode === "RequestFirstMode"}
                    onClick={() => setMode("RequestFirstMode")}
                  >
                    <Radio.Indicator>
                      <Icon icon="DropdownMenuRadioIndicator" className="size-2! text-white" />
                    </Radio.Indicator>
                  </Radio.Item>

                  <label htmlFor="RequestFirstModeRadioOpenWorkspace" className="cursor-pointer py-2">
                    Request-first mode
                  </label>
                  <p className="col-start-2 text-left text-xs leading-3.75 text-(--moss-secondary-text)">
                    Start by designing your API structure (endpoints, schemas, etc.) before writing requests. Ideal for
                    planning and generating documentation upfront.
                  </p>
                </div>
                <div className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,min-content)] items-center gap-x-2">
                  <Radio.Item
                    value="DesignFirstMode"
                    id="DesignFirstModeRadioOpenWorkspace"
                    checked={mode === "DesignFirstMode"}
                    onClick={() => setMode("DesignFirstMode")}
                  >
                    <Radio.Indicator>
                      <Icon icon="DropdownMenuRadioIndicator" className="size-2! text-white" />
                    </Radio.Indicator>
                  </Radio.Item>

                  <label htmlFor="DesignFirstModeRadioOpenWorkspace" className="cursor-pointer py-2">
                    Design-first mode
                  </label>
                  <p className="col-start-2 text-left text-xs leading-3.75 text-(--moss-secondary-text)">
                    Begin by writing and testing requests, then define the API structure based on actual usage. Great
                    for quick prototyping and iterating.
                  </p>
                </div>
              </Radio.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-between py-0.75">
          <div className="flex gap-2">
            <Checkbox.Root
              id="OpenAutomaticallyAfterCreationId"
              className="cursor-pointer"
              checked={openAutomatically}
              onCheckedChange={(check) => {
                if (check !== "indeterminate") setOpenAutomatically(check);
              }}
            >
              <Checkbox.Indicator className="size-4">
                <Icon icon="CheckboxIndicator" className="mx-auto mt-0.25 size-3.5 text-white" />
              </Checkbox.Indicator>
            </Checkbox.Root>
            <label htmlFor="OpenAutomaticallyAfterCreationId" className="cursor-pointer">
              Reopen this workspace on next session
            </label>
          </div>
          <div className="flex gap-3 px-0.25 py-1.25">
            <Button variant="outlined" intent="neutral" size="md" onClick={handleCancel}>
              Close
            </Button>
            {/* //TODO This should be a button component */}
            <button
              disabled={!selectedWorkspace}
              type="submit"
              className="background-(--moss-primary) hover:background-(--moss-blue-3) disabled:background-(--moss-gray-12) disabled:hover:background-(--moss-gray-12) flex cursor-pointer items-center justify-center rounded px-3.75 text-white disabled:cursor-not-allowed disabled:text-(--moss-gray-8)"
            >
              Open
            </button>
          </div>
        </div>
      }
    />
  );
};
