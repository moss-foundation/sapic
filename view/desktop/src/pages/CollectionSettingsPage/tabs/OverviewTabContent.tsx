import { useEffect, useState } from "react";

import { InputOutlined } from "@/components";
import { DeleteCollectionModal } from "@/components/Modals/Collection/DeleteCollectionModal";
import { VALID_NAME_PATTERN } from "@/constants/validation";
import { useModal, useStreamCollections, useUpdateCollection } from "@/hooks";
import { IDockviewPanelProps } from "@/lib/moss-tabs/src";

import { CollectionDangerZoneSection } from "../CollectionDangerZoneSection";
import { CollectionSummarySection } from "../CollectionSummarySection";

interface OverviewTabContentProps {
  collectionId: string;
}

export const OverviewTabContent = ({ params, containerApi }: IDockviewPanelProps<OverviewTabContentProps>) => {
  const { data: streamedCollections } = useStreamCollections();
  const { mutateAsync: updateCollection } = useUpdateCollection();

  const collection = streamedCollections?.find((collection) => collection.id === params.collectionId);

  const { showModal, closeModal, openModal } = useModal();

  const [name, setName] = useState(collection?.name || "");
  const [repository, setRepository] = useState(collection?.repository || "");

  useEffect(() => {
    if (collection) {
      setName(collection.name);
      setRepository(collection?.repository || "");
      const currentPanel = containerApi.getPanel(collection.id);
      currentPanel?.api.setTitle(collection.name);
    }
  }, [collection, containerApi]);

  const handleUpdateCollectionName = async () => {
    if (!collection) return;

    if (!name || name === collection.name) {
      setName(collection?.name);
      return;
    }
    try {
      await updateCollection({
        id: collection.id,
        name,
      });
    } catch (e) {
      console.error("handleUpdateCollectionName", e);
      setName(collection?.name);
    }
  };
  const handleNameBlur = () => {
    handleUpdateCollectionName();
  };

  const handleUpdateCollectionRepository = async () => {
    if (!collection) return;

    if (!repository || repository === collection.repository) {
      setRepository(collection?.repository || "github.com/moss-foundation/sapic");
      return;
    }

    try {
      await updateCollection({
        id: collection.id,
        repository: !repository ? "REMOVE" : { UPDATE: repository },
      });
    } catch (e) {
      console.error("handleUpdateCollectionRepository", e);
      setRepository(collection?.repository || "");
    }
  };

  const handleRepositoryBlur = () => {
    handleUpdateCollectionRepository();
  };

  if (!collection) {
    return (
      <div className="flex h-full items-center justify-center text-(--moss-primary-text)">
        <div className="text-center">
          <h2 className="text-lg font-semibold">No Active Collection</h2>
          <p className="text-sm">Please select a collection to view its settings.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="relative flex h-full justify-center">
      <div className="w-full max-w-2xl space-y-9 px-6 py-5">
        <div className="space-y-6">
          <div className="flex items-start gap-3.5 text-(--moss-primary-text)">
            <label className="mt-1 w-20 font-medium">Name:</label>
            <div>
              <InputOutlined
                value={name}
                onChange={(e) => setName(e.target.value)}
                onBlur={handleNameBlur}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    e.preventDefault();
                    handleUpdateCollectionName();
                    e.currentTarget.blur();
                  }
                }}
                placeholder="Enter collection name..."
                pattern={VALID_NAME_PATTERN}
                className="w-72 border-(--moss-input-border)"
              />
              <p className="mt-1 w-72 text-sm text-(--moss-secondary-text)">
                Invalid filename characters (e.g. / \ : * ? " &lt; &gt; |) will be escaped
              </p>
            </div>
          </div>

          <div className="mt-10 flex items-start gap-3.5 text-(--moss-primary-text)">
            <label className="mt-1 w-20 font-medium">Repository:</label>
            <div>
              <InputOutlined
                value={repository}
                onChange={(e) => setRepository(e.target.value)}
                onBlur={handleRepositoryBlur}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    e.preventDefault();
                    handleUpdateCollectionRepository();
                    e.currentTarget.blur();
                  }
                }}
                placeholder="Enter repository URL..."
                className="w-72 border-(--moss-input-border)"
                required
              />
            </div>
          </div>
        </div>

        <CollectionDangerZoneSection onDeleteClick={openModal} />
      </div>

      {/* Right Column - Summary positioned absolutely on the right */}
      <div className="absolute top-0 right-2 w-60 py-2">
        <CollectionSummarySection />
      </div>

      {showModal && <DeleteCollectionModal showModal={showModal} closeModal={closeModal} id={params.collectionId} />}
    </div>
  );
};
