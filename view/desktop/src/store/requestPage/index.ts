import { create } from "zustand";

interface UrlParameter {
  key: string;
  value: string;
}

interface RequestPageUrl {
  raw: string;
  port: number | null;
  host: string[];
  path_params: UrlParameter[];
  query_params: UrlParameter[];
}

interface RequestPageData {
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
}

export const useRequestPageStore = create<RequestPageStore>((set, get) => ({
  // Initial state
  requestData: {
    url: {
      raw: "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns?queryParam={{queryValue}}",
      port: null,
      host: [],
      path_params: [
        { key: "docId", value: "{{docId}}" },
        { key: "tableIdOrName", value: "{{tableIdOrName}}" },
      ],
      query_params: [{ key: "queryParam", value: "{{queryValue}}" }],
    },
  },
  httpMethod: "POST",

  // Actions
  setUrl: (rawUrl: string) => {
    set((state) => ({
      requestData: {
        ...state.requestData,
        url: {
          ...state.requestData.url,
          raw: rawUrl,
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
}));

export type { RequestPageData, RequestPageUrl, UrlParameter };
