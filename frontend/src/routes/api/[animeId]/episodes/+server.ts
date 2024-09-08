import getEpisodes from '@/server/getEpisodes';
import { error, json, type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ params }) => {
  const episodes = await getEpisodes(params.animeId!, 0, 24);
  if(!episodes) return error(404, "No episodes found");
  return json(episodes)
}