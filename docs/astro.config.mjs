import { defineConfig } from "astro/config";
import preact from "@astrojs/preact";

import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  integrations: [
    preact(),
    tailwind({
      config: {
        applyBaseStyles: false,
      },
    }),
  ],
  site: `https://prisma.brendonovich.dev`,
});
