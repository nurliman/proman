/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: "class",
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],

  /** @type {import('./types/daisyui').DaisyUiConfig} */
  daisyui: {},
};
