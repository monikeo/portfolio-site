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
  daisyui: {
    themes: [
         {
        mytheme: {
            "primary": "#2ac670",
            "secondary": "#e65f3f",
            "accent": "#5b5c63",
            "neutral": "#3fa1f2",
            "base-100": "#000000",
            "info": "#2563eb",
            "success": "#4ade80",
            "warning": "#fbbf24",
            "error": "#ef4444",
            "display": "#ffffff",
          },
        },
        "dark",
        "fantasy"
    ], // false: only light + dark | true: all themes | array: specific themes like this ["light", "dark", "cupcake"]
    darkTheme: "mytheme", // name of one of the included themes for dark mode
    base: true, // applies background color and foreground color for root element by default
    styled: true, // include daisyUI colors and design decisions for all components
    utils: true, // adds responsive and modifier utility classes
    prefix: "", // prefix for daisyUI classnames (components, modifiers and responsive class names. Not colors)
    logs: true, // Shows info about daisyUI version and used config in the console when building your CSS
    themeRoot: ":root", // The element that receives theme color CSS variables
  },
  darkMode: ['class', '[data-theme="mytheme"]']
}
