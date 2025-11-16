import { JsonValue } from "@repo/moss-bindingutils";
import {
  BatchGetValueOutput,
  GetValueOutput,
  RemoveValueOutput,
  SettingScope,
  UpdateValueOutput,
} from "@repo/settings-storage";

export interface ISettingsStorageIpc {
  getValue: (key: string, scope: SettingScope) => Promise<GetValueOutput>;
  batchGetValue: (keys: string[], scope: SettingScope) => Promise<BatchGetValueOutput>;
  updateValue: (key: string, value: JsonValue, scope: SettingScope) => Promise<UpdateValueOutput>;
  removeValue: (key: string, scope: SettingScope) => Promise<RemoveValueOutput>;
}
