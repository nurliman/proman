/** @type {import('prettier').Options} */
module.exports = {
  arrowParens: "always",
  bracketSpacing: true,
  overrides: [{ files: "*.svelte", options: { parser: "svelte" } }],
  printWidth: 100,
  plugins: [require("prettier-plugin-svelte"), require("prettier-plugin-tailwindcss")],
  quoteProps: "as-needed",
  semi: true,
  tabWidth: 2,
  trailingComma: "all",
  useTabs: false,
};
