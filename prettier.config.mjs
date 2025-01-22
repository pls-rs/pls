/** @type {import("prettier").Config} */
export default {
  trailingComma: "es5",
  astroAllowShorthand: true,
  bracketSameLine: true,
  singleAttributePerLine: true,
  overrides: [
    {
      files: ["*.astro"],
      options: {
        parser: "astro",
      },
    },
    {
      files: ["*.svg"],
      options: {
        parser: "html",
      },
    },
  ],
  proseWrap: "always",
};
