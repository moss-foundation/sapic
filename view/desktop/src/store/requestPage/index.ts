import { create } from "zustand";
import { parseUrl, reconstructUrl } from "@/pages/RequestPage/utils/urlParser";

export interface UrlParameter {
  key: string;
  value: string;
  disabled?: boolean;
}

export interface RequestPageUrl {
  raw: string;
  originalPathTemplate: string;
  port: number | null;
  host: string[];
  path_params: UrlParameter[];
  query_params: UrlParameter[];
}

export interface RequestPageData {
  url: RequestPageUrl;
}

interface RequestPageStore {
  // State
  requestData: RequestPageData;
  httpMethod: string;

  // Actions
  setUrl: (rawUrl: string) => void;
  setHttpMethod: (method: string) => void;
  updateRequestData: (data: RequestPageData) => void;
  updatePathParams: (pathParams: UrlParameter[]) => void;
  updateQueryParams: (queryParams: UrlParameter[]) => void;
  updatePathParam: (index: number, param: UrlParameter) => void;
  updateQueryParam: (index: number, param: UrlParameter) => void;
  addPathParam: (param: UrlParameter) => void;
  addQueryParam: (param: UrlParameter) => void;
  removePathParam: (index: number) => void;
  removeQueryParam: (index: number) => void;
  reconstructUrlFromParams: () => void;
}

export const useRequestPageStore = create<RequestPageStore>((set, get) => ({
  requestData: {
    url: {
      raw: "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns?sort={{sortValue}}&limit=2",
      originalPathTemplate: "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns",
      port: null,
      host: [],
      path_params: [
        { key: "docId", value: "" },
        { key: "tableIdOrName", value: "" },
      ],
      query_params: [
        { key: "sort", value: "{{sortValue}}" },
        { key: "limit", value: "2" },
      ],
    },
  },
  httpMethod: "POST",

  setUrl: (rawUrl: string) => {
    const parsedUrl = parseUrl(rawUrl);
    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          raw: rawUrl,
          originalPathTemplate: parsedUrl.url.originalPathTemplate,
          path_params: parsedUrl.url.path_params,
          query_params: parsedUrl.url.query_params,
        },
      },
    }));
  },

  setHttpMethod: (method: string) => {
    set({ httpMethod: method });
  },

  updateRequestData: (data: RequestPageData) => {
    set({ requestData: data });
  },

  updatePathParams: (pathParams: UrlParameter[]) => {
    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          path_params: pathParams,
        },
      },
    }));
  },

  updateQueryParams: (queryParams: UrlParameter[]) => {
    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          query_params: queryParams,
        },
      },
    }));
  },

  updatePathParam: (index: number, param: UrlParameter) => {
    const currentState = get();
    const newPathParams = [...currentState.requestData.url.path_params];
    newPathParams[index] = param;

    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          path_params: newPathParams,
        },
      },
    }));
  },

  updateQueryParam: (index: number, param: UrlParameter) => {
    const currentState = get();
    const newQueryParams = [...currentState.requestData.url.query_params];
    newQueryParams[index] = param;

    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          query_params: newQueryParams,
        },
      },
    }));
  },

  addPathParam: (param: UrlParameter) => {
    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          path_params: [...state.requestData.url.path_params, param],
        },
      },
    }));
  },

  addQueryParam: (param: UrlParameter) => {
    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          query_params: [...state.requestData.url.query_params, param],
        },
      },
    }));
  },

  removePathParam: (index: number) => {
    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          path_params: state.requestData.url.path_params.filter((_, i) => i !== index),
        },
      },
    }));
  },

  removeQueryParam: (index: number) => {
    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          query_params: state.requestData.url.query_params.filter((_, i) => i !== index),
        },
      },
    }));
  },

  reconstructUrlFromParams: () => {
    const currentState = get();
    const { path_params, query_params, originalPathTemplate } = currentState.requestData.url;

    let reconstructedPath = originalPathTemplate || currentState.requestData.url.raw.split("?")[0];

    const enabledPathParams = path_params.filter((param) => !param.disabled);
    const currentParamKeys = new Set(enabledPathParams.map((param) => param.key));

    const pathSegments = reconstructedPath.split("/");
    const filteredSegments = pathSegments.filter((segment) => {
      if (segment.startsWith(":")) {
        const paramKey = segment.substring(1);
        return currentParamKeys.has(paramKey);
      }
      return true;
    });

    reconstructedPath = filteredSegments.join("/");

    enabledPathParams.forEach((param) => {
      if (param.key && param.key.trim() !== "") {
        const paramPattern = new RegExp(`:${param.key}\\b`, "g");
        if (param.value && param.value.trim() !== "") {
          reconstructedPath = reconstructedPath.replace(paramPattern, param.value);
        }
      }
    });

    const enabledQueryParams = query_params.filter((param) => !param.disabled);

    const newUrl = reconstructUrl(reconstructedPath, [], enabledQueryParams);

    if (newUrl !== currentState.requestData.url.raw) {
      set((state) => ({
        requestData: {
          ...state.requestData,
          url: {
            ...state.requestData.url,
            raw: newUrl,
          },
        },
      }));
    }
  },
}));
