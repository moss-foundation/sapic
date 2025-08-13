import { useState } from "react";
import { invokeTauriIpc } from "@/lib/backend/tauri.ts";
import { UserInfo } from "@repo/moss-git-hosting-provider";

export const GitProviders = () => {
  const [currentGitHubAccount, setCurrentGitHubAccount] = useState<UserInfo>();
  const [currentGitLabAccount, setCurrentGitLabAccount] = useState<UserInfo>();
  async function handleGitHubLogin() {
    const result = await invokeTauriIpc<UserInfo>("log_in_with_github");
    if (result.status === "error") {
      alert(`failed to log in with GitHub ${result.error}`);
    } else {
      setCurrentGitHubAccount(result.data);
    }
  }
  async function handleGitLabLogin() {
    const result = await invokeTauriIpc<UserInfo>("log_in_with_gitlab");
    if (result.status === "error") {
      alert(`failed to log in with GitLab ${result.error}`);
    } else {
      setCurrentGitLabAccount(result.data);
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
