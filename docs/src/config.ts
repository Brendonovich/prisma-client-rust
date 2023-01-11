export const SITE = {
  title: "Prisma Client Rust",
  description: "Type-safe database access for Rust",
  defaultLanguage: "en_US",
};

export const OPEN_GRAPH = {
  image: {
    src: "https://github.com/withastro/astro/blob/main/assets/social/banner.jpg?raw=true",
    alt:
      "astro logo on a starry expanse of space," +
      " with a purple saturn-like planet floating in the right foreground",
  },
  twitter: "astrodotbuild",
};

export const KNOWN_LANGUAGES = {
  English: "en",
};

export const GITHUB_EDIT_URL = `https://github.com/brendonovich/prisma-client-rust/blob/main/docs/`;
export const DISCORD_INVITE_URL = `https://discord.gg/5M6fpszrry`;

// Uncomment this to enable site search.
// See "Algolia" section of the README for more information.
// export const ALGOLIA = {
//   indexName: 'XXXXXXXXXX',
//   appId: 'XXXXXXXXXX',
//   apiKey: 'XXXXXXXXXX',
// }

function section(name: string, data: any[]) {
  let fsName = name.toLowerCase().replace(" ", "-");

  return [
    { text: name, header: true },
    ...data.map((d) => ({ ...d, link: `${fsName}/${d.link}` })),
  ];
}

export const SIDEBAR = {
  en: [
    { text: "Introduction", link: "introduction" },
    { text: "0.6.0 Migration", link: "0.6.0-migration" },
    ...section("Getting Started", [
      { text: "Installation", link: "installation" },
      { text: "Setup", link: "setup" },
      { text: "Structure", link: "structure" },
    ]),
    ...section("Reading Data", [
      { text: "Find Queries", link: "find" },
      { text: "Fetching Relations", link: "fetch" },
      { text: "Pagination", link: "pagination" },
      { text: "Ordering", link: "order-by" },
      { text: "Counting Records", link: "count" },
      { text: "Select & Include", link: "select-include" },
    ]),
    ...section("Writing Data", [
      { text: "Create Queries", link: "create" },
      { text: "Update Queries", link: "update" },
      { text: "Delete Queries", link: "delete" },
      { text: "Upserting", link: "upsert" },
    ]),
    ...section("Extra", [
      { text: "Raw Queries", link: "raw" },
      { text: "Batching Queries", link: "batching" },
      { text: "Transactions", link: "transactions" },
      { text: "Mocking Queries", link: "mocking" },
      { text: "Error Handling", link: "error-handling" },
      { text: "Migrations", link: "migrations" },
      { text: "rspc Integration", link: "rspc" },
      { text: "Query Traits", link: "traits" },
    ]),
  ],
};
