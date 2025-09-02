import { Context } from "hono";
import { getConnection } from "../middleware/connection";
import { ArticleService } from "../services/article-service";
import { AddArticle } from "../models/article-model";

// Controller get all articles
export const getArticlesData = async (c: Context) => {
  const pool = getConnection(c);
  const page = parseInt(c.req.query("page") || "1");
  const pageSize = parseInt(c.req.query("pageSize") || "10");
  const service = new ArticleService(pool, c.env.KV);
  try {
    const data = await service.getArticlesData(page, pageSize);
    return c.json(data);
  } catch (err) {
    console.error(err);
    return c.json({ error: "Internal Server Error" }, 500);
  }
};

// Controller create article
export const createArticle = async (c: Context) => {
  try {
    const request = (await c.req.json()) as AddArticle;
    console.log(request);
    const pool = getConnection(c);
    const service = new ArticleService(pool, c.env.KV);
    await service.createArticle(request);
    return c.json({ message: "Article created successfully" });
  } catch (err: any) {
    return c.json({ error: `Error in createArticle: ${err.message}` }, 500);
  }
};
