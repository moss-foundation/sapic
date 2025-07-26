export interface ParsedUrl {
  url: {
    raw: string;
    port: number | null;
    host: string[];
    path_params: Array<{
      key: string;
      value: string;
    }>;
    query_params: Array<{
      key: string;
      value: string;
    }>;
  };
}

export interface UrlParameter {
  key: string;
  value: string;
  type: string;
  description: string;
}

export const parseUrl = (rawUrl: string): ParsedUrl => {
  const result: ParsedUrl = {
    url: {
      raw: rawUrl,
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

    // Extract path parameters (including partial ones during typing)
    if (pathPart) {
      const pathParts = pathPart.split("/");
      pathParts.forEach((part) => {
        if (part.startsWith(":")) {
          const key = part.substring(1) || "param";
          result.url.path_params.push({
            key,
            value: key ? `{{${key}}}` : "",
          });
        }
      });
    }

    // Parse query parameters (handles partial parameters during typing)
    if (queryPart !== undefined) {
      if (queryPart === "") {
        result.url.query_params.push({ key: "", value: "" });
      } else {
        const paramPairs = queryPart.split("&");

        paramPairs.forEach((pair) => {
          if (pair === "") {
            result.url.query_params.push({ key: "", value: "" });
          } else if (pair.includes("=")) {
            const equalIndex = pair.indexOf("=");
            const key = pair.substring(0, equalIndex);
            const value = pair.substring(equalIndex + 1);
            result.url.query_params.push({
              key: key || "",
              value: value || "",
            });
          } else {
            result.url.query_params.push({
              key: pair,
              value: "",
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
  if (!value || value.trim() === "") {
    return "string";
  }

  // Skip type detection for template variables
  if (value.includes("{{") && value.includes("}}")) {
    return "string";
  }

  const trimmedValue = value.trim().toLowerCase();

  // Check for boolean values
  if (trimmedValue === "true" || trimmedValue === "false") {
    return "bool";
  }

  // Check for numbers (integer or decimal)
  if (/^-?\d+(\.\d+)?$/.test(trimmedValue)) {
    return "number";
  }

  // Default to string
  return "string";
};

export const getParameterSuggestions = (key: string): { type: string; description: string } => {
  const suggestions: Record<string, { type: string; description: string }> = {
    id: { type: "string", description: "Unique identifier" },
    page: { type: "number", description: "Page number for pagination" },
    limit: { type: "number", description: "Number of items per page" },
    offset: { type: "number", description: "Number of items to skip" },
    sort: { type: "string", description: "Sort field name" },
    order: { type: "string", description: "Sort order (asc/desc)" },
    filter: { type: "string", description: "Filter criteria" },
    search: { type: "string", description: "Search query" },
    tab: { type: "string", description: "Tab selection" },
    status: { type: "string", description: "Status filter" },
    type: { type: "string", description: "Type filter" },
  };

  return suggestions[key.toLowerCase()] || { type: "string", description: `Parameter: ${key}` };
};
