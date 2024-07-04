/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",       // Fichiers Rust dans le frontend
    "./static/**/*.html",  // Fichiers HTML dans le frontend
    "./static/**/*.css",   // Fichiers CSS dans le frontend
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
