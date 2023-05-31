import express, { Express, Request, Response } from "express";
import { html_to_markdown } from "html-markdown-converter";
import { readFileSync } from "fs";
import { NodeHtmlMarkdown } from "node-html-markdown";

const app: Express = express();
const port = process.env.PORT || 5001;

app.get("/", (req: Request, res: Response) => {
  res.send("Hello from Express + TypeScript Server");
});

app.listen(port, () => {
  console.log(`⚡️[server]: Server is running at http://localhost:${port}`);
  const html = readFileSync("../tests/output.html", "utf8");
  const rust_output = html_to_markdown(html);
  const nhm = new NodeHtmlMarkdown();
  console.time("node");
  const node_output = nhm.translate(html);
  console.timeEnd("node");
});
