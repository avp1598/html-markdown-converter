import express, { Express, Request, Response } from "express";
import { greet } from "html-markdown-converter";

const app: Express = express();
const port = process.env.PORT || 5001;

app.get("/", (req: Request, res: Response) => {
  res.send("Hello from Express + TypeScript Server");
});

app.listen(port, () => {
  console.log(`⚡️[server]: Server is running at http://localhost:${port}`);
  greet("Aditya");
});
