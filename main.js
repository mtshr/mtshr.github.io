import init, { run_app } from './pkg/github_pages.js';
async function main() {
	await init('./pkg/github_pages_bg.wasm');
	run_app();
}
main()
