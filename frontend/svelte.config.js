import adapterNode from '@sveltejs/adapter-node';
import adapterCloudflare from '@sveltejs/adapter-cloudflare';

const adapter = process.env.CF_PAGES ? adapterCloudflare() : adapterNode();

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter
	}
};

export default config;
