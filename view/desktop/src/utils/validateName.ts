export const validateName = (
  name: string,
  restrictedNames: string[]
): {
  isValid: boolean;
  message: string;
} => {
  if (!name) {
    return {
      isValid: false,
      message: "The name cannot be empty",
    };
  }

  const lowerCaseName = name.toLowerCase();
  const lowerCaseRestrictedNames = restrictedNames.map((name) => name.toLowerCase());

  if (lowerCaseRestrictedNames.includes(lowerCaseName)) {
    return {
      isValid: false,
      message: `The "${name}" is already exists here`,
    };
  }

  return {
    isValid: true,
    message: "",
  };
};
