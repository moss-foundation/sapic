import { ITokenizerIpc } from "@/shared/tokenizer/ipc";
import { invoke } from "@tauri-apps/api/core";

export const tokenizerIpc: ITokenizerIpc = {
  getTokens: async (url: string) => {
    return await invoke("plugin:url-parser|parse_url", {
      input: { url },
    });
  },
};
