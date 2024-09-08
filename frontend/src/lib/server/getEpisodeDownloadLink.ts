import { parse } from 'node-html-parser';
import fetchGogo from './fetchGogo';

export default async function getEpisodeDownloadUrl(episodeUrl: string): Promise<string | null> {
	const html = await fetchGogo(episodeUrl).then((res) => res.text());
	const document = parse(html);

	const btn = document.querySelector('.cf-download a:last-child');
	const link = btn?.getAttribute('href');

	return link ?? null;
}
