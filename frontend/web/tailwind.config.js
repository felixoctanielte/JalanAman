/** @type {import('tailwindcss').Config} */
module.exports = {
  // Aktifkan mode dark mode berbasis selector class manual ("dark")
  darkMode: 'class', 
  content: [
    "./src/**/*.rs",
    "./index.html"
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}