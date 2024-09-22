<script lang="ts">
	import { PUBLIC_API_URI } from '$env/static/public';
	// import { FilePathPicker } from '@/components/FolderPicker';
	import { Search } from '@/components/Search';
	import { Button } from '@/components/ui/button';
	import Home from 'lucide-svelte/icons/house';
	import { onMount } from 'svelte';
	import * as Tabs from '@/components/ui/tabs';
	import { page } from '$app/stores';

	let seriesId: string | null = null;

	let activeTab = '';

	type SeriesData = {
		title: string;
		poster_url: string;
		series_type: string;
		description: string;
		genres: string[];
		released: string;
		status: string;
		other_name: string;
		episodes: Array<{ start: number; end: number }>;
	};

	let data: SeriesData | null = null;

	async function loadSeries() {
		if (!seriesId) console.error('`id` not defined in query params');

		data = await fetch(`${PUBLIC_API_URI}/api/series?id=${seriesId}`).then((res) => res.json());
		activeTab = data!.episodes[0].start + '-' + data!.episodes[0].end;
	}

	onMount(loadSeries);

	$: if ($page.url.searchParams.get('id') !== seriesId) {
		seriesId = $page.url.searchParams.get('id');
		loadSeries();
	}
</script>

<div class="dark">
	<header class="flex flex-row gap-2 p-2 align-middle">
		<Button href="/" variant="ghost">
			<Home />
		</Button>

		<Search />
	</header>

	<main class="mx-2 mt-6">
		<!-- <FilePathPicker folderStore={seriesFolderStore} /> -->
	</main>
</div>

{#if data}
	<main class="flex flex-row gap-8 px-4 mt-8">
		<div class="w-1/3">
			<img src={data.poster_url} alt={data.title} />
		</div>

		<div class="flex flex-col w-2/3 gap-3">
			<h1 class="text-3xl">{data.title}</h1>
			<p class="mb-4">{data.other_name}</p>

			<p><span class="text-slate-400">Type:</span> {data.series_type}</p>

			<p>{data.description}</p>

			<p>
				<span class="text-slate-400">Genres:</span>

				{#each data.genres as genre, i}
					<span>{genre}</span>
				{/each}
			</p>

			<p><span class="text-slate-400">Released:</span> {data.released}</p>

			<p><span class="text-slate-400">Status:</span> {data.status}</p>
		</div>
	</main>

	<section>
		<Tabs.Root bind:value={activeTab} class="mx-4 mt-8">
			<Tabs.List>
				{#each data.episodes as episode}
					<Tabs.Trigger value={episode.start + '-' + episode.end}
						>{episode.start + '-' + episode.end}</Tabs.Trigger
					>
				{/each}
			</Tabs.List>

			{#each data.episodes as episode}
				{#if activeTab === episode.start + '-' + episode.end}
					<Tabs.Content value={episode.start + '-' + episode.end} class="flex flex-wrap gap-2">
						<!-- TODO: Fetch actual episode list from API -->
						{#each { length: episode.end - episode.start } as _, i}
							<Button variant="outline">Ep {episode.start + i}</Button>
						{/each}
					</Tabs.Content>
				{/if}
			{/each}
		</Tabs.Root>
	</section>
{/if}
