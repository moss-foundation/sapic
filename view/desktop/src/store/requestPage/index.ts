import { create } from "zustand";
import { reconstructUrl } from "../../pages/RequestPage/utils/urlParser";

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
  reconstructUrlFromParams: () => void;
}

export const useRequestPageStore = create<RequestPageStore>((set, get) => ({
  // Initial state
  requestData: {
    url: {
      raw: "{{baseUrl}}/docs/:docId/tables/:tableIdOrName/columns?sort={{sortValue}}&limit=2",
      port: null,
      host: [],
      path_params: [
        { key: "docId", value: "{{docId}}" },
        { key: "tableIdOrName", value: "{{tableIdOrName}}" },
      ],
      query_params: [
        { key: "sort", value: "{{sortValue}}" },
        { key: "limit", value: "2" },
      ],
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

  reconstructUrlFromParams: () => {
    const currentState = get();
    const { path_params, query_params } = currentState.requestData.url;

    // Simple approach: start with current URL and reconstruct it
    let baseUrl = currentState.requestData.url.raw;

    // Remove current query string to get the path part
    const [pathPart] = baseUrl.split("?");

    // For path params, we'll build the URL by replacing values or keeping template patterns
    let reconstructedPath = pathPart;

    // Replace each path param
    path_params.forEach((param) => {
      if (param.key && param.value) {
        // Replace :paramKey with the actual value
        const paramPattern = new RegExp(`:${param.key}\\b`, "g");
        reconstructedPath = reconstructedPath.replace(paramPattern, param.value);
      }
    });

    // Reconstruct the URL with current parameter values
    const newUrl = reconstructUrl(reconstructedPath, [], query_params);

    // Only update if the URL actually changed
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

export type { RequestPageData, RequestPageUrl, UrlParameter };
