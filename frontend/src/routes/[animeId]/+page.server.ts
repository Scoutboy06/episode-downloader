import getAnimeInfo from '@/server/getAnimeInfo';
import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
  const res = await getAnimeInfo(params.animeId)

  if(res) return res;
  
  error(404, 'Not found')
};
