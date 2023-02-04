pub fn error_svelte() -> &'static str {
    return "<script lang=\"ts\">
  import { page } from '$app/stores';
</script>

<h1>{$page.status}: {$page.error.message}</h1>";
}

pub fn error_svelte_js() -> &'static str {
    return "<script>
  import { page } from '$app/stores';
</script>

<h1>{$page.status}: {$page.error.message}</h1>";
}

pub fn layout_svelte() -> &'static str {
    return "<script lang=\"ts\">
    import type { LayoutData } from \"./$types\";

    export let data: LayoutData;
</script>

<slot />";
}

pub fn layout_svelte_js() -> &'static str {
    return "<script>
    export let data;
</script>

<slot />";
}

pub fn layout_client() -> &'static str {
    return "import type { LayoutLoad } from \"./$types\";

export const load = (({ }) => {
    return {};
}) satisfies LayoutLoad;";
}

pub fn layout_client_js() -> &'static str {
    return "export const load = ({ }) => {
    return {};
}";
}

pub fn layout_server() -> &'static str {
    return "import type { LayoutServerLoad } from \"./$types\";

export const load = (({ }) => {
    return {};
}) satisfies LayoutServerLoad;";
}

pub fn layout_server_js() -> &'static str {
    return "export const load = ({ }) => {
    return {};
}";
}

pub fn page_svelte() -> &'static str {
    return "<script lang=\"ts\">
    import type { PageData } from \"./$types\";

    export let data: PageData;
</script>

<h1>Page</h1>";
}

pub fn page_svelte_js() -> &'static str {
    return "<script>
    export let data: PageData;
</script>

<h1>Page</h1>";
}

pub fn page_client() -> &'static str {
    return "import type { PageLoad } from \"./$types\";

export const load = (({ }) => {
    return {};
}) satisfies PageLoad;";
}

pub fn page_client_js() -> &'static str {
    return "export const load = ({ }) => {
    return {};
}";
}

pub fn page_server() -> &'static str {
    return "import type { Actions, PageServerLoad } from \"./$types\";

export const load = (({ }) => {
    return {};
}) satisfies PageServerLoad;

export const actions = {
    formAction: async ({ request }) => {
        const form = await request.formData();
        const id = form.get('id');

        return { id };
    },
} satisfies Actions;";
}

pub fn page_server_js() -> &'static str {
    return "export const load = ({ }) => {
    return {};
};

export const actions = {
    formAction: async ({ request }) => {
        const form = await request.formData();
        const id = form.get('id');

        return { id };
    },
};";
}

pub fn server() -> &'static str {
    return "import type { RequestHandler } from \"./$types\";

export const GET = ( ({ url }) => {
    return new Response(String(url));
}) satisfies RequestHandler;";
}

pub fn server_js() -> &'static str {
    return "export const GET = ({ url }) => {
    return new Response(String(url));
};";
}
