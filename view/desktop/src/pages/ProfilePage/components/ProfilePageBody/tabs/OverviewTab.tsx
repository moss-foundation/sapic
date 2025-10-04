import { useState } from "react";

import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import ButtonDanger from "@/components/ButtonDanger";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import { useModal } from "@/hooks";
import { invoke } from "@tauri-apps/api/core";
import { UpdateProfileInput } from "@repo/moss-app";
import { AccountInfo, ProfileInfo } from "@repo/moss-user";

import { ConfirmationModal } from "@/components/Modals/ConfirmationModal";
import { ProfilePageProps } from "../../../ProfilePage";
import { NewAccountModal } from "@/components/Modals/Account/NewAccountModal";
import { ProviderIcon } from "@/components/ProviderIcon";

interface OverviewTabProps extends IDockviewPanelProps<ProfilePageProps> {
  profile: ProfileInfo;
}

export const OverviewTab = ({ profile }: OverviewTabProps) => {
  const [showNewAccountModal, setShowNewAccountModal] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [accountToRemove, setAccountToRemove] = useState<AccountInfo | null>(null);

  const { openModal: openRevokeModal, closeModal: closeRevokeModal, showModal: isRevokeModalOpen } = useModal();

  const handleRevokeClick = (account: AccountInfo) => {
    setAccountToRemove(account);
    openRevokeModal();
  };

  const handleEditDetails = (account: AccountInfo) => {
    alert(`Edit details for ${account.username} (${account.kind})\n\nThis feature is not yet implemented.`);
  };

  const handleRemoveAccount = async () => {
    if (!accountToRemove) return;

    try {
      setIsSubmitting(true);
      const input: UpdateProfileInput = {
        accountsToAdd: [],
        accountsToRemove: [accountToRemove.id],
      };
      await invoke("update_profile", { input });
      console.log("Account removed successfully");
      closeRevokeModal();
      window.location.reload();
    } catch (error) {
      console.error("Error removing account:", error);
      alert(`Failed to remove account: ${error}`);
    } finally {
      setIsSubmitting(false);
    }
  };

  const getProviderIcon = (kind: string) => {
    switch (kind) {
      case "GITHUB":
        return <ProviderIcon icon="github" />;
      case "GITLAB":
        return <ProviderIcon icon="gitlab" />;
      default:
        return null;
    }
  };

  return (
    <>
      <NewAccountModal
        showModal={showNewAccountModal}
        closeModal={() => setShowNewAccountModal(false)}
        onAccountAdded={() => {
          console.log("Account added successfully");
        }}
      />

      {isRevokeModalOpen && accountToRemove && (
        <ConfirmationModal
          showModal={isRevokeModalOpen}
          closeModal={closeRevokeModal}
          title="Revoke Account"
          message={`Are you sure you want to remove this account?`}
          description={`This will revoke access for ${accountToRemove.username} (${accountToRemove.kind}).`}
          confirmLabel={isSubmitting ? "Revoking..." : "Revoke"}
          cancelLabel="Cancel"
          onConfirm={handleRemoveAccount}
          variant="danger"
          loading={isSubmitting}
        />
      )}

      <div className="flex flex-col gap-6">
        {/* Accounts Section */}
        <section>
          <div className="mb-4 flex items-center justify-between">
            <h2 className="text-[16px] font-normal">Accounts</h2>
            <ButtonPrimary onClick={() => setShowNewAccountModal(true)}>Connect</ButtonPrimary>
          </div>

          {/* Divider and Description */}
          <div className="-mt-2 flex flex-col gap-2.5">
            <div className="background-(--moss-border-color) h-px w-full" />
            <p className="text-sm text-(--moss-secondary-text)">Manage your connected accounts</p>
          </div>

          {/* Accounts List */}
          <div className="mt-2.5 flex flex-col gap-2">
            {profile.accounts.length === 0 ? (
              <div className="rounded-sm border border-(--moss-border-color) p-6 text-center text-sm text-(--moss-secondary-text)">
                <p>No accounts connected yet</p>
              </div>
            ) : (
              profile.accounts.map((account: AccountInfo) => (
                <div
                  key={account.id}
                  className="flex items-center justify-between rounded-md border border-(--moss-border-color) px-3 py-2.5"
                >
                  <div className="flex items-center gap-2">
                    {getProviderIcon(account.kind)}
                    <span className="text-sm">{account.username}</span>
                  </div>
                  <div className="flex items-center gap-3">
                    <ButtonNeutralOutlined
                      onClick={() => handleEditDetails(account)}
                      className="background-(--moss-secondary-background) border-(--moss-border-color) px-3"
                    >
                      Edit details
                    </ButtonNeutralOutlined>
                    <ButtonDanger onClick={() => handleRevokeClick(account)} disabled={isSubmitting}>
                      Revoke
                    </ButtonDanger>
                  </div>
                </div>
              ))
            )}
          </div>
        </section>
      </div>
    </>
  );
};
