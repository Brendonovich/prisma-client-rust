/*
  Warnings:

  - You are about to drop the column `desc_` on the `Post` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "User" ADD COLUMN "underscored_" INTEGER;

-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Post" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL,
    "title" TEXT NOT NULL,
    "published" BOOLEAN NOT NULL,
    "views" INTEGER NOT NULL DEFAULT 0,
    "desc" TEXT,
    "author_id" TEXT,
    CONSTRAINT "Post_author_id_fkey" FOREIGN KEY ("author_id") REFERENCES "User" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);
INSERT INTO "new_Post" ("author_id", "created_at", "id", "published", "title", "updated_at", "views") SELECT "author_id", "created_at", "id", "published", "title", "updated_at", "views" FROM "Post";
DROP TABLE "Post";
ALTER TABLE "new_Post" RENAME TO "Post";
CREATE UNIQUE INDEX "Post_title_author_id_key" ON "Post"("title", "author_id");
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
