import { useState } from "react";

import { invokeTauriIpc } from "@/lib/backend/tauri.ts";
import { AddAccountInput, AddAccountOutput } from "@repo/moss-app";
import { UserInfo } from "@repo/moss-git-hosting-provider";

export const GitProviders = () => {
  const [currentGitHubAccount, setCurrentGitHubAccount] = useState<UserInfo>();
  const [currentGitLabAccount, setCurrentGitLabAccount] = useState<UserInfo>();

  async function handleGitHubLogin() {
    const input: AddAccountInput = {
      gitProviderType: "GitHub",
    };

    const result = await invokeTauriIpc<AddAccountOutput>("add_account", {
      input: input,
    });
    if (result.status === "error") {
      alert(`failed to log in with GitHub ${result.error}`);
    } else {
      setCurrentGitHubAccount(result.data.userInfo);
    }
  }
  async function handleGitLabLogin() {
    const input: AddAccountInput = {
      gitProviderType: "GitLab",
    };

    const result = await invokeTauriIpc<AddAccountOutput>("add_account", {
      input: input,
    });
    if (result.status === "error") {
      alert(`failed to log in with GitLab ${result.error}`);
    } else {
      setCurrentGitLabAccount(result.data.userInfo);
    }
  }

  return (
    <div>
      <p>Current GitHub Account:</p>
      <p>name: {currentGitHubAccount?.name}</p>
      <p>email: {currentGitHubAccount?.email}</p>
      <button
        className="cursor-pointer rounded bg-green-500 p-2 text-white hover:bg-green-600"
        onClick={handleGitHubLogin}
      >
        Log in with GitHub
      </button>
      <p>Current GitLab Account:</p>
      <p>name: {currentGitLabAccount?.name}</p>
      <p>email: {currentGitLabAccount?.email}</p>
      <button
        className="cursor-pointer rounded bg-green-500 p-2 text-white hover:bg-green-600"
        onClick={handleGitLabLogin}
      >
        Log in with GitLab
      </button>
    </div>
  );
};
