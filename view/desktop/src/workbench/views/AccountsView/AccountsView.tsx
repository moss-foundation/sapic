import { useUpdateProfile } from "@/adapters/tanstackQuery/user";
import { useDescribeApp, useModal } from "@/hooks";
import { Button } from "@/lib/ui";
import { cn } from "@/utils";
import { ConfirmationModal, PageView } from "@/workbench/ui/components";
import { EditAccountModal } from "@/workbench/ui/components/Modals/Account/EditAccountModal";
import { NewAccountModal } from "@/workbench/ui/components/Modals/Account/NewAccountModal";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";
import { ProviderIcon } from "@/workbench/ui/components/ProviderIcon";
import { DefaultViewProps } from "@/workbench/ui/parts/TabbedPane/types";
import { AccountInfo } from "@repo/base";
import { UpdateProfileInput } from "@repo/window";

export type AccountsViewProps = DefaultViewProps;

export const AccountsView = ({}: AccountsViewProps) => {
  const { data: appState, isLoading, error } = useDescribeApp();
  const profile = appState?.profile;

  const {
    openModal: openNewAccountModal,
    closeModal: closeNewAccountModal,
    showModal: isNewAccountModalOpen,
  } = useModal();

  if (error) {
    console.error("ProfileView error:", error);
  }

  if (isLoading) {
    return (
      <PageView>
        <PageWrapper>
          <div className="flex flex-1 items-center justify-center">
            <div className="text-center">
              <p className="text-(--moss-secondary-foreground) mb-4 text-sm">Loading profile...</p>
            </div>
          </div>
        </PageWrapper>
      </PageView>
    );
  }

  if (!profile) {
    return (
      <PageView>
        <PageWrapper>
          <div className="flex flex-1 items-center justify-center">
            <div className="text-center">
              <p className="text-(--moss-secondary-foreground) mb-4 text-sm">
                {error ? "Error loading profile" : "No profile found"}
              </p>
            </div>
          </div>
        </PageWrapper>
      </PageView>
    );
  }

  return (
    <>
      <PageView>
        <PageWrapper>
          <header className="flex items-center justify-between gap-2 pb-3.5 pt-1">
            <h1 className="text-lg font-medium">Accounts</h1>
            <Button intent="primary" onClick={openNewAccountModal}>
              Connect
            </Button>
          </header>

          <div className="flex flex-col gap-6">
            {/* Accounts Section */}
            <section>
              {/* Divider and Description */}
              <div className="flex flex-col gap-2.5">
                <div className="background-(--moss-border) h-px w-full" />
                <p className="text-(--moss-secondary-foreground) text-sm">Manage your connected accounts</p>
              </div>

              {/* Accounts List */}
              <div className="mt-2.5">
                {profile.accounts.length === 0 ? (
                  <div className="border-(--moss-border) text-(--moss-secondary-foreground) rounded-sm border p-6 text-center text-sm">
                    <p>No accounts connected yet</p>
                  </div>
                ) : (
                  <div className="border-(--moss-border) overflow-hidden rounded-md border">
                    {profile.accounts.map((account: AccountInfo, index: number) => (
                      <AccountRow key={account.id} account={account} isLast={index === profile.accounts.length - 1} />
                    ))}
                  </div>
                )}
              </div>
            </section>
          </div>
        </PageWrapper>
      </PageView>

      <NewAccountModal showModal={isNewAccountModalOpen} closeModal={closeNewAccountModal} />
    </>
  );
};

const AccountRow = ({ account, isLast }: { account: AccountInfo; isLast: boolean }) => {
  const { isFetching: isFetchingDescribeApp } = useDescribeApp();
  const { mutateAsync: updateProfile } = useUpdateProfile();

  const { openModal: openEditModal, closeModal: closeEditModal, showModal: isEditModalOpen } = useModal();
  const { openModal: openRevokeModal, closeModal: closeRevokeModal, showModal: isRevokeModalOpen } = useModal();

  const handleRemoveAccount = async () => {
    try {
      const input: UpdateProfileInput = {
        accountsToAdd: [],
        accountsToRemove: [account.id],
        accountsToUpdate: [],
      };

      await updateProfile(input);

      closeRevokeModal();
    } catch (error) {
      console.error("Error removing account:", error);
      alert(`Failed to remove account: ${error}`);
    }
  };

  return (
    <>
      <div
        className={cn("flex items-center justify-between px-3 py-2.5", {
          "border-(--moss-border) border-b": !isLast,
        })}
      >
        <div className="flex flex-col gap-1.5">
          <div className="flex items-center gap-2">
            {getProviderIcon(account.kind)}
            <span className="text-sm">{account.username}</span>
            {/*TODO: Background color should use a named theme variable when we decide how to name this component*/}
            <span className="background-(--moss-gray-12) leading-3.5 rounded-full px-[5px] text-xs">
              {account.method === "PAT" ? "PAT" : "OAuth"}
            </span>
          </div>

          {account.metadata.patExpiresAt && (
            //TODO: Text should use a named theme variable when we decide how to name this component
            <span className="text-(--moss-yellow-1) text-sm">Expires on {account.metadata.patExpiresAt}</span>
          )}
        </div>

        <div className="flex items-center gap-3">
          {account.method === "PAT" && (
            <Button
              intent="outlined"
              onClick={openEditModal}
              className="background-(--moss-secondary-background) border-(--moss-border) px-3"
            >
              Edit details
            </Button>
          )}
          <Button intent="danger" onClick={openRevokeModal} disabled={isFetchingDescribeApp}>
            Revoke
          </Button>
        </div>
      </div>

      {/*  Modals */}

      {isEditModalOpen && (
        <EditAccountModal showModal={isEditModalOpen} closeModal={closeEditModal} account={account} />
      )}

      {isRevokeModalOpen && (
        <ConfirmationModal
          showModal={isRevokeModalOpen}
          closeModal={closeRevokeModal}
          title="Revoke Account"
          message={`Are you sure you want to remove this account?`}
          description={`This will revoke access for ${account.username} (${account.kind}).`}
          confirmLabel="Revoke"
          cancelLabel="Cancel"
          onConfirm={handleRemoveAccount}
          variant="danger"
          loading={isFetchingDescribeApp}
        />
      )}
    </>
  );
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
