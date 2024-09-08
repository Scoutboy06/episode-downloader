<script lang="ts">
	import Button from '@/components/ui/button/button.svelte';
	import { Skeleton } from '@/components/ui/skeleton';
	import type getAnimeInfo from '@/server/getAnimeInfo';
	import type getEpisodes from '@/server/getEpisodes';
	import { onMount } from 'svelte';

	export let data: Awaited<ReturnType<typeof getAnimeInfo>>;

	let episodeTabIndex = 0;
	let episodes: Awaited<ReturnType<typeof getEpisodes>> | null = null;

	onMount(async () => {
		// episodes = await getEpisodes(data.);
		episodes = [];
	});
</script>

{#if !data}
	<h1>No anime found</h1>
{:else}
	<main class="flex flex-row gap-8 px-4 mt-8">
		<div class="w-1/3">
			<img src={data?.posterUrl} alt="Poster" />
		</div>

		<div class="flex flex-col w-2/3 gap-2">
			{#if data.title}
				<h1 class="text-3xl">{data.title}</h1>
			{/if}

			{#if data.type}
				<p><span class="text-slate-400">Type:</span> {data.type}</p>
			{/if}

			{#if data.description}
				<p>{data.description}</p>
			{/if}

			{#if data.genres}
				<p>
					<span class="text-slate-400">Genres:</span>

					{#each data.genres as genre, index}
						<span>{index != 0 ? ', ' : ''}{genre}</span>
					{/each}
				</p>
			{/if}

			{#if data.released}
				<p><span class="text-slate-400">Released:</span> {data.released}</p>
			{/if}

			{#if data.status}
				<p><span class="text-slate-400">Status:</span> {data.status}</p>
			{/if}

			{#if data.otherNames}
				<p>
					<span class="text-slate-400">Other names:</span>
					{#each data.otherNames as name, index}
						{index != 0 ? ', ' : ''}{name}
					{/each}
				</p>
			{/if}
		</div>
	</main>

	<section class="px-4 my-4">
		<h2 class="mb-2 text-lg">Episodes</h2>

		{#each data?.episodes ?? [] as episode, index}
			<Button
				variant={index === episodeTabIndex ? 'secondary' : 'outline'}
				on:click={() => (episodeTabIndex = index)}
			>
				{episode.start}-{episode.end}
			</Button>
		{/each}
	</section>

	<section class="flex flex-row gap-2 px-4">
		{#if episodes !== null}
			{#each episodes as episode}
				<Button variant="secondary">{episode.epNum}</Button>
			{/each}
		{:else}
			{#each { length: data.episodes[episodeTabIndex].end - data.episodes[episodeTabIndex].start } as _, i}
				<Skeleton class="inline-block w-24 h-11" style={`animation-delay: ${i * 50}ms`} />
			{/each}
		{/if}
	</section>
{/if}
