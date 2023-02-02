pub fn error_svelte() -> &'static str {
    return "<script lang=\"ts\">
  import { page } from '$app/stores';
</script>

<h1>{$page.status}: {$page.error.message}</h1>";
}

pub fn layout_svelte() -> &'static str {
    return "<script lang=\"ts\">
    import { LayoutData } from \"./$types\";

    export let data: LayoutData;
</script>

<slot />";
}

pub fn layout_client() -> &'static str {
    return "import { LayoutLoad } from \"./$types\";

export const load = (({ }) => {
  return {};
}) satisfies LayoutLoad;";
}

pub fn layout_server() -> &'static str {
    return "import { LayoutServerLoad } from \"./$types\";

export const load = (({ }) => {
  return {};
}) satisfies LayoutServerLoad;";
}

pub fn page_svelte() -> &'static str {
    return "<script lang=\"ts\">
    import { PageData } from \"./$types\";

    export let data: PageData;
</script>

<h1>Page</h1>";
}

pub fn page_client() -> &'static str {
    return "import { PageLoad } from \"./$types\";

export const load = (({ }) => {
  return {};
}) satisfies PageLoad;";
}

pub fn page_server() -> &'static str {
    return "import { Actions, PageServerLoad } from \"./$types\";

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

pub fn server() -> &'static str {
    return "import type { RequestHandler } from \"./$types\";

export const GET = ( ({ url }) => {
  return new Response(String(url));
}) satisfies RequestHandler;";
}
