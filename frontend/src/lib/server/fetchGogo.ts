export default async function fetchGogo(
	path: string,
	options?: RequestInit & {
		queryParams?: URLSearchParams | Record<string, string>;
		headers?: RequestInit['headers'];
	}
) {
	const url = new URL(path, 'https://anitaku.pe/');

	if (options?.queryParams instanceof URLSearchParams) {
		for (const [key, value] of options.queryParams.entries()) {
			url.searchParams.set(key, value);
		}
	} else if (typeof options?.queryParams === 'object') {
		for (const [key, value] of Object.entries(options.queryParams)) {
			url.searchParams.set(key, value);
		}
	}

	const cookies = {
		gogoanime: 'mr8dvasisphogmkbi03mbk2s83',
		auth: '1plH5IVCxHPvQTK9rwhF7jMIWl6kk5cYUbNHQFCr3vAuS0v%2FaQcgk7inZkBaFf%2BX1iqDVXWsSAf2FlDzsrxNIA%3D%3D'
	};

	const _options = {
		...options,
		headers: {
			'accept-language': 'en-US,en;q=0.9,sv;q=0.8',
			'sec-ch-ua': '"Chromium";v="128", "Not;A=Brand";v="24", "Microsoft Edge";v="128"',
			'sec-ch-ua-mobile': '?0',
			'sec-ch-ua-platform': '"Windows"',
			'sec-fetch-dest': 'document',
			'sec-fetch-mode': 'navigate',
			'sec-fetch-site': 'cross-site',
			...options?.headers,
			cookie: Object.entries(cookies)
				.map((kv) => kv.join('='))
				.join('; ')
		}
	};

	return fetch(url, _options);
}
