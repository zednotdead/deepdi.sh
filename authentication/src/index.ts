import express, { type Express } from 'express';
import { logger } from './utils/log.ts';

const PORT: number = parseInt(process.env.PORT || '8333');
const app: Express = express();

function getRandomNumber(min: number, max: number) {
    return Math.floor(Math.random() * (max - min + 1) + min);
}

app.get('/rolldice', (_req, res) => {
    const randomnumber = getRandomNumber(1, 6).toString()

    res.send(randomnumber);
});

app.listen(PORT, () => {
    logger.info(`Listening for requests on http://localhost:${PORT}`);
});
