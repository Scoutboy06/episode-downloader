import { json, type RequestHandler } from '@sveltejs/kit';
import { parse } from 'node-html-parser';

const bgImgRegex = /background: url\("(.+)"\)/;

export type SearchResponse = Array<{
	name: string;
	link: string;
	img: string;
}>;

export const GET: RequestHandler = async ({ url }) => {
	const query = url.searchParams.get('q') || '';
	const category = url.searchParams.get('cat') || 'all';

	const queryParams = new URLSearchParams({ keyword: query, cat: category });

	const fetchUrl = new URL(
		'/site/loadAjaxSearch?' + queryParams.toString(),
		'https://ajax.gogocdn.net/'
	);
	const data = await fetch(fetchUrl).then((res) => res.json());
	const ajax = data.content;

	const document = parse(ajax);
	const items = document.querySelectorAll('.list_search_ajax > a').map((el) => {
		return {
			name: el.textContent.trim(),
			link: el.getAttribute('href')?.split('/').at(-1) ?? null,
			img: el.querySelector('div')?.getAttribute('style')?.match(bgImgRegex)?.[1]
		};
	});

	return json(<SearchResponse>items);
};
