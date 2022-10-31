import { writable } from 'svelte/store';

export const modal = writable(null);
export const windowStyle = writable({});

export const settings = writable({
    refreshRate: 15,
    maxConnections: 500,
    connectionTimeout: 10,
    readTimeout: 15,
});

export const pools = writable([]);
