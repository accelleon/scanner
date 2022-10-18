export type Miner = {
    ip: string;
    make?: string;
    model?: string;
    hashrate?: number;
};

export type Rack = {
    id: number;
    name: string;
    width: number;
    miners: Miner[][];
};