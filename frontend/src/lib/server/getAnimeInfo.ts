import { parse } from 'node-html-parser';
import fetchGogo from './fetchGogo';

export default async function getAnimeInfo(alias: string) {
	const html = await fetchGogo(`/category/${alias}`).then((res) => res.text());
	const document = parse(html);

	const container = document.querySelector(
		'#wrapper_bg > section > section.content_left > div.main_body > div:nth-child(2) > div.anime_info_body_bg'
	);
	if (!container) return null;

	const title = container.querySelector('h1')?.textContent.trim();
	const posterUrl = container.querySelector('img')?.getAttribute('src');
	const type = container.querySelector('p:nth-child(4) > a')?.textContent.trim();
	const description = container.querySelector('.description')?.textContent.trim();
	const genres = container
		.querySelectorAll('p:nth-child(7) > a')
		.map((el) => el.textContent.replace(', ', ''));
	const released = container.querySelector('p:nth-child(8) > a')?.textContent;
	const status = container.querySelector('p:nth-child(9) > a')?.textContent;
	const otherNames = container
		.querySelectorAll('p:nth-child(10) > a')
		.map((el) => el.textContent.trim());

	const episodes = document.querySelectorAll('#episode_page > li > a')?.map((el) => ({
		start: Number(el.getAttribute('ep_start')),
		end: Number(el.getAttribute('ep_end'))
	}));

	return {
		title,
		posterUrl,
		type,
		description,
		genres,
		released,
		status,
		otherNames,
		episodes
	};
}
