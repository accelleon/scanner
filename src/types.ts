export type Pool = {
  url: string;
  user: string;
};

export interface Profile {
  type: string;
  name?: string;
  power?: number;
  ths?: number;
  volt?: number;
  freq?: number;
  min_freq?: number;
  max_freq?: number;
  min_volt?: number;
  max_volt?: number;
}

export type Miner = {
  ip: string;
  make?: string;
  model?: string;
  submodel?: string;
  mac?: string;
  hashrate?: number;
  temp?: number;
  fan?: number[];
  uptime?: number;
  errors?: string[];
  pools?: Pool[];
  sleep?: boolean;
  locate?: boolean;
  power?: number;
  efficiency?: number;
  profile?: Profile;
  profiles?: Profile[];
  hashboard?: string;
  nameplate?: number;
};

export type Rack = {
  id: number;
  name: string;
  width: number;
  miners: Miner[][];
};
