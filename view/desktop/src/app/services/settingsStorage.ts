import { settingsStorageIpc } from "@/infra/ipc/settingsStorage";
import { SettingScopeEnum } from "@/shared/settingsStorage/types";
import { JsonValue } from "@repo/moss-bindingutils";

type TypedValue<T> = T extends object | boolean | number | string | null ? T : never;

async function batchGetValue<T extends Record<string, object | boolean | number | string | null>>(
  keys: Array<keyof T>
): Promise<{ [K in keyof T]: T[K] | null }>;

async function batchGetValue<T extends Record<string, object | boolean | number | string | null>>(
  keys: Array<keyof T>,
  defaults: { [K in keyof T]: T[K] }
): Promise<{ [K in keyof T]: T[K] }>;

async function batchGetValue<T extends Record<string, object | boolean | number | string | null>>(
  keys: Array<keyof T>,
  defaults?: Partial<{ [K in keyof T]: T[K] }> | { [K in keyof T]: T[K] }
): Promise<{ [K in keyof T]: T[K] | null } | { [K in keyof T]: T[K] }> {
  const output = await settingsStorageIpc.batchGetValue(keys as string[], SettingScopeEnum.USER);
  const hasAllDefaults = defaults && keys.every((key) => key in defaults);

  if (hasAllDefaults) {
    const result = {} as { [K in keyof T]: T[K] };
    for (const key of keys) {
      const value = (output.values[key as string] as T[typeof key]) ?? null;
      result[key] = (value ?? (defaults as { [K in keyof T]: T[K] })[key]) as T[typeof key];
    }
    return result;
  } else {
    const result = {} as { [K in keyof T]: T[K] | null };
    for (const key of keys) {
      const value = (output.values[key as string] as T[typeof key]) ?? null;
      const defaultValue = defaults?.[key];
      result[key] = (value ?? defaultValue) as T[typeof key] | null;
    }
    return result;
  }
}

export const settingsStorageService = {
  getValue: async <T extends object | boolean | number | string | null>(key: string): Promise<T | null> => {
    const output = await settingsStorageIpc.getValue(key, SettingScopeEnum.USER);
    return (output.value as T) ?? null;
  },
  /**
   * Batch get values from settings storage
   * @param keys - The keys to get values for
   * @param defaults - The default values to use if the keys are not found
   * @returns The values for the keys
   *
   * @example
   * const values = await settingsStorageService.batchGetValue(["language", "colorTheme"], { language: "en", colorTheme: "default" });
   *
   * @example
   * const values = await settingsStorageService.batchGetValue(["language", "colorTheme"]);
   *
   * @example
   * const values = await settingsStorageService.batchGetValue(["language", "colorTheme"], { language: "en" });
   */
  batchGetValue,
  updateValue: async <T extends object | boolean | number | string | null>(key: string, value: TypedValue<T>) => {
    return await settingsStorageIpc.updateValue(key, value as JsonValue, SettingScopeEnum.USER);
  },
  removeValue: async (key: string) => {
    return await settingsStorageIpc.removeValue(key, SettingScopeEnum.USER);
  },
};
