import { useState } from "react";

import { InputOutlined, RadioGroup } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { ModalForm } from "@/components/ModalForm";
import { useCreateWorkspace } from "@/hooks/workbench/useCreateWorkspace";
import { useOpenWorkspace } from "@/hooks/workbench/useOpenWorkspace";
import { WorkspaceMode } from "@repo/moss-workspace";

import { ModalWrapperProps } from "../types";

export const NewWorkspaceModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { mutate: createWorkspace } = useCreateWorkspace();
  const { mutate: openWorkspace } = useOpenWorkspace();

  const [name, setName] = useState("");
  const [mode, setMode] = useState<WorkspaceMode>("REQUEST_FIRST");
  const [openAutomatically, setOpenAutomatically] = useState(true);

  const handleSubmit = async () => {
    if (name) {
      createWorkspace(
        {
          name,
          mode,
          openOnCreation: openAutomatically,
        },
        {
          onSuccess: (data) => {
            if (openAutomatically) {
              openWorkspace(data.id);
            }
          },
        }
      );
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
      setMode("REQUEST_FIRST");
      setOpenAutomatically(true);
    }, 200);
  };

  return (
    <ModalForm
      title="New Workspace"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      className="background-(--moss-primary-background)"
      titleClassName="border-b border-(--moss-border-color)"
      footerClassName="border-t border-(--moss-border-color)"
      content={
        <div className="flex flex-col gap-2">
          <div className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,1fr)] items-center gap-x-3.75 py-4">
            <div className="self-start">Name:</div>
            <InputOutlined
              value={name}
              className="max-w-72"
              onChange={(e) => setName(e.target.value)}
              pattern="^[^/:\*?|]+$"
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
                  value="REQUEST_FIRST"
                  checked={mode === "REQUEST_FIRST"}
                  onClick={() => setMode("REQUEST_FIRST")}
                />

                <RadioGroup.ItemWithLabel
                  label="Design-first mode"
                  description="Begin by writing and testing requests, then define the API structure based on actual usage. Great
                    for quick prototyping and iterating."
                  value="DESIGN_FIRST"
                  checked={mode === "DESIGN_FIRST"}
                  onClick={() => setMode("DESIGN_FIRST")}
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
            <ButtonNeutralOutlined type="button" onClick={handleCancel}>
              Close
            </ButtonNeutralOutlined>
            <ButtonPrimary disabled={name.length === 0} type="submit">
              Create
            </ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
