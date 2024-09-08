import { parse } from 'node-html-parser';

export default async function getEpisodes(animeId: string, ep_start: number, ep_end: number) {
  // https://ajax.gogocdn.net/ajax/load-list-episode?ep_start=0&ep_end=9&id=15289&default_ep=0&alias=oshi-no-ko-2nd-season
  const url = new URL("https://ajax.gogocdn.net/ajax/load-list-episode");
  url.searchParams.set('id', animeId);
  url.searchParams.set('ep_start', ep_start.toString());
  url.searchParams.set('ep_end', ep_end.toString());
  console.log(url.toString())

  const cookies = {
    auth: '1plH5IVCxHPvQTK9rwhF7jMIWl6kk5cYUbNHQFCr3vAuS0v%2FaQcgk7inZkBaFf%2BX1iqDVXWsSAf2FlDzsrxNIA%3D%3D',
    gogoanime: 'mr8dvasisphogmkbi03mbk2s83'
  };

  const html = await fetch(url, {
    headers: {
      'accept-language': "en-US,en;q=0.9,sv;q=0.8",
      "sec-ch-ua": "\"Chromium\";v=\"128\", \"Not;A=Brand\";v=\"24\", \"Microsoft Edge\";v=\"128\"",
      "sec-ch-ua-mobile": "?0",
      "sec-ch-ua-platform": "\"Windows\"",
      "sec-fetch-dest": "document",
      "sec-fetch-mode": "navigate",
      "sec-fetch-site": "cross-site",
      cookie: Object.entries(cookies).map(kv => kv.join('=')).join('; ')
    },
    body: null,
    method: 'GET'
  }).then(res => res.text());
  const document = parse(html);

  const episodes = document.querySelectorAll('#episode_related > li > a').map(el => ({
    epNum: Number(el.textContent.trim().split(' ')[1]),
    url: el.getAttribute('href')?.trim()
  }));

  return episodes;
}