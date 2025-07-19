export const validateName = (
  name: string,
  restrictedNames: (string | number)[]
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

  if (restrictedNames.includes(name)) {
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
