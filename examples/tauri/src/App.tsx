import { For, Suspense, createResource, createSignal } from "solid-js";
import { createPost, getPosts, Post } from "./bindings";

interface PostCreatorProps {
    onPostCreated(): void
}

const PostCreator = (props: PostCreatorProps) => {
    const [loading, setLoading] = createSignal(false);

    return <form class="bg-white/10 w-full rounded-md" onSubmit={async e => {
        e.preventDefault()
        const data = new FormData(e.target as HTMLFormElement);

        setLoading(true)

        try {
            await createPost({
                title: data.get("title") as string,
                content: data.get("content") as string,
            });

            (e.target as HTMLFormElement).reset()
            props.onPostCreated()
        }
        finally {
            setLoading(false)
        }
    }}>
        <input name="title" class="w-full bg-transparent px-4 py-2" placeholder="Title" />
        <textarea name="content" class="w-full bg-transparent px-4 py-2" placeholder="Content" />
        <button disabled={loading()} class="text-right m-2 py-2 px-4 rounded-full bg-blue-500 disabled:bg-gray-500">Post</button>
    </form>
}

interface PostListProps {
    posts: Post[]
}

const PostList = (props: PostListProps) => {
    return <ul class="space-y-2">
        <For each={props.posts}>
            {post => <li class="bg-white/10 p-4 rounded-md">
                <span class="text-xl">{post.title}</span>
                <p>{post.content}</p>
            </li>}
        </For>
    </ul>
}

const App = () => {
    const [posts, postsActions] = createResource(async () => {
        await new Promise(res => setTimeout(res, 1000))
        return await getPosts()
    });

    return (
        <div class="bg-gray-900 w-screen h-screen p-4 text-white">
            <PostCreator onPostCreated={postsActions.refetch} />
            <div class="h-px bg-white/30 my-2" />
            <Suspense fallback={"Loading Posts..."}>
                <PostList posts={posts()?.reverse()!} />
            </Suspense>
        </div>
    );
}

export default App
