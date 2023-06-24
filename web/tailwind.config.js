/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    './src/**/*.rs',
    "./src/**/*.html",
    "./src/**/*.css",
    "./index.html",
  ],
  theme: {
    fontFamily: {
      'op': "Pixel Operator Mono"
    },
    extend: {},
  },
  plugins: [],
}