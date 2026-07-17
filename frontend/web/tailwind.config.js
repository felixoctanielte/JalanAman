/** @type {import('tailwindcss').Config} */
module.exports = {
  // Aktifkan mode dark mode berbasis selector class manual ("dark")
  darkMode: "class",
  content: [
    "./index.html",
    "./src/**/*.rs",
    "../shared/src/**/*.rs",
  ],
  theme: { 
    extend: {}, 
  },
  plugins: [],
};