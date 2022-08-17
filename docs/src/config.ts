export const SITE = {
  title: 'Prisma Client Rust',
  description: 'Type-safe database access for Rust',
  defaultLanguage: 'en_US',
};

export const OPEN_GRAPH = {
  image: {
    src: 'https://github.com/withastro/astro/blob/main/assets/social/banner.jpg?raw=true',
    alt:
      'astro logo on a starry expanse of space,' +
      ' with a purple saturn-like planet floating in the right foreground',
  },
  twitter: 'astrodotbuild',
};

export const KNOWN_LANGUAGES = {
  English: 'en',
};

// Uncomment this to add an "Edit this page" button to every page of documentation.
export const GITHUB_EDIT_URL = `https://github.com/brendonovich/prisma-client-rust/blob/main/docs/`;

// Uncomment this to add an "Join our Community" button to every page of documentation.
// export const COMMUNITY_INVITE_URL = `https://astro.build/chat`;

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
    ...data.map(d => ({ ...d, link: `${fsName}/${d.link}` }))
  ]
}

export const SIDEBAR = {
  en: [
    { text: 'Introduction', link: 'introduction' },
    ...section("Getting Started", [
      { text: 'Installation', link: 'installation' },
      { text: 'Setup', link: 'setup' },
      { text: 'Syntax', link: 'syntax' },
    ]),
    ...section("Reading Data", [
      { text: "Find Queries", link: 'find' },
      { text: "Fetching Relations", link: 'fetch' },
      { text: "Pagination", link: 'pagination' },
      { text: "Ordering", link: 'order-by' },
      { text: "Counting Records", link: 'count' },
      { text: "Selecting Fields", link: 'select' }
    ]),
    ...section("Writing Data", [
      { text: "Create Queries", link: 'create' },
      { text: "Update Queries", link: 'update' },
      { text: "Delete Queries", link: 'delete' },
      { text: "Upserting", link: 'upsert' },
    ]),
    ...section("Extra", [
      { text: "Raw Queries", link: 'raw' },
      { text: "Error Handling", link: "error-handling" }
    ])
  ],
};
