import type { SettingScope } from "@repo/settings-storage";

export const SettingScopeEnum = {
  USER: "USER" as const satisfies SettingScope,
  workspace: (id: string) => ({ WORKSPACE: id }) as const satisfies SettingScope,
} as const;

const _ensureUserForSettingScope: "USER" extends SettingScope ? typeof SettingScopeEnum.USER : never =
  SettingScopeEnum.USER;
const _ensureWorkspaceForSettingScope: { WORKSPACE: string } extends SettingScope
  ? ReturnType<typeof SettingScopeEnum.workspace>
  : never = SettingScopeEnum.workspace("test");
void _ensureUserForSettingScope;
void _ensureWorkspaceForSettingScope;
