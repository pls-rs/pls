const { path } = require("@vuepress/utils");
const packageInfo = require("../../package.json");

const features = {
  text: "Features",
  children: [
    "/features/colors",
    "/features/icons",
    "/features/suffixes",
    "/features/details",
    "/features/filtering",
    "/features/sorting",
    "/features/importance",
    "/features/collapse",
  ],
};

module.exports = {
  lang: "en-GB",
  title: "pls",
  description: packageInfo.description,
  base: "/pls/",

  head: [["link", { rel: "icon", href: "/pls/favicon.png" }]],

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
      "/get_started": ["/get_started"],
      "/features": [features],
    },
  },
  alias: Object.fromEntries(
    ["HomeHero", "HomeFooter"].map((component) => [
      `@theme/${component}.vue`,
      path.resolve(__dirname, `./components/overrides/${component}.vue`),
    ])
  ),

  plugins: [
    [
      "@vuepress/plugin-docsearch",
      {
        apiKey: "aab9e7596d3aa3ef1a9834543eadbf60",
        indexName: "pls",
        appId: "V3X44L2GDB",
        placeholder: "Search...",
      },
    ],
    [
      "@vuepress/plugin-register-components",
      {
        components: Object.fromEntries(
          ["GitHubStars"].map((component) => [
            component,
            path.resolve(__dirname, `./components/custom/${component}.vue`),
          ])
        ),
      },
    ],
  ],
};
