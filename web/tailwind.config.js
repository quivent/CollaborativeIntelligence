module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Extend with any custom colors if needed
      },
    },
  },
  plugins: [],
  safelist: [
    // Include all color variations we use for agents
    {
      pattern: /bg-(slate|blue|indigo|purple|pink|rose|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|violet|fuchsia|gray|stone|zinc)-(50|100|200)/,
    },
    {
      pattern: /border-(slate|blue|indigo|purple|pink|rose|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|violet|fuchsia|gray|stone|zinc)-(200|300)/,
    },
    {
      pattern: /text-(slate|blue|indigo|purple|pink|rose|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|violet|fuchsia|gray|stone|zinc)-(700|800)/,
    },
    {
      pattern: /hover:bg-(slate|blue|indigo|purple|pink|rose|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|violet|fuchsia|gray|stone|zinc)-(100|200)/,
    },
    {
      pattern: /hover:border-(slate|blue|indigo|purple|pink|rose|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|violet|fuchsia|gray|stone|zinc)-(300)/,
    }
  ]
}