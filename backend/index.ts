import express from 'express';
import morgan from 'morgan';

const app = express();
const PORT = 3001;

app.use(morgan('dev'));

app.get('/', (_req, res) => {
  res.send('Hello World');
});

app.get('/health', (_req, res) => {
  res.json({ status: 'ok', uptime: process.uptime() });
});

app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});
