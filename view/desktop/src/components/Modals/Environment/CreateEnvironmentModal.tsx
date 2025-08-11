import { useRef, useState } from "react";

import { ButtonNeutralOutlined, ButtonPrimary, InputOutlined, RadioGroup } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { ModalForm } from "@/components/ModalForm";
import { VALID_NAME_PATTERN } from "@/constants/validation";
import { useStreamedCollections } from "@/hooks";
import { useCreateEnvironment } from "@/hooks/environment";

import { ModalWrapperProps } from "../types";

export const CreateEnvironmentModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const { mutateAsync: createEnvironment } = useCreateEnvironment();
  const { data: collections } = useStreamedCollections();

  const [name, setName] = useState("");
  const [collectionId, setCollectionId] = useState<string | null>(null);
  const [mode, setMode] = useState<"Workspace" | "Collection">("Workspace");
  const [openAutomatically, setOpenAutomatically] = useState(true);

  const handleSubmit = async () => {
    const newEnvironment = await createEnvironment({
      name,
      order: 0,
      variables: [],
      collectionId: collectionId ?? undefined,
    });

    if (newEnvironment) {
      closeModal();
    }
  };
  const handleCancel = () => {
    closeModal();
  };
  const handleSelectCollection = (value: string) => {
    setCollectionId(value);
    setMode("Collection");
  };

  return (
    <ModalForm
      title="New Environment"
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
                ref={inputRef}
                value={name}
                className="max-w-72"
                onChange={(e) => setName(e.target.value)}
                pattern={VALID_NAME_PATTERN}
                required
              />
              <p className="col-start-2 max-w-72 text-xs text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
            </div>
          </div>

          <div>
            <div className="flex gap-2">
              <span>Scope</span>
              <div className="background-(--moss-border-color) my-auto h-px w-full" />
            </div>
            <p className="text-xs leading-5 text-(--moss-secondary-text)">
              You can switch modes in the workspace at any time and as often as needed.
            </p>
            <div className="pl-5">
              <RadioGroup.Root>
                <RadioGroup.ItemWithLabel
                  label="Workspace"
                  description="This mode is suitable when your collection is stored in a separate repository or doesn’t have a repository at all."
                  value="Workspace"
                  checked={mode === "Workspace"}
                  onClick={() => setMode("Workspace")}
                />

                <RadioGroup.ItemWithSelect
                  placeholder="MyCollection"
                  label="Collection"
                  description="This mode is suitable if you want to store the collection in your project’s repository or in any other folder you specify."
                  value="Collection"
                  checked={mode === "Collection"}
                  onClick={() => setMode("Collection")}
                  disabled={!collections}
                  options={collections?.map((collection) => ({
                    label: collection.name,
                    value: collection.id,
                  }))}
                  selectValue={collectionId ?? undefined}
                  onChange={handleSelectCollection}
                />
              </RadioGroup.Root>
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-between py-0.75">
          <CheckboxWithLabel
            label="Activate after creation"
            checked={openAutomatically}
            onCheckedChange={(check) => {
              if (check !== "indeterminate") setOpenAutomatically(check);
            }}
          />
          <div className="flex gap-3 px-0.25 py-1.25">
            <ButtonNeutralOutlined type="button" onClick={handleCancel}>
              Close
            </ButtonNeutralOutlined>
            <ButtonPrimary type="submit">Create</ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
