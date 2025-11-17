import type { SettingScope } from "@repo/settings-storage";

export const SettingScopeEnum = {
  USER: "USER" as const satisfies SettingScope,
  Workspace: (id: string) => ({ WORKSPACE: id }) as const satisfies SettingScope,
} as const;

const _ensureUserForSettingScope: "USER" extends SettingScope ? typeof SettingScopeEnum.USER : never =
  SettingScopeEnum.USER;
const _ensureWorkspaceForSettingScope: { WORKSPACE: string } extends SettingScope
  ? ReturnType<typeof SettingScopeEnum.Workspace>
  : never = SettingScopeEnum.Workspace("test");
void _ensureUserForSettingScope;
void _ensureWorkspaceForSettingScope;
