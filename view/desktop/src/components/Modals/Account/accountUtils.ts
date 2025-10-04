import { AccountKind } from "@repo/moss-user";

export const getProviderName = (kind: AccountKind): string => {
  return kind === "GITHUB" ? "GitHub" : "GitLab";
};

export const getProviderSettingsUrl = (kind: AccountKind): string => {
  return kind === "GITHUB"
    ? "https://github.com/settings/tokens"
    : "https://gitlab.com/-/user_settings/personal_access_tokens";
};

export const getProviderHost = (kind: AccountKind): string => {
  return kind === "GITHUB" ? "github.com" : "gitlab.com";
};

export const getPatPlaceholder = (kind: AccountKind): string => {
  const prefix = kind === "GITHUB" ? "github" : "gitlab";
  return `${prefix}_pat_11AJP6K3A0nS9zI77AkyOB_uLU0OUSZu0TRUGo9czDrXzur3kMGpusg9XJpzYaeYYEAKALQUTZ0L3v6q9i`;
};
