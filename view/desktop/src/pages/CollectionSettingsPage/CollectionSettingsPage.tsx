import { useState, useEffect } from "react";

import { ConfirmationModal } from "@/components/Modals/ConfirmationModal";
import { PageContainerWithTabs, TabItem } from "@/components/PageContainer";
import { Icon } from "@/lib/ui";
import { useUpdateCollection, useDeleteCollection, useActiveCollection } from "@/hooks/collection";
import { Collection } from "@/components/CollectionTree/types";

import { OverviewTabContent } from "./OverviewTabContent";
import { AuthTabContent } from "./AuthTabContent";
import { HeadersTabContent } from "./HeadersTabContent";
import { VariablesTabContent } from "./VariablesTabContent";
import { PreRequestTabContent } from "./PreRequestTabContent";
import { PostRequestTabContent } from "./PostRequestTabContent";

// Badge component for tab numbers
const Badge = ({ count }: { count: number }) => (
  <span className="ml-1 rounded-full bg-(--moss-info-background) px-1.5 py-0.5 text-xs text-(--moss-primary)">
    {count}
  </span>
);

// Indicator dot for status
const StatusDot = ({ active }: { active: boolean }) =>
  active ? <div className="background-(--moss-auth-indicator-color) ml-1 h-2 w-2 rounded-full" /> : null;

export const CollectionSettings = () => {
  const collection = useActiveCollection();
  const { mutate: updateCollection } = useUpdateCollection();
  const { mutate: deleteCollection } = useDeleteCollection();

  const [name, setName] = useState("");
  const [repository, setRepository] = useState("github.com/moss-foundation/sapic");
  const [showDeleteConfirmModal, setShowDeleteConfirmModal] = useState(false);
  const [activeTabId, setActiveTabId] = useState("overview");

  useEffect(() => {
    if (collection) {
      // Get display name from collection
      const displayName = typeof collection.id === "string" ? collection.id : `Collection ${collection.id}`;
      setName(displayName);
    }
  }, [collection]);

  const handleSave = () => {
    if (name.trim() && collection && name.trim() !== getCollectionDisplayName(collection)) {
      const collectionId = typeof collection.id === "string" ? collection.id : String(collection.id);
      updateCollection(
        { collectionId, name: name.trim() },
        {
          onError: (error) => {
            console.error("Failed to update collection:", error.message);
          },
        }
      );
    }
  };

  // Auto-save when input loses focus
  const handleBlur = () => {
    handleSave();
  };

  // Delete collection handlers
  const handleDeleteClick = () => {
    setShowDeleteConfirmModal(true);
  };

  const handleDeleteCollection = () => {
    if (collection) {
      const collectionId = typeof collection.id === "string" ? collection.id : String(collection.id);
      deleteCollection({ id: collectionId });
      setShowDeleteConfirmModal(false);
    }
  };

  const closeDeleteConfirmModal = () => {
    setShowDeleteConfirmModal(false);
  };

  // Helper function to get collection display name
  const getCollectionDisplayName = (collection: Collection) => {
    return typeof collection.id === "string" ? collection.id : `Collection ${collection.id}`;
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

  const collectionDisplayName = getCollectionDisplayName(collection);

  // Define the tabs for the PageContainer matching the design
  const tabs: TabItem[] = [
    {
      id: "overview",
      label: (
        <div className="flex items-center gap-1.5">
          <Icon icon="Home" className="h-4 w-4" />
          <span>Overview</span>
        </div>
      ),
      content: (
        <OverviewTabContent
          name={name}
          setName={setName}
          repository={repository}
          setRepository={setRepository}
          onSave={handleSave}
          onBlur={handleBlur}
          onDeleteClick={handleDeleteClick}
        />
      ),
    },
    {
      id: "auth",
      label: (
        <div className="flex items-center gap-1.5">
          <Icon icon="Key" className="h-4 w-4" />
          <span>Auth</span>
          <StatusDot active={true} />
        </div>
      ),
      content: <AuthTabContent />,
    },
    {
      id: "headers",
      label: (
        <div className="flex items-center gap-1.5">
          <Icon icon="ConfigMap" className="h-4 w-4" />
          <span>Headers</span>
          <Badge count={3} />
        </div>
      ),
      content: <HeadersTabContent />,
    },
    {
      id: "variables",
      label: (
        <div className="flex items-center gap-1.5">
          <Icon icon="ToolBarVariables" className="h-4 w-4" />
          <span>Variables</span>
          <Badge count={3} />
        </div>
      ),
      content: <VariablesTabContent />,
    },
    {
      id: "pre-request",
      label: (
        <div className="flex items-center gap-1.5">
          <Icon icon="JsonPath" className="h-4 w-4" />
          <span>Pre Request</span>
        </div>
      ),
      content: <PreRequestTabContent />,
    },
    {
      id: "post-request",
      label: (
        <div className="flex items-center gap-1.5">
          <Icon icon="JsonPath" className="h-4 w-4" />
          <span>Post Request</span>
        </div>
      ),
      content: <PostRequestTabContent />,
    },
  ];

  return (
    <>
      <ConfirmationModal
        showModal={showDeleteConfirmModal}
        closeModal={closeDeleteConfirmModal}
        title="Delete"
        message={`Delete "${collectionDisplayName}"?`}
        description="This will delete all requests, endpoints, and other items in this collection. This action cannot be undone."
        confirmLabel="Delete"
        cancelLabel="Close"
        onConfirm={handleDeleteCollection}
        variant="danger"
      />

      <PageContainerWithTabs tabs={tabs} activeTabId={activeTabId} onTabChange={setActiveTabId} />
    </>
  );
};
