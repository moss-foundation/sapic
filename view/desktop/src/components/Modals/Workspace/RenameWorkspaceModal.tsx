import { useState } from "react";

import { InputOutlined } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import { ModalForm } from "@/components/ModalForm";
import { useUpdateWorkspace } from "@/hooks/workbench/useUpdateWorkspace";

interface RenameWorkspaceModalProps {
  showModal: boolean;
  closeModal: () => void;
  workspaceId: string;
  currentName: string;
}

export const RenameWorkspaceModal = ({ 
  closeModal, 
  showModal, 
  currentName 
}: RenameWorkspaceModalProps) => {
  const { mutate: updateWorkspace, isPending } = useUpdateWorkspace();
  const [name, setName] = useState(currentName);

  const handleSubmit = () => {
    if (name.trim() && name.trim() !== currentName) {
      updateWorkspace(
        { name: name.trim() },
        {
          onSuccess: () => {
            closeModal();
            resetForm();
          },
          onError: (error) => {
            console.error("Failed to rename workspace:", error.message);
          },
        }
      );
    } else {
      closeModal();
      resetForm();
    }
  };

  const handleCancel = () => {
    closeModal();
    resetForm();
  };

  const resetForm = () => {
    setTimeout(() => {
      setName(currentName);
    }, 200);
  };

  const isSubmitDisabled = !name.trim() || name.trim() === currentName || isPending;

  return (
    <ModalForm
      showModal={showModal}
      onBackdropClick={handleCancel}
      title="Rename Workspace"
      content={
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-(--moss-primary-text) mb-2">
              Workspace Name
            </label>
            <InputOutlined
              size="md"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Enter workspace name..."
              autoFocus
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  e.preventDefault();
                  handleSubmit();
                } else if (e.key === "Escape") {
                  e.preventDefault();
                  handleCancel();
                }
              }}
            />
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-end gap-3 py-0.75">
          <ButtonNeutralOutlined onClick={handleCancel}>
            Cancel
          </ButtonNeutralOutlined>
          <ButtonPrimary disabled={isSubmitDisabled} onClick={handleSubmit}>
            {isPending ? "Renaming..." : "Rename"}
          </ButtonPrimary>
        </div>
      }
    />
  );
}; 