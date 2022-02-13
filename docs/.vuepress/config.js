const packageInfo = require("../../package.json");

module.exports = {
  lang: "en-GB",
  title: "pls",
  description: packageInfo.description,
  base: "/pls/",

  theme: "@vuepress/theme-default",
  themeConfig: {
    repo: packageInfo.repository.replace("github:", ""),
    docsBranch: "docs",
    docsDir: "docs",

    navbar: [
      {
        text: "Get started",
        link: "/get_started",
      },
      {
        text: "PyPI",
        link: "https://pypi.org/project/pls/",
      },
    ],
  },
};
