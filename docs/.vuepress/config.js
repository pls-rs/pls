const Prism = require("prismjs");
const loadLanguages = require("prismjs/components/");

const { path } = require("@vuepress/utils");
const packageInfo = require("../../package.json");

/* Configure Prism */

loadLanguages(["bash"]);
bash_functions = ["gh", "git", "just", "pip", "pipx", "pls", "poetry"].join(
  "|"
);
Prism.languages.bash.function.pattern = new RegExp(
  `(^|[\\s;|&]|[<>]\\()(?:${bash_functions})(?=$|[)\\s;|&])`
);

const get_started = {
  text: "Get started",
  children: ["/get_started/installation", "/get_started/usage"],
};

const about = {
  text: "About",
  children: ["/about/faq", "/about/comparison"],
};

const features = {
  text: "Features",
  children: [
    "/features",
    "/features/colors",
    "/features/icons",
    "/features/multi_cols",
    "/features/suffixes",
    "/features/details",
    "/features/sorting",
    "/features/filtering",
    "/features/importance",
    "/features/collapse",
    "/features/upcoming",
  ],
};

const reference = {
  text: "Reference",
  children: [
    "/reference",
    "/reference/node_specs",
    "/reference/icons",
    "/reference/prefs",
  ],
};

const contributing = {
  text: "Contributing",
  children: ["/contributing"],
};

module.exports = {
  lang: "en-GB",
  title: packageInfo.name,
  description: packageInfo.description,
  base: "/pls/",

  head: [
    ["link", { rel: "icon", href: "/pls/favicon.png" }],
    ["meta", { content: packageInfo.name, property: "og:title" }],
    ["meta", { content: packageInfo.homepage, property: "og:url" }],
    ["meta", { content: packageInfo.description, property: "og:description" }],
    [
      "meta",
      {
        content: `${packageInfo.homepage}/opengraph.png`,
        property: "og:image",
      },
    ],
  ],

  markdown: {
    code: {
      lineNumbers: false,
    },
  },

  theme: "@vuepress/theme-default",
  themeConfig: {
    repo: packageInfo.repository.replace("github:", ""),
    docsBranch: "docs",
    docsDir: "docs",

    navbar: [
      get_started,
      about,
      features,
      reference,
      contributing,
      { text: "PyPI", link: "https://pypi.org/project/pls/" },
    ],

    sidebar: {
      "/get_started": [get_started],
      "/about": [about],
      "/features": [features],
      "/reference": [reference],
      "/contributing": [contributing],
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
          ["ColorPreview"].map((component) => [
            component,
            path.resolve(__dirname, `./components/custom/${component}.vue`),
          ])
        ),
      },
    ],
  ],
};
