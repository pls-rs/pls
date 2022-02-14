const packageInfo = require("../../package.json");

const features = {
  text: "Features",
  children: ["/features/colors", "/features/icons"],
};

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
      { text: "Get started", link: "/get_started" },
      features,
      { text: "PyPI", link: "https://pypi.org/project/pls/" },
    ],

    sidebar: {
      "/features/": [features],
    },
  },
};
