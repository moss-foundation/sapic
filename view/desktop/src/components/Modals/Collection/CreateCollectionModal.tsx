import { useState } from "react";

import { RadioGroup } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import InputOutlined from "@/components/InputOutlined";
import { ModalForm } from "@/components/ModalForm";
import { useCollectionsStore } from "@/store/collections";

import { ModalWrapperProps } from "../types";

export const CreateCollectionModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { createCollection, isCreateCollectionLoading } = useCollectionsStore();

  const [name, setName] = useState("");
  const [mode, setMode] = useState<"Radio 1" | "Radio 2">("Radio 1");

  const handleSubmit = async () => {
    await createCollection({
      name,
    });
    closeModal();
  };

  const handleCancel = () => {
    closeModal();
    resetForm();
  };

  const resetForm = () => {
    setTimeout(() => {
      setName("");
    }, 200);
  };

  const isSubmitDisabled = name.length === 0 || isCreateCollectionLoading;

  return (
    <ModalForm
      title="Create Collection"
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
              pattern="^[^/:\\*?|]+$"
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
                  label="Radio 1"
                  description="Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quos."
                  value="Radio 1"
                  checked={mode === "Radio 1"}
                  onClick={() => setMode("Radio 1")}
                />

                <RadioGroup.ItemWithLabel
                  label="Radio 2"
                  description="Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quos."
                  value="Radio 2"
                  checked={mode === "Radio 2"}
                  onClick={() => setMode("Radio 2")}
                />
              </RadioGroup.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-end py-0.75">
          <div className="flex gap-3 px-0.25 py-1.25">
            <ButtonNeutralOutlined onClick={handleCancel}>Cancel</ButtonNeutralOutlined>
            <ButtonPrimary disabled={isSubmitDisabled} type="submit">
              Create
            </ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
