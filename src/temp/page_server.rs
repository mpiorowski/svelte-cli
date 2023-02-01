pub fn page_server() -> &'static str {
    return 
"import { PageLoad } from \"./$types\";

export const load = (({}) => {
  return {};
}) satisfies PageLoad;";
}
