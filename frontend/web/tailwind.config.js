/** @type {import('tailwindcss').Config} */
module.exports = {
<<<<<<< HEAD
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
=======
  content: [
    "./index.html",
    "./src/**/*.rs",
    "../shared/src/**/*.rs",
  ],
  theme: { extend: {} },
  plugins: [],
}
>>>>>>> origin/develop-mobile-and-backend
