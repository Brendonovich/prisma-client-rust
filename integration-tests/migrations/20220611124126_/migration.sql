/*
  Warnings:

  - The primary key for the `Types` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `bigint` on the `Types` table. All the data in the column will be lost.
  - You are about to drop the column `decimal` on the `Types` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[id]` on the table `Profile` will be added. If there are existing duplicate values, this will fail.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Types" (
    "id" INTEGER NOT NULL DEFAULT 0,
    "bool_" BOOLEAN NOT NULL DEFAULT false,
    "string" TEXT NOT NULL DEFAULT '',
    "integer" INTEGER NOT NULL DEFAULT 0,
    "datetime" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "float_" REAL NOT NULL DEFAULT 0,

    PRIMARY KEY ("id", "string")
);
INSERT INTO "new_Types" ("bool_", "datetime", "float_", "id", "integer", "string") SELECT "bool_", "datetime", "float_", "id", "integer", "string" FROM "Types";
DROP TABLE "Types";
ALTER TABLE "new_Types" RENAME TO "Types";
CREATE UNIQUE INDEX "Types_id_string_key" ON "Types"("id", "string");
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;

-- CreateIndex
CREATE UNIQUE INDEX "Profile_id_key" ON "Profile"("id");
