import { Context } from "hono";
import { NewsService } from "../services/news-service";
import { AddNews, NewsArticle } from "../models/news-model";
import { getConnection } from "../middleware/connection";

// Controller get all news
export const getNewsData = async (c: Context) => {
  const pool = getConnection(c);
  const page = parseInt(c.req.query("page") || "1");
  const pageSize = parseInt(c.req.query("pageSize") || "10");
  const service = new NewsService(pool, c.env.KV);
  try {
    const data = await service.getNewsData(page, pageSize);
    return c.json(data);
  } catch (err) {
    console.error(err);
    return c.json({ error: "Internal Server Error" }, 500);
  }
};

// Controller create news
export const createNews = async (c: Context) => {
  try {
    const request = (await c.req.json()) as AddNews;
    console.log(request);
    const pool = getConnection(c);
    const service = new NewsService(pool, c.env.KV);
    await service.createNews(request);
    return c.json({ message: "News created successfully" });
  } catch (err: any) {
    return c.json(
      {
        error: `Error in createNews: ${err.message}`,
      },
      500
    );
  }
};

// Controller update news
export const updateNews = async (c: Context) => {
  try {
    const request = (await c.req.json()) as AddNews;
    const id = parseInt(c.req.param("id"));
    const pool = getConnection(c);
    const service = new NewsService(pool, c.env.KV);
    await service.updateNews(request, id);
    return c.json({ message: "News updated successfully" });
  } catch (err: any) {
    return c.json(
      {
        error: `Error in updateNews: ${err.message}`,
      },
      500
    );
  }
};

// Controller delete news
export const deleteNews = async (c: Context) => {
  try {
    const id = parseInt(c.req.param("id"));
    const pool = getConnection(c);
    const service = new NewsService(pool, c.env.KV);
    await service.deleteNews(id);
    return c.json({ message: "News deleted successfully" });
  } catch (err: any) {
    return c.json(
      {
        error: `Error in deleteNews: ${err.message}`,
      },
      500
    );
  }
};

// Controller get single news
export const getSingleNews = async (c: Context) => {
  try {
    const id = parseInt(c.req.param("id"));
    const pool = getConnection(c);
    const service = new NewsService(pool, c.env.KV);
    const news = await service.getSingleNews(id);
    const newsData = news[0];
    return c.json({ data: newsData });
  } catch (err: any) {
    return c.json(
      {
        error: `Error in getSingleNews: ${err.message}`,
      },
      500
    );
  }
};

// Controller create news article
export const createNewsArticle = async (c: Context) => {
  try {
    const body: NewsArticle = await c.req.json();
    const { slug, markdown } = body;

    if (!slug || !markdown) {
      return c.json({ error: "slug & markdown required" }, 400);
    }

    const pool = getConnection(c);
    const service = new NewsService(pool, c.env.KV);
    const article = await service.createNewsArticle({ slug, markdown });

    return c.json({ data: article });
  } catch (err: any) {
    return c.json({ error: `Error in createNewsArticle: ${err.message}` }, 500);
  }
};

export const getNewsArticle = async (c: Context) => {
  try {
    const slug = c.req.param("slug");
    const pool = getConnection(c);
    const service = new NewsService(pool, c.env.KV);
    const article = await service.getNewsArticle(slug);
    return c.json({ data: article });
  } catch (err: any) {
    return c.json({ error: `Error in getNewsArticle: ${err.message}` }, 500);
  }
};

// controller
export const updateNewsArticle = async (c: Context) => {
  try {
    const slug = c.req.param("slug");
    const body = await c.req.json<{ markdown: string }>();

    if (!body.markdown) {
      return c.json({ error: "markdown required" }, 400);
    }

    const pool = getConnection(c);
    const service = new NewsService(pool, c.env.KV);
    const updated = await service.updateNewsArticle(slug, body.markdown);

    return c.json({ data: updated });
  } catch (err: any) {
    return c.json({ error: `Error in updateNewsArticle: ${err.message}` }, 500);
  }
};
