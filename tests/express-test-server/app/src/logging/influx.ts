import { InfluxDB, Point, WriteApi } from '@influxdata/influxdb-client';

const influx = new InfluxDB({
  url: process.env.INFLUX_URL!,
  token: process.env.INFLUX_TOKEN!,
});

const writeApi: WriteApi = influx.getWriteApi(
  process.env.INFLUX_ORG!,
  process.env.INFLUX_BUCKET!
);

interface Stat {
  endpoint: string;
  statusCode: number;
  responseTimeMs: number;
  timestamp: Date;
}

export default function writeLog(stat: Stat): void {
  const point = new Point('request')
    .tag('endpoint', stat.endpoint)
    .intField('statusCode', stat.statusCode)
    .intField('responseTimeMs', stat.responseTimeMs)
    .timestamp(stat.timestamp);

  writeApi.writePoint(point);
}
