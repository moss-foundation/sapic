// Create a new router instance
import App from "@/app";
import NotFound from "@/app/NotFound";
import { createHashHistory, createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

const rootRoute = createRootRoute({ notFoundComponent: NotFound });
const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: App,
});

const workspaceIdRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/$workspaceId",
  component: App,
});

export const router = createRouter({
  routeTree: rootRoute.addChildren([indexRoute, workspaceIdRoute]),
  history: createHashHistory(),
});

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
