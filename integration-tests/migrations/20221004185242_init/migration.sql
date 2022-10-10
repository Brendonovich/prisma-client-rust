-- CreateTable
CREATE TABLE "Post" (
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

-- CreateTable
CREATE TABLE "User" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "email" TEXT,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "underscored_" INTEGER
);

-- CreateTable
CREATE TABLE "FilePath" (
    "local_id" INTEGER NOT NULL,
    "path" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,

    PRIMARY KEY ("user_id", "local_id"),
    CONSTRAINT "FilePath_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "User" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Category" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "Profile" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "user_id" TEXT NOT NULL,
    "bio" TEXT NOT NULL,
    "city" TEXT,
    "country" TEXT NOT NULL,
    "views" INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT "Profile_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "User" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Types" (
    "id" INTEGER NOT NULL DEFAULT 0,
    "bool_" BOOLEAN NOT NULL DEFAULT false,
    "string" TEXT NOT NULL DEFAULT '',
    "integer" INTEGER NOT NULL DEFAULT 0,
    "datetime" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "float_" REAL NOT NULL DEFAULT 0,

    PRIMARY KEY ("id", "string")
);

-- CreateTable
CREATE TABLE "_favouritePosts" (
    "A" TEXT NOT NULL,
    "B" TEXT NOT NULL,
    CONSTRAINT "_favouritePosts_A_fkey" FOREIGN KEY ("A") REFERENCES "Post" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "_favouritePosts_B_fkey" FOREIGN KEY ("B") REFERENCES "User" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "_CategoryToPost" (
    "A" INTEGER NOT NULL,
    "B" TEXT NOT NULL,
    CONSTRAINT "_CategoryToPost_A_fkey" FOREIGN KEY ("A") REFERENCES "Category" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "_CategoryToPost_B_fkey" FOREIGN KEY ("B") REFERENCES "Post" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX "Post_title_author_id_key" ON "Post"("title", "author_id");

-- CreateIndex
CREATE UNIQUE INDEX "User_email_key" ON "User"("email");

-- CreateIndex
CREATE UNIQUE INDEX "Profile_id_key" ON "Profile"("id");

-- CreateIndex
CREATE UNIQUE INDEX "Profile_user_id_key" ON "Profile"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "Types_id_string_key" ON "Types"("id", "string");

-- CreateIndex
CREATE UNIQUE INDEX "_favouritePosts_AB_unique" ON "_favouritePosts"("A", "B");

-- CreateIndex
CREATE INDEX "_favouritePosts_B_index" ON "_favouritePosts"("B");

-- CreateIndex
CREATE UNIQUE INDEX "_CategoryToPost_AB_unique" ON "_CategoryToPost"("A", "B");

-- CreateIndex
CREATE INDEX "_CategoryToPost_B_index" ON "_CategoryToPost"("B");
