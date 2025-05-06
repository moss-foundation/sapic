import { useState } from "react";

import { Input, Modal, RadioGroup } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
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
              pattern=""
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
              <RadioGroup.Root>
                <RadioGroup.ItemWithLabel
                  label="Request-first mode"
                  description="Start by designing your API structure (endpoints, schemas, etc.) before writing requests. Ideal for
                    planning and generating documentation upfront."
                  value="RequestFirstMode"
                  checked={mode === "RequestFirstMode"}
                  onClick={() => setMode("RequestFirstMode")}
                />

                <RadioGroup.ItemWithLabel
                  label="Design-first mode"
                  description="Begin by writing and testing requests, then define the API structure based on actual usage. Great
                    for quick prototyping and iterating."
                  value="DesignFirstMode"
                  checked={mode === "DesignFirstMode"}
                  onClick={() => setMode("DesignFirstMode")}
                />
              </RadioGroup.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-between py-0.75">
          <CheckboxWithLabel
            label="Open automatically after creation"
            checked={openAutomatically}
            onCheckedChange={(check) => {
              if (check !== "indeterminate") setOpenAutomatically(check);
            }}
          />
          <div className="flex gap-3 px-0.25 py-1.25">
            <ButtonNeutralOutlined onClick={handleCancel}>Close</ButtonNeutralOutlined>
            <ButtonPrimary disabled={name.length === 0} type="submit">
              Create
            </ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
