export default {
  content: [
    "./index.html",
    "./src/**/*.rs",
    "./node_modules/tw-elements/js/**/*.js"
  ],
  theme: {
    extend: {
      keyframes: {
        slidein: {
          from: {
            opacity: "0",
            transform: "translateY(-10px)",
          },
          to: {
            opacity: "1",
            transform: "translateY(0)",
          },
        },
      },
      animation: {
        slidein: "slidein 1s ease 300ms",
      },
    },
  },
  plugins: [require("tw-elements/plugin.cjs")],
};