# Mini Grep

```sh
❯ ./mini-grep 'pub fn ' ~/Documents/Code/OrganicCoding/mini-grep/src
A Mini Grep Write in Rust
parsed args: Args { pattern: Some("pub fn "), regexps: [], paths: ["/Users/alexz/Documents/Code/OrganicCoding/mini-grep/src"] }
/Users/alexz/Documents/Code/OrganicCoding/mini-grep/src/cmd.rs
	25:     pub fn all_patterns(&self) -> Vec<String> {
	36:     pub fn all_paths(&self) -> Vec<PathBuf> {

/Users/alexz/Documents/Code/OrganicCoding/mini-grep/src/search.rs
	19:     pub fn new(args: &Args) -> GrepResult<Self> {
	31:     pub fn is_match(&self, line: &[u8]) -> bool {
	36: pub fn split_lines(data: &[u8]) -> Vec<Vec<u8>> {
	43: pub fn search_file(path: &PathBuf, matcher: &Matcher) -> GrepResult<FileResult> {

/Users/alexz/Documents/Code/OrganicCoding/mini-grep/src/walk.rs
	7: pub fn collect_files(paths: &[PathBuf]) -> GrepResult<Vec<PathBuf>> {

```
