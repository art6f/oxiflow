import { Request, Response, NextFunction } from 'express';
import writeLog from './influx';

export function statsMiddleware(req: Request, res: Response, next: NextFunction): void {
  const start = Date.now();

  res.on('finish', () => {
    const stat = {
      endpoint: req.path,
      statusCode: res.statusCode,
      responseTimeMs: Date.now() - start,
      timestamp: new Date(),
    };

    writeLog(stat);
  });

  next();
}
