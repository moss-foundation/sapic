import { BackendModule, ReadCallback } from "i18next";

import { GetTranslationsInput, GetTranslationsOutput } from "@repo/moss-nls";

import { invokeTauriIpc, IpcResult } from "./tauri";

interface I18nDictionary {
  [key: string]: string;
}

const getTranslationsFn = async (input: GetTranslationsInput): Promise<IpcResult<GetTranslationsOutput, string>> => {
  return await invokeTauriIpc<GetTranslationsOutput, string>("get_translations", {
    input: input,
  });
};

const I18nTauriBackend: BackendModule = {
  type: "backend",
  init: () => {},
  read: async (language: string, namespace: string, callback: ReadCallback) => {
    const result = await getTranslationsFn({ language, namespace });
    if (result.status === "ok") {
      callback(null, result.data as I18nDictionary);
    } else {
      callback(result.error, false);
    }
  },
};

export default I18nTauriBackend;
