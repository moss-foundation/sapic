import { ITokenizerIpc } from "@/shared/tokenizer/ipc";

import { invokeTauriServiceIpc } from "./tauri";

export const tokenizerIpc: ITokenizerIpc = {
  getTokens: async (url: string) => {
    return await invokeTauriServiceIpc("plugin:template-parser|parse_url", {
      input: { url },
    });
  },
};
