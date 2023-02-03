# Svelte Cli
Svelte cli for fast Svelte/SvelteKit scaffolding using the newest routing. Written fully using Rust.

WIP

## Install
```
cargo install sveltecli
```
Please make sure that `~/.cargo/bin` is in your PATH.  
In the future more distro related options will be added :)

## Usage
```
sv add [pages] -p [path]
```
This command will add specified routing files with the basic template. 
Example: 
```
sv add ps -p reports
```
This will add `+page.server.ts` to Your `reports` folder (it will create it if not existed) with the following template:
```
import { Actions, PageServerLoad } from "./$types";

export const load = (({ }) => {
  return {};
}) satisfies PageServerLoad;

export const actions = {
    formAction: async ({ request }) => {
        const form = await request.formData();
        const id = form.get('id');

        return { id };
    },
} satisfies Actions;
```
You can also add multiple files in one command:
```
sv add ps p e l
```

## Pages
| Command      | Page             |
| ----------- | ------------------|
| e           | +error.svelte     |
| l           | +layout.svelte    |
| lc          | +layout.ts        |
| ls          | +layout.server.ts |
| p           | +page.svelte      |
| pc          | +page.ts          |
| ps          | +page.server.ts   |
| s           | +server.ts        |

## Config
You can also set Your default templates. To do that, create a folder and put the files with the template You want inside. You don't need to put all the pages, the cli will use the default one if some will be missing. Then add it:
```
sv config temp [path_to_your_templates_folder]
```

