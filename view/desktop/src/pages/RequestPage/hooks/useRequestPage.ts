import { useCallback, useState } from "react";

import { areUrlsEquivalent, parseUrl, reconstructUrl } from "@/pages/RequestPage/utils/urlParser";

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

export const useRequestPage = () => {
  const [requestData, setRequestData] = useState<RequestPageData>({
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
  });

  const [httpMethod, setHttpMethod] = useState<string>("POST");

  const setUrl = useCallback((rawUrl: string) => {
    const parsedUrl = parseUrl(rawUrl);
    setRequestData((prev) => ({
      ...prev,
      url: {
        ...prev.url,
        raw: rawUrl,
        originalPathTemplate: parsedUrl.url.originalPathTemplate,
        path_params: parsedUrl.url.path_params,
        query_params: parsedUrl.url.query_params,
      },
    }));
  }, []);

  const updateRequestData = useCallback((data: RequestPageData) => {
    setRequestData(data);
  }, []);

  const updatePathParams = useCallback((pathParams: UrlParameter[]) => {
    setRequestData((prev) => ({
      ...prev,
      url: {
        ...prev.url,
        path_params: pathParams,
      },
    }));
  }, []);

  const updateQueryParams = useCallback((queryParams: UrlParameter[]) => {
    setRequestData((prev) => ({
      ...prev,
      url: {
        ...prev.url,
        query_params: queryParams,
      },
    }));
  }, []);

  const updatePathParam = useCallback((index: number, param: UrlParameter) => {
    setRequestData((prev) => {
      const newPathParams = [...prev.url.path_params];
      newPathParams[index] = param;

      return {
        ...prev,
        url: {
          ...prev.url,
          path_params: newPathParams,
        },
      };
    });
  }, []);

  const updateQueryParam = useCallback((index: number, param: UrlParameter) => {
    setRequestData((prev) => {
      const newQueryParams = [...prev.url.query_params];
      newQueryParams[index] = param;

      return {
        ...prev,
        url: {
          ...prev.url,
          query_params: newQueryParams,
        },
      };
    });
  }, []);

  const addPathParam = useCallback((param: UrlParameter) => {
    setRequestData((prev) => ({
      ...prev,
      url: {
        ...prev.url,
        path_params: [...prev.url.path_params, param],
      },
    }));
  }, []);

  const addQueryParam = useCallback((param: UrlParameter) => {
    setRequestData((prev) => ({
      ...prev,
      url: {
        ...prev.url,
        query_params: [...prev.url.query_params, param],
      },
    }));
  }, []);

  const removePathParam = useCallback((index: number) => {
    setRequestData((prev) => ({
      ...prev,
      url: {
        ...prev.url,
        path_params: prev.url.path_params.filter((_, i) => i !== index),
      },
    }));
  }, []);

  const removeQueryParam = useCallback((index: number) => {
    setRequestData((prev) => ({
      ...prev,
      url: {
        ...prev.url,
        query_params: prev.url.query_params.filter((_, i) => i !== index),
      },
    }));
  }, []);

  const reconstructUrlFromParams = useCallback(() => {
    const { path_params, query_params, originalPathTemplate } = requestData.url;

    let reconstructedPath = originalPathTemplate || requestData.url.raw.split("?")[0];

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

    // Path parameters kept as template variables in URL display

    const enabledQueryParams = query_params.filter((param) => !param.disabled);

    const newUrl = reconstructUrl(reconstructedPath, [], enabledQueryParams);

    // Use normalized comparison to prevent unnecessary updates
    if (!areUrlsEquivalent(newUrl, requestData.url.raw)) {
      setRequestData((prev) => ({
        ...prev,
        url: {
          ...prev.url,
          raw: newUrl,
        },
      }));
    }
  }, [requestData.url]);

  const getRequestUrlWithPathValues = useCallback(() => {
    const { path_params, query_params, originalPathTemplate } = requestData.url;

    let requestPath = originalPathTemplate || requestData.url.raw.split("?")[0];

    // Replace path parameters with actual values for HTTP requests
    const enabledPathParams = path_params.filter((param) => !param.disabled);

    enabledPathParams.forEach((param) => {
      if (param.key && param.key.trim() !== "") {
        const paramPattern = new RegExp(`:${param.key}\\b`, "g");
        if (param.value && param.value.trim() !== "") {
          requestPath = requestPath.replace(paramPattern, param.value);
        }
      }
    });

    const enabledQueryParams = query_params.filter((param) => !param.disabled);

    return reconstructUrl(requestPath, [], enabledQueryParams);
  }, [requestData.url]);

  return {
    requestData,
    httpMethod,
    setUrl,
    setHttpMethod,
    updateRequestData,
    updatePathParams,
    updateQueryParams,
    updatePathParam,
    updateQueryParam,
    addPathParam,
    addQueryParam,
    removePathParam,
    removeQueryParam,
    reconstructUrlFromParams,
    getRequestUrlWithPathValues,
  };
};
