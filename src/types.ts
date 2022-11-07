export type Pool = {
  url: string;
  user: string;
};

export type Miner = {
  ip: string;
  make?: string;
  model?: string;
  mac?: string;
  hashrate?: number;
  temp?: number;
  fan?: number[];
  uptime?: number;
  errors?: string[];
  pools?: Pool[];
  sleep?: boolean;
  locate?: boolean;
};

export type Rack = {
  id: number;
  name: string;
  width: number;
  miners: Miner[][];
};
