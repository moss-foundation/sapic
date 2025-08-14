export const validateValue = (
  value: string,
  restrictedValues: string[]
): {
  isValid: boolean;
  message: string;
} => {
  if (!value) {
    return {
      isValid: false,
      message: "This field cannot be empty",
    };
  }

  const lowerCaseValue = value.toLowerCase();
  const lowerCaseRestrictedValues = restrictedValues.map((value) => value.toLowerCase());

  if (lowerCaseRestrictedValues.includes(lowerCaseValue)) {
    return {
      isValid: false,
      message: `The "${value}" is already exists here`,
    };
  }

  return {
    isValid: true,
    message: "",
  };
};
