/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    "../app/src/**/*.{rs}",
    "../frontend/src/**/*.{rs}"
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
