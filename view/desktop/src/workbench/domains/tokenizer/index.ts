import { tokenizerIpc } from "@/infra/ipc/tokenizer";
import { ParsedUrl } from "@/workbench/views/EndpointView/utils";

export interface ITokenizerService {
  getTokens: (url: string) => Promise<ParsedUrl>;
}

export const tokenizerService: ITokenizerService = {
  getTokens: async (url: string) => {
    return await tokenizerIpc.getTokens(url);
  },
};
