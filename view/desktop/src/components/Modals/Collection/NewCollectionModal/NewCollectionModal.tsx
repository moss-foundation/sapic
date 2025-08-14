import { FormEvent, useEffect, useRef, useState } from "react";

import PaddedTabs from "@/components/PaddedTabs/PaddedTabs";
import { useCreateCollection } from "@/hooks/collection/useCreateCollection";
import { useStreamedCollections } from "@/hooks/collection/useStreamedCollections";
import { Modal } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { ModalWrapperProps } from "../../types";
import { CreateSection } from "./CreateSection/CreateSection";
import { Divider } from "./Divider";
import { FooterActions } from "./FooterActions";
import { Header } from "./Header";
import { ImportSection } from "./ImportSection/ImportSection";
import { ModeRadioGroup } from "./ModeRadioGroup";

export const NewCollectionModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const { mutateAsync: createCollection, isPending: isCreateCollectionLoading } = useCreateCollection();
  const { data: collections } = useStreamedCollections();

  const { addOrFocusPanel } = useTabbedPaneStore();

  const [name, setName] = useState("New Collection");
  const [repository, setRepository] = useState("github.com/moss-foundation/sapic");
  const [mode, setMode] = useState<"Default" | "Custom">("Default");
  const [openAutomatically, setOpenAutomatically] = useState(true);

  const [tab, setTab] = useState("Create");

  useEffect(() => {
    if (!inputRef.current) return;
    inputRef.current.focus();
    inputRef.current.select();
  }, []);

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const result = await createCollection({
      name: name.trim(),
      repository: repository.trim(),
      order: collections?.length ? collections.length + 1 : 1,
    });

    closeModal();

    if (openAutomatically) {
      addOrFocusPanel({
        id: result.id,
        title: name.trim(),
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
    <Modal onBackdropClick={handleCancel} showModal={showModal}>
      <Header />

      <Divider />

      <form onSubmit={handleSubmit} className="flex w-[544px] flex-col gap-1">
        <PaddedTabs.Root value={tab} onValueChange={setTab}>
          <PaddedTabs.List className="border-b border-(--moss-border-color) px-3">
            <PaddedTabs.Trigger value="Create">Create</PaddedTabs.Trigger>
            <PaddedTabs.Trigger value="Import">Import</PaddedTabs.Trigger>
          </PaddedTabs.List>

          <div className="px-6 pt-3">
            <PaddedTabs.Content value="Create">
              <CreateSection />
            </PaddedTabs.Content>
            <PaddedTabs.Content value="Import">
              <ImportSection />
            </PaddedTabs.Content>{" "}
          </div>
        </PaddedTabs.Root>

        <div className="px-6 pb-3">
          <ModeRadioGroup mode={mode} setMode={setMode} />
        </div>

        <Divider />

        <FooterActions
          openAutomatically={openAutomatically}
          setOpenAutomatically={setOpenAutomatically}
          handleCancel={handleCancel}
          isSubmitDisabled={isSubmitDisabled}
        />
      </form>
    </Modal>
  );
};
