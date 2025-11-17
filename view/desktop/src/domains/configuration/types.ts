import type { ConfigurationTarget } from "@repo/base";

export enum ConfigurationTargetEnum {
  USER = "USER",
  WORKSPACE = "WORKSPACE",
}

const _ensureAllValuesForConfigurationTarget: {
  [K in ConfigurationTarget]: ConfigurationTargetEnum;
} = {
  USER: ConfigurationTargetEnum.USER,
  WORKSPACE: ConfigurationTargetEnum.WORKSPACE,
};
void _ensureAllValuesForConfigurationTarget;
