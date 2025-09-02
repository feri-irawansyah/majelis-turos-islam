import { AddNews, News, NewsArticle } from "../models/news-model";

export class NewsService {
  constructor(private db: D1Database, private kv: KVNamespace) {}

  async getNewsData(
    page = 1,
    pageSize = 10
  ): Promise<{ rows: News[]; total: number; totalPages: number }> {
    const offset = (page - 1) * pageSize;

    // ambil data
    const stmt = this.db.prepare(
      "SELECT * FROM mti_news ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    const result = await stmt.bind(pageSize, offset).all<News>();
    if (result.error) throw new Error(result.error);

    // hitung total rows
    const countStmt = this.db.prepare("SELECT COUNT(*) as count FROM mti_news");
    const countResult = await countStmt.all<{ count: number }>();
    if (countResult.error) throw new Error(countResult.error);

    const total = countResult.results[0].count;
    const totalPages = Math.ceil(total / pageSize);

    return {
      rows: result.results,
      total,
      totalPages,
    };
  }

  async createNews(request: AddNews): Promise<void> {
    console.log(request);
    const stmt = this.db.prepare(
      "INSERT INTO mti_news (title, content, slug, created_at) VALUES (?, ?, ?, datetime('now'))"
    );
    await stmt
      .bind(request.title, request.content, request.slug)
      .run()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        throw new Error(err);
      });
  }

  async updateNews(request: AddNews, id: number): Promise<void> {
    if (!id) {
      throw new Error("ID is required");
    }
    const stmt = this.db.prepare(
      "UPDATE mti_news SET title = ?, content = ?, slug = ?, created_at = datetime('now') WHERE news_id = ?"
    );
    await stmt
      .bind(request.title, request.content, request.slug, id)
      .run()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        throw new Error(err);
      });
  }

  async deleteNews(id: number): Promise<void> {
    if (!id) {
      throw new Error("ID is required");
    }
    const stmt = this.db.prepare("DELETE FROM mti_news WHERE news_id = ?");
    await stmt
      .bind(id)
      .run()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        throw new Error(err);
      });
  }

  // Service CRUD single news
  async getSingleNews(id: number): Promise<News[]> {
    const stmt = this.db
      .prepare("SELECT * FROM mti_news WHERE news_id = ?")
      .bind(id);
    const result = await stmt.all<News>();
    if (result.error) {
      console.log(result.error);
      throw new Error(result.error);
    }
    return result.results;
  }

  async createNewsArticle(article: NewsArticle) {
    const key = `news:${article.slug}`;
    const exists = await this.kv.get(key);

    if (exists) {
      throw new Error("Slug already exists");
    }

    const data = {
      ...article,
      createdAt: new Date().toISOString(),
    };

    await this.kv.put(key, JSON.stringify(data));
    return data;
  }

  async getNewsArticle(slug: string) {
    const key = `news:${slug}`;
    const value = await this.kv.get(key);
    return value ? JSON.parse(value) : null;
  }

  // service
  async updateNewsArticle(slug: string, markdown: string) {
    const key = `news:${slug}`;
    const exists = await this.kv.get(key);
    if (!exists) throw new Error("Article not found");

    const oldData = JSON.parse(exists);
    const newData = {
      ...oldData,
      markdown,
      updatedAt: new Date().toISOString(),
    };

    await this.kv.put(key, JSON.stringify(newData));
    return newData;
  }
}
