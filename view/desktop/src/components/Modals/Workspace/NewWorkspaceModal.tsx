import { useState } from "react";

import { Button, Checkbox, Icon, Input, Modal, Radio } from "@/components";
import { useCreateWorkspace } from "@/hooks/workspaces/useCreateWorkspace";

import { ModalWrapperProps } from "../types";

export const NewWorkspaceModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { mutate: createWorkspace } = useCreateWorkspace();

  const [name, setName] = useState("");
  const [mode, setMode] = useState<"RequestFirstMode" | "DesignFirstMode">("RequestFirstMode");
  const [openAutomatically, setOpenAutomatically] = useState(true);

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
      setMode("RequestFirstMode");
      setOpenAutomatically(true);
    }, 200);
  };

  return (
    <Modal
      title="New Workspace"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      content={
        <div className="flex flex-col gap-2">
          <div className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,1fr)] items-center gap-x-3.75 py-4">
            <div className="self-start">Name:</div>
            <Input
              value={name}
              variant="outlined"
              className="max-w-72"
              onChange={(e) => setName(e.target.value)}
              pattern={'^[^\\/:\\*\\?"><>|]+$'}
              required
            />
            <p className="col-start-2 max-w-72 text-xs text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
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
                    id="RequestFirstModeRadioNewWorkspace"
                    checked={mode === "RequestFirstMode"}
                    onClick={() => setMode("RequestFirstMode")}
                  >
                    <Radio.Indicator>
                      <Icon icon="DropdownMenuRadioIndicator" className="size-2! text-white" />
                    </Radio.Indicator>
                  </Radio.Item>

                  <label htmlFor="RequestFirstModeRadioNewWorkspace" className="cursor-pointer py-2">
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
                    id="DesignFirstModeRadioNewWorkspace"
                    checked={mode === "DesignFirstMode"}
                    onClick={() => setMode("DesignFirstMode")}
                  >
                    <Radio.Indicator>
                      <Icon icon="DropdownMenuRadioIndicator" className="size-2! text-white" />
                    </Radio.Indicator>
                  </Radio.Item>

                  <label htmlFor="DesignFirstModeRadioNewWorkspace" className="cursor-pointer py-2">
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
              id="c1"
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
            <label htmlFor="c1" className="cursor-pointer">
              Open automatically after creation
            </label>
          </div>
          <div className="flex gap-3 px-0.25 py-1.25">
            <Button variant="outlined" intent="neutral" size="md" onClick={handleCancel}>
              Close
            </Button>
            {/* //TODO This should be a button component */}
            <button
              disabled={name.length === 0}
              type="submit"
              className="background-(--moss-primary) hover:background-(--moss-blue-3) disabled:background-(--moss-gray-12) disabled:hover:background-(--moss-gray-12) flex cursor-pointer items-center justify-center rounded px-3.75 text-white disabled:cursor-not-allowed disabled:text-(--moss-gray-8)"
            >
              Create
            </button>
          </div>
        </div>
      }
    />
  );
};
