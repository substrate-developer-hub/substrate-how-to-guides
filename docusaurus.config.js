/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: "Substrate How-to Guides",
  tagline: "A modular approach to learning Substrate for all levels",
  url: "https://substrate-developer-hub.github.io",
  baseUrl: "/substrate-how-to-guides/",
  onBrokenLinks: "ignore", // Change back to `throw` eventually 
  onBrokenMarkdownLinks: "warn",
  favicon: "img/favicon.ico",
  organizationName: "substrate-developer-hub", 
  projectName: "substrate-how-to-guides", 
  themeConfig: {
    navbar: {
      title: "Welcome",
      logo: {
        alt: "My Site Logo",
        src: "img/substrate.png",
      },
      items: [
        {
          type: "doc",
          docId: "intro",
          position: "left",
          label: "How-to Guides",
        },
        {
          href: "https://github.com/substrate-developer-hub/substrate-how-to-guides/",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Learn Substrate",
          items: [
            {
              label: "How-to Guides",
              to: "/docs/intro",
            },
            {
              label: "Tutorials",
              to: "/docs/tutorials/kitties/overview",
            },
          ],
        },
      ],
      copyright: `The Substrate How-to Guides are GPL 3.0 Licensed, open source and open for contributions.`,
    },
    prism: { 
      additionalLanguages: ['rust'],
    },
  },
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          // Please change this to your repo.
          editUrl:
            "https://github.com/substrate-developer-hub/substrate-how-to-guides/edit/main/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
  plugins: [
    [require.resolve('@cmfcmf/docusaurus-search-local'), {

      // whether to index docs pages
      indexDocs: true,  
      docsRouteBasePath: '/docs',
      // Whether to also index the titles of the parent categories in the sidebar of a doc page.
      // 0 disables this feature.
      // 1 indexes the direct parent category in the sidebar of a doc page
      // 2 indexes up to two nested parent categories of a doc page
      indexDocSidebarParentCategories: 0
    }],
    
  ],

  themes: [
      '@saucelabs/theme-github-codeblock'
      ],

};
