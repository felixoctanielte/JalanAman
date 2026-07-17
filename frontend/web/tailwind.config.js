/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: [
    "./index.html",
    "./src/**/*.rs",
    "../shared/src/**/*.rs",
  ],
  theme: { extend: {} },
  plugins: [],
};
