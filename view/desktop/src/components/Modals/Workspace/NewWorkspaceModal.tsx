import { useState } from "react";

import { Button, Checkbox, Icon, Input, Modal, Radio } from "@/components";
import { useCreateWorkspace } from "@/hooks/useCreateWorkspace";

export const NewWorkspaceModal = ({ closeModal, showModal }: { showModal: boolean; closeModal: () => void }) => {
  const { mutate: createWorkspace } = useCreateWorkspace();

  const [name, setName] = useState("");
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

  const handleSubmit = async () => {
    if (name) {
      createWorkspace({ name });
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
      setName("");
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
      title="New Workspace"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      content={
        <div className="flex flex-col gap-2">
          <div className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,1fr)] items-center gap-3">
            <div>Name:</div>
            <Input
              value={name}
              variant="outlined"
              className="max-w-72"
              required
              onChange={(e) => setName(e.target.value)}
            />
            <p className="col-start-2 max-w-72 text-xs text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
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
                          list.map((item) =>
                            item.id === radio.id ? { ...item, checked: true } : { ...item, checked: false }
                          )
                        )
                      }
                    >
                      <Radio.Indicator>
                        <Icon icon="DropdownMenuRadioIndicator" className="size-2 text-white" />
                      </Radio.Indicator>
                    </Radio.Item>

                    <label htmlFor={radio.id} className="cursor-pointer py-2">
                      {radio.label}
                    </label>
                    <p className="col-start-2 text-left text-(--moss-secondary-text)">{radio.description}</p>
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
            <Checkbox.Root id="c1" className="cursor-pointer">
              <Checkbox.Indicator>
                <Icon icon="CheckboxIndicator" className="size-3.5 text-white" />
              </Checkbox.Indicator>
            </Checkbox.Root>
            <label htmlFor="c1" className="cursor-pointer">
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
              Create
            </button>
          </div>
        </div>
      }
    />
  );
};
