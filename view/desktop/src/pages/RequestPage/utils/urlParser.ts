import { parameterSuggestions, type ParameterSuggestion } from "./parameterSuggestions";

export interface ParsedUrl {
  url: {
    raw: string;
    originalPathTemplate: string;
    port: number | null;
    host: string[];
    path_params: Array<{
      key: string;
      value: string;
      disabled?: boolean;
    }>;
    query_params: Array<{
      key: string;
      value: string;
      disabled?: boolean;
    }>;
  };
}

export interface UrlParameter {
  key: string;
  value: string;
  type: string;
  description: string;
  disabled?: boolean;
}

export const parseUrl = (rawUrl: string): ParsedUrl => {
  const result: ParsedUrl = {
    url: {
      raw: rawUrl,
      originalPathTemplate: "",
      port: null,
      host: [],
      path_params: [],
      query_params: [],
    },
  };

  if (!rawUrl || rawUrl.trim() === "") {
    return result;
  }

  try {
    const [pathPart, queryPart] = rawUrl.split("?");

    result.url.originalPathTemplate = pathPart || "";

    if (pathPart) {
      const pathParts = pathPart.split("/");
      pathParts.forEach((part) => {
        if (part.startsWith(":")) {
          const key = part.substring(1) || "param";
          result.url.path_params.push({
            key,
            value: "",
            disabled: false,
          });
        }
      });
    }

    if (queryPart !== undefined) {
      if (queryPart === "") {
        result.url.query_params.push({ key: "", value: "", disabled: false });
      } else {
        const paramPairs = queryPart.split("&");

        paramPairs.forEach((pair) => {
          if (pair === "") {
            result.url.query_params.push({ key: "", value: "", disabled: false });
          } else if (pair.includes("=")) {
            const equalIndex = pair.indexOf("=");
            const key = pair.substring(0, equalIndex);
            const value = pair.substring(equalIndex + 1);
            result.url.query_params.push({
              key: key || "",
              value: value || "",
              disabled: false,
            });
          } else {
            result.url.query_params.push({
              key: pair,
              value: "",
              disabled: false,
            });
          }
        });
      }
    }

    try {
      const tempUrl = rawUrl.replace(/\{\{[^}]+\}\}/g, "example.com").replace(/^([^:]+):\/\//, "http://");

      let urlToParse = tempUrl;
      if (!urlToParse.match(/^https?:\/\//)) {
        urlToParse = "http://" + urlToParse;
      }

      const url = new URL(urlToParse);
      result.url.host = url.hostname.split(".").filter((part) => part.length > 0);

      if (url.port) {
        result.url.port = parseInt(url.port, 10);
      }
    } catch (hostError) {
      // Ignore host parsing errors
    }
  } catch (error) {
    console.warn("URL parsing failed:", error);
  }

  return result;
};

export const convertToTableFormat = (
  params: Array<{ key: string; value: string }>,
  type: "path" | "query"
): UrlParameter[] => {
  return params.map((param) => ({
    key: param.key,
    value: param.value,
    type: type === "path" ? "string" : "string",
    description: `${type === "path" ? "Path" : "Query"} parameter: ${param.key}`,
  }));
};

export const detectValueType = (value: string): string => {
  if (!value || value.trim() === "") return "string";

  // Keep template variables as string
  if (value.includes("{{") && value.includes("}}")) return "string";

  if (value.toLowerCase() === "true" || value.toLowerCase() === "false") return "bool";

  if (!isNaN(Number(value)) && !isNaN(parseFloat(value))) return "number";

  return "string";
};

export const reconstructUrl = (
  baseUrl: string,
  pathParams: Array<{ key: string; value: string }>,
  queryParams: Array<{ key: string; value: string }>
): string => {
  try {
    let reconstructedUrl = baseUrl;

    pathParams.forEach((param) => {
      if (param.key && param.key.trim() !== "") {
        const pathParamPattern = new RegExp(`:${param.key}\\b`, "g");
        reconstructedUrl = reconstructedUrl.replace(pathParamPattern, param.value || param.key);
      }
    });

    const validQueryParams = queryParams.filter((param) => param.key && param.key.trim() !== "");

    if (validQueryParams.length > 0) {
      const [urlBase] = reconstructedUrl.split("?");
      reconstructedUrl = urlBase;

      const queryString = validQueryParams
        .map((param) => {
          const key = encodeURIComponent(param.key);
          // Don't encode template variables
          const isTemplateVariable =
            param.value && ((param.value.includes("{{") && param.value.includes("}}")) || param.value.startsWith(":"));
          const value = param.value ? (isTemplateVariable ? param.value : encodeURIComponent(param.value)) : "";
          return value ? `${key}=${value}` : key;
        })
        .join("&");

      if (queryString) {
        reconstructedUrl += `?${queryString}`;
      }
    } else {
      const [urlBase] = reconstructedUrl.split("?");
      reconstructedUrl = urlBase;
    }

    return reconstructedUrl;
  } catch (error) {
    console.warn("URL reconstruction failed:", error);
    return baseUrl; // Fallback to original URL
  }
};

export const getParameterSuggestions = (key: string): ParameterSuggestion => {
  return parameterSuggestions[key.toLowerCase()] || { type: "string", description: `Parameter: ${key}` };
};
