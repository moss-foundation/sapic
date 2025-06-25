import { useState } from "react";

import { RadioGroup } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import InputOutlined from "@/components/InputOutlined";
import { ModalForm } from "@/components/ModalForm";
import { useCollectionsStore } from "@/store/collections";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { ModalWrapperProps } from "../types";

export const CreateCollectionModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { createCollection, isCreateCollectionLoading } = useCollectionsStore();
  const { addOrFocusPanel } = useTabbedPaneStore();

  const [name, setName] = useState("New Collection");
  const [repo, setRepo] = useState("");
  const [mode, setMode] = useState<"Default" | "Custom">("Default");
  const [openAutomatically, setOpenAutomatically] = useState(true);

  const handleSubmit = async () => {
    const result = await createCollection({
      name,
    });

    closeModal();

    if (openAutomatically) {
      addOrFocusPanel({
        id: result.id,
        title: name,
        component: "CollectionSettings",
        params: {
          collectionId: result.id,
        },
      });
    }
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
      title="New Collection"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      className="background-(--moss-primary-background)"
      titleClassName="border-b border-(--moss-border-color)"
      footerClassName="border-t border-(--moss-border-color)"
      content={
        <div className="flex flex-col gap-2">
          <div className="grid grid-cols-[min-content_1fr] items-center gap-x-3.75 gap-y-5 py-5">
            <div className="col-span-2 grid grid-cols-subgrid items-center gap-y-3">
              <div>Name:</div>
              <InputOutlined
                value={name}
                className="max-w-72"
                onChange={(e) => setName(e.target.value)}
                pattern="[A-Za-z0-9\s]+"
                required
              />
              <p className="col-start-2 max-w-72 text-xs text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
            </div>

            <div className="col-span-2 grid grid-cols-subgrid items-center">
              <div>Repository:</div>
              <InputOutlined value={repo} className="max-w-72" onChange={(e) => setRepo(e.target.value)} />
            </div>
          </div>

          <div className="cursor-not-allowed opacity-50">
            <div className="flex gap-2">
              <span>Mode</span>
              <div className="background-(--moss-border-color) my-auto h-px w-full" />
            </div>
            <p className="text-xs leading-5 text-(--moss-secondary-text)">
              You can switch modes in the workspace at any time and as often as needed.
            </p>
            <div className="pl-5">
              <RadioGroup.Root disabled>
                <RadioGroup.ItemWithLabel
                  label="Default"
                  description="This mode is suitable when your collection is stored in a separate repository or doesn’t have a repository at all."
                  value="Default"
                  checked={mode === "Default"}
                  onClick={() => setMode("Default")}
                  disabled
                />

                <RadioGroup.ItemWithLabel
                  label="Custom"
                  description="This mode is suitable if you want to store the collection in your project’s repository or in any other folder you specify."
                  value="Custom"
                  checked={mode === "Custom"}
                  onClick={() => setMode("Custom")}
                  disabled
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
            <ButtonPrimary disabled={isSubmitDisabled} type="submit">
              Create
            </ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
