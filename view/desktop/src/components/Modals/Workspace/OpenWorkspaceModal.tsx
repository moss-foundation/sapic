import { useState } from "react";

import { Button, Checkbox, Icon, Modal, Radio } from "@/components";
import Select from "@/components/Select";
import { useGetWorkspaces } from "@/hooks/useGetWorkspaces";
import { useOpenWorkspace } from "@/hooks/useOpenWorkspace";

export const OpenWorkspaceModal = ({ closeModal, showModal }: { showModal: boolean; closeModal: () => void }) => {
  const { data: workspaces } = useGetWorkspaces();

  const [radioList, setRadioList] = useState([
    {
      id: "RequestFirstMode",
      label: "Request-first mode",
      description:
        "Start by designing your API structure (endpoints, schemas, etc.) before writing requests. Ideal for planning and generating documentation upfront.",
      checked: true,
    },
    {
      id: "DesignFirstMode",
      label: "Design-first mode",
      description:
        "Begin by writing and testing requests, then define the API structure based on actual usage. Great for quick prototyping and iterating.",
      checked: false,
    },
  ]);

  const [selectedWorkspace, setSelectedWorkspace] = useState<string | undefined>(undefined);

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
      setRadioList((list) =>
        list.map((item) => {
          return {
            ...item,
            checked: item.id === "RequestFirstMode",
          };
        })
      );
    }, 100);
  };

  return (
    <Modal
      title="Open Workspace"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      content={
        <div className="flex flex-col gap-2">
          <div className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(1,1fr)] items-center gap-3">
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
            <div className="pl-5">
              <Radio.Root>
                {radioList.map((radio) => (
                  <div
                    key={radio.id}
                    className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,min-content)] items-center gap-x-2"
                  >
                    <Radio.Item
                      value={radio.id}
                      id={radio.id}
                      checked={radio.checked}
                      onClick={() =>
                        setRadioList((list) =>
                          list.map((item) => {
                            return {
                              ...item,
                              checked: item.id === radio.id,
                            };
                          })
                        )
                      }
                    >
                      <Radio.Indicator>
                        <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
                      </Radio.Indicator>
                    </Radio.Item>

                    <label
                      htmlFor={radio.id}
                      className="cursor-pointer py-2"
                      onClick={() =>
                        setRadioList((list) =>
                          list.map((item) => {
                            return {
                              ...item,
                              checked: item.id === radio.id,
                            };
                          })
                        )
                      }
                    >
                      {radio.label}
                    </label>
                    <span className="col-start-2 text-left text-(--moss-secondary-text)">{radio.description}</span>
                  </div>
                ))}
              </Radio.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-between">
          <div className="flex gap-2">
            <Checkbox.Root id="OpenAutomaticallyAfterCreationId" className="cursor-pointer">
              <Checkbox.Indicator>
                <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
              </Checkbox.Indicator>
            </Checkbox.Root>
            <label htmlFor="OpenAutomaticallyAfterCreationId" className="cursor-pointer">
              Open automatically after creation
            </label>
          </div>
          <div className="flex gap-2">
            <Button variant="outlined" intent="neutral" onClick={handleCancel}>
              Close
            </Button>
            {/* //TODO This should be a button component */}
            <button
              type="submit"
              className="background-(--moss-primary) hover:background-(--moss-blue-3) flex cursor-pointer items-center justify-center rounded px-3.75 py-1.5 text-white"
            >
              Open
            </button>
          </div>
        </div>
      }
    />
  );
};
