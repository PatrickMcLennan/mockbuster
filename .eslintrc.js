module.exports = {
  env: {
    es6: true,
    browser: true,
  },
  globals: {
    SUBSCRIPTION_PUBLIC_KEY: "readonly",
  },
  extends: ["plugin:@typescript-eslint/recommended", "prettier"],
  plugins: ["@typescript-eslint", "prettier"],
  rules: {
    "prettier/prettier": ["error"],
    "@typescript-eslint/no-unused-vars": "error",
    "@typescript-eslint/no-var-requires": "off",
  },
};
