/** @type {import('tailwindcss').Config} */
const defaultTheme = require("tailwindcss/defaultTheme");
module.exports = {
  mode: 'jit',
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      screens: {
        ...defaultTheme.screens,
      },
    },
  },
  plugins: [],
};
