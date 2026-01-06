import { OnboardingPage } from "@/pages/onboarding";
import NotFoundPage from "@/welcome/pages/NotFoundPage";
import { createHashHistory, createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

const rootRoute = createRootRoute({ notFoundComponent: NotFoundPage });
const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: OnboardingPage,
});

export const onboardingRouter = createRouter({
  routeTree: rootRoute.addChildren([indexRoute]),
  history: createHashHistory(),
});
