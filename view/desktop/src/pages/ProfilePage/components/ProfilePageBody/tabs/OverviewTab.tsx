import { useState } from "react";

import { IDockviewPanelProps } from "@/lib/moss-tabs/src";
import { Button, Icon } from "@/lib/ui";
import { Input } from "@/lib/ui/Input";
import { invoke } from "@tauri-apps/api/core";
import { AddAccountParams, UpdateProfileInput } from "@repo/moss-app";
import { AccountInfo, AccountKind, ProfileInfo } from "@repo/moss-user";

import { ProfilePageProps } from "../../../ProfilePage";

interface OverviewTabProps extends IDockviewPanelProps<ProfilePageProps> {
  profile: ProfileInfo;
}

export const OverviewTab = ({ profile }: OverviewTabProps) => {
  const [isAddingAccount, setIsAddingAccount] = useState(false);
  const [accountForm, setAccountForm] = useState<AddAccountParams>({
    host: "github.com",
    label: "",
    kind: "GITHUB",
  });
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleAddAccount = async () => {
    try {
      setIsSubmitting(true);
      const input: UpdateProfileInput = {
        accountsToAdd: [accountForm],
        accountsToRemove: [],
      };
      await invoke("update_profile", { input });
      console.log("Account added successfully");

      setAccountForm({
        host: "github.com",
        label: "",
        kind: "GITHUB",
      });
      setIsAddingAccount(false);
      window.location.reload();
    } catch (error) {
      console.error("Error adding account:", error);
      alert(`Failed to add account: ${error}`);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleRemoveAccount = async (accountId: string) => {
    if (!confirm("Are you sure you want to remove this account?")) {
      return;
    }

    try {
      setIsSubmitting(true);
      const input: UpdateProfileInput = {
        accountsToAdd: [],
        accountsToRemove: [accountId],
      };
      await invoke("update_profile", { input });
      console.log("Account removed successfully");
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
        return "VCS";
      case "GITLAB":
        return "VCS";
      default:
        return "Placeholder";
    }
  };

  return (
    <div className="flex flex-col gap-6">
      {/* Accounts Section */}
      <section>
        <div className="mb-4 flex items-center justify-between">
          <h2 className="text-[16px] font-normal">Accounts</h2>
          {!isAddingAccount && (
            <Button
              onClick={() => setIsAddingAccount(true)}
              className="background-(--moss-primary) hover:background-(--moss-primary-hover) rounded-sm px-4 py-1.5 text-sm font-medium text-(--moss-primary-text-inverse)"
            >
              Connect
            </Button>
          )}
        </div>

        {/* Add Account Form */}
        {isAddingAccount && (
          <div className="mb-4 rounded-sm border border-(--moss-border-color) p-4">
            <h4 className="mb-4 text-sm font-medium">Add New Account</h4>
            <div className="flex flex-col gap-3">
              <div className="flex flex-col gap-1.5">
                <label className="text-sm text-(--moss-secondary-text)">Host</label>
                <Input
                  type="text"
                  value={accountForm.host}
                  onChange={(e) => setAccountForm((prev) => ({ ...prev, host: e.target.value }))}
                  placeholder="github.com"
                  className="background-(--moss-secondary-background) rounded-sm border border-(--moss-border-color) px-2 py-1.5 text-sm"
                />
              </div>

              <div className="flex flex-col gap-1.5">
                <label className="text-sm text-(--moss-secondary-text)">Label (optional)</label>
                <Input
                  type="text"
                  value={accountForm.label}
                  onChange={(e) => setAccountForm((prev) => ({ ...prev, label: e.target.value }))}
                  placeholder="My GitHub Account"
                  className="background-(--moss-secondary-background) rounded-sm border border-(--moss-border-color) px-2 py-1.5 text-sm"
                />
              </div>

              <div className="flex flex-col gap-1.5">
                <label className="text-sm text-(--moss-secondary-text)">Provider</label>
                <select
                  value={accountForm.kind}
                  onChange={(e) => setAccountForm((prev) => ({ ...prev, kind: e.target.value as AccountKind }))}
                  className="background-(--moss-secondary-background) w-full rounded-sm border border-(--moss-border-color) px-2 py-1.5 text-sm"
                >
                  <option value="GITHUB">GitHub</option>
                  <option value="GITLAB">GitLab</option>
                </select>
              </div>

              <div className="flex gap-2">
                <Button
                  onClick={handleAddAccount}
                  disabled={isSubmitting || !accountForm.host}
                  loading={isSubmitting}
                  className="background-(--moss-primary) hover:background-(--moss-primary-hover) rounded-sm px-4 py-1.5 text-sm text-(--moss-primary-text-inverse)"
                >
                  Add Account
                </Button>
                <Button
                  onClick={() => {
                    setIsAddingAccount(false);
                    setAccountForm({
                      host: "github.com",
                      label: "",
                      kind: "GITHUB",
                    });
                  }}
                  disabled={isSubmitting}
                  className="background-(--moss-secondary-background) hover:background-(--moss-secondary-background-hover) rounded-sm border border-(--moss-border-color) px-4 py-1.5 text-sm"
                >
                  Cancel
                </Button>
              </div>
            </div>
          </div>
        )}

        {/* Accounts List */}
        <div className="flex flex-col gap-2">
          {profile.accounts.length === 0 ? (
            <div className="rounded-sm border border-(--moss-border-color) p-6 text-center text-sm text-(--moss-secondary-text)">
              <p>No accounts connected yet</p>
            </div>
          ) : (
            profile.accounts.map((account: AccountInfo) => (
              <div
                key={account.id}
                className="flex items-center justify-between rounded-sm border border-(--moss-border-color) px-4 py-3"
              >
                <div className="flex items-center gap-3">
                  <Icon icon={getProviderIcon(account.kind)} className="size-4" />
                  <span className="text-sm">{account.username}</span>
                </div>
                <div className="flex items-center gap-2">
                  <Button
                    onClick={() => console.log("Edit details for", account.id)}
                    className="background-(--moss-secondary-background) hover:background-(--moss-secondary-background-hover) rounded-sm border border-(--moss-border-color) px-3 py-1 text-xs"
                  >
                    Edit details
                  </Button>
                  <Button
                    onClick={() => handleRemoveAccount(account.id)}
                    disabled={isSubmitting}
                    className="background-(--moss-error) rounded-sm px-3 py-1 text-xs text-white hover:opacity-90"
                  >
                    Revoke
                  </Button>
                </div>
              </div>
            ))
          )}
        </div>
      </section>
    </div>
  );
};
