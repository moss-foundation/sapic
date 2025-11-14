export interface ParameterSuggestion {
  type: string;
  description: string;
}

export const parameterSuggestions: Record<string, ParameterSuggestion> = {
  // Authentication & Authorization
  token: { type: "string", description: "Authentication token" },
  apikey: { type: "string", description: "API key for authentication" },
  auth: { type: "string", description: "Authorization header value" },
  bearer: { type: "string", description: "Bearer token" },
  jwt: { type: "string", description: "JSON Web Token" },
  clientid: { type: "string", description: "OAuth client identifier" },
  clientsecret: { type: "string", description: "OAuth client secret" },

  // Pagination
  page: { type: "number", description: "Page number for pagination" },
  limit: { type: "number", description: "Number of items per page" },
  size: { type: "number", description: "Page size" },
  offset: { type: "number", description: "Number of items to skip" },
  skip: { type: "number", description: "Number of records to skip" },
  take: { type: "number", description: "Number of records to take" },
  perpage: { type: "number", description: "Items per page" },
  pagesize: { type: "number", description: "Size of each page" },

  // Sorting & Ordering
  sort: { type: "string", description: "Sort field name" },
  sortby: { type: "string", description: "Field to sort by" },
  order: { type: "string", description: "Sort order (asc/desc)" },
  orderby: { type: "string", description: "Order by field" },
  direction: { type: "string", description: "Sort direction" },

  // Filtering & Search
  filter: { type: "string", description: "Filter criteria" },
  search: { type: "string", description: "Search query" },
  query: { type: "string", description: "Search query string" },
  q: { type: "string", description: "Query parameter" },
  keyword: { type: "string", description: "Search keyword" },
  term: { type: "string", description: "Search term" },
  where: { type: "string", description: "Where condition" },

  // Identifiers
  id: { type: "string", description: "Unique identifier" },
  uuid: { type: "string", description: "Universally unique identifier" },
  userid: { type: "string", description: "User identifier" },
  accountid: { type: "string", description: "Account identifier" },
  organizationid: { type: "string", description: "Organization identifier" },
  projectid: { type: "string", description: "Project identifier" },
  sessionid: { type: "string", description: "Session identifier" },
  endpointid: { type: "string", description: "Endpoint identifier" },
  transactionid: { type: "string", description: "Transaction identifier" },

  // Status & State
  status: { type: "string", description: "Status filter" },
  state: { type: "string", description: "State filter" },
  active: { type: "bool", description: "Filter by active status" },
  enabled: { type: "bool", description: "Filter by enabled status" },
  disabled: { type: "bool", description: "Filter by disabled status" },
  published: { type: "bool", description: "Filter by published status" },
  draft: { type: "bool", description: "Filter by draft status" },

  // Time & Date
  from: { type: "string", description: "Start date/time" },
  to: { type: "string", description: "End date/time" },
  since: { type: "string", description: "Since date/time" },
  until: { type: "string", description: "Until date/time" },
  startdate: { type: "string", description: "Start date" },
  enddate: { type: "string", description: "End date" },
  createdat: { type: "string", description: "Creation timestamp" },
  updatedat: { type: "string", description: "Last update timestamp" },
  timestamp: { type: "string", description: "Timestamp value" },

  // Format & Output
  format: { type: "string", description: "Response format (json, xml, csv)" },
  type: { type: "string", description: "Content type" },
  accept: { type: "string", description: "Accept header value" },
  contenttype: { type: "string", description: "Content-Type header value" },
  encoding: { type: "string", description: "Content encoding" },
  compression: { type: "string", description: "Compression type" },

  // API Versioning
  version: { type: "string", description: "API version" },
  v: { type: "string", description: "Version number" },
  apiversion: { type: "string", description: "API version identifier" },

  // Localization
  locale: { type: "string", description: "Locale identifier" },
  language: { type: "string", description: "Language code" },
  lang: { type: "string", description: "Language preference" },
  country: { type: "string", description: "Country code" },
  timezone: { type: "string", description: "Timezone identifier" },

  // UI & Navigation
  tab: { type: "string", description: "Tab selection" },
  view: { type: "string", description: "View type" },
  mode: { type: "string", description: "Display mode" },
  theme: { type: "string", description: "Theme selection" },
  layout: { type: "string", description: "Layout type" },

  // Configuration
  config: { type: "string", description: "Configuration parameter" },
  setting: { type: "string", description: "Setting value" },
  option: { type: "string", description: "Option parameter" },
  preference: { type: "string", description: "User preference" },
  feature: { type: "string", description: "Feature flag" },

  // Data Operations
  fields: { type: "string", description: "Fields to include/exclude" },
  include: { type: "string", description: "Related data to include" },
  exclude: { type: "string", description: "Data to exclude" },
  expand: { type: "string", description: "Related entities to expand" },
  embed: { type: "string", description: "Embedded resources" },

  // Cache & Performance
  cache: { type: "bool", description: "Enable/disable caching" },
  nocache: { type: "bool", description: "Bypass cache" },
  refresh: { type: "bool", description: "Force refresh" },
  timeout: { type: "number", description: "Endpoint timeout in seconds" },

  // Debug & Testing
  debug: { type: "bool", description: "Enable debug mode" },
  verbose: { type: "bool", description: "Enable verbose output" },
  test: { type: "bool", description: "Test mode flag" },
  mock: { type: "bool", description: "Use mock data" },
  dryrun: { type: "bool", description: "Dry run mode" },

  // Common Resource Names
  name: { type: "string", description: "Resource name" },
  title: { type: "string", description: "Title or heading" },
  description: { type: "string", description: "Description text" },
  email: { type: "string", description: "Email address" },
  username: { type: "string", description: "Username" },
  password: { type: "string", description: "Password" },
  url: { type: "string", description: "URL address" },
  path: { type: "string", description: "File or resource path" },
  filename: { type: "string", description: "File name" },
  category: { type: "string", description: "Category identifier" },
  tag: { type: "string", description: "Tag identifier" },
  label: { type: "string", description: "Label text" },

  // Numeric Values
  count: { type: "number", description: "Count value" },
  total: { type: "number", description: "Total count" },
  amount: { type: "number", description: "Amount value" },
  price: { type: "number", description: "Price value" },
  cost: { type: "number", description: "Cost value" },
  weight: { type: "number", description: "Weight value" },
  length: { type: "number", description: "Length value" },
  width: { type: "number", description: "Width value" },
  height: { type: "number", description: "Height value" },
  filesize: { type: "number", description: "File size in bytes" },
};
