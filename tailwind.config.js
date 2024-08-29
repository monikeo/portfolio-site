/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  darkMode: 'selector',
  theme: {
    extend: {
        backgroundImage: {
        'gradient-radial':
          'radial-gradient(30% 50% at center 50%, var(--tw-gradient-stops))'
        },
    },
    fontFamily: {
      'mono': ['ui-monospace', 'SFMono-Regular'],
    },
  },
  plugins: [
    require('daisyui'),
  ],
  daisyui: {
    themes: [
         {
        mydark: {
            "primary": "#2ac670",
            "secondary": "#e65f3f",
            "accent": "#ffffff",
            "neutral": "#3fa1f2",
            "base-100": "#000000",
            "info": "#2563eb",
            "success": "#4ade80",
            "warning": "#fbbf24",
            "error": "#ef4444",
            "display": "#ffffff",

            "--rounded-box": "1rem",
            "--rounded-btn": "0.4rem"
          },
        myday: {
          "primary": "#009c66",
          "secondary": "#d9534f",
          "accent": "#000000",
          "neutral": "#004080",
          "base-100": "#f8f9fa",
          "info": "#007bff",
          "success": "#28a745",
          "warning": "#ffc107",
          "error": "#dc3545",
          "display": "#000000",

          "--rounded-box": "1rem",
          "--rounded-btn": "0.4rem"
        }
        },
        "fantasy"
    ], // false: only light + dark | true: all themes | array: specific themes like this ["light", "dark", "cupcake"]
    darkTheme: "mydark", // name of one of the included themes for dark mode
    base: true, // applies background color and foreground color for root element by default
    styled: true, // include daisyUI colors and design decisions for all components
    utils: true, // adds responsive and modifier utility classes
    prefix: "", // prefix for daisyUI classnames (components, modifiers and responsive class names. Not colors)
    logs: true, // Shows info about daisyUI version and used config in the console when building your CSS
    themeRoot: ":root", // The element that receives theme color CSS variables
  },
  darkMode: ['class', '[data-theme="mydark"]', '[data-mode="mydark"]']
}
