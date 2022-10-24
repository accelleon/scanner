export type Miner = {
  ip: string;
  make?: string;
  model?: string;
  hashrate?: number;
  temp?: number;
  fan?: number[];
  uptime?: number;
  mac?: string;
  errors?: string[];
};

export type Rack = {
  id: number;
  name: string;
  width: number;
  miners: Miner[][];
};
