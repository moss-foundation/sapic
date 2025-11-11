// Create a new router instance
import App from "@/app";
import NotFound from "@/app/NotFound";
import { createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

const rootRoute = createRootRoute({
  notFoundComponent: NotFound,
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: App,
});

const welcomeRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/welcome.html",
  component: App,
});

const workspaceRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/workspace.html",
  component: App,
});

const workspaceIdRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/workspace.html/$workspaceId",
  component: App,
});

export const router = createRouter({
  routeTree: rootRoute.addChildren([indexRoute, welcomeRoute, workspaceRoute, workspaceIdRoute]),
});

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
