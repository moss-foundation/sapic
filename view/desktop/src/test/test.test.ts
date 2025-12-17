import { describe, expect, it } from "vitest";

//TODO this should be removed when we would have actual tests in the desktop
// for now it's just a placeholder to ensure the CI pipeline is not failing
describe("Simple test suite", () => {
  it("should pass a basic assertion", () => {
    expect(1 + 1).toBe(2);
  });
});
