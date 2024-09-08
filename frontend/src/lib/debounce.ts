/* eslint-disable @typescript-eslint/no-explicit-any */

// eslint-disable-next-line @typescript-eslint/no-unsafe-function-type
export default function debounce<T extends Function>(cb: T, wait = 20) {
	let h: NodeJS.Timeout;
	const callable = (...args: any) => {
		clearTimeout(h);
		h = setTimeout(() => cb(...args), wait);
	};
	return <T>(<any>callable);
}
