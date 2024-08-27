/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
    extend: {
        backgroundImage: {
        'gradient-radial':
          'radial-gradient(30% 50% at center 50%, var(--tw-gradient-stops))'
        },
    },
  },
  plugins: [
    require('daisyui'),
  ],
}
