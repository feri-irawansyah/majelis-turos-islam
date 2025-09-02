import { AddArticle, Article } from "../models/article-model";

export class ArticleService {
  constructor(private db: D1Database, private kv: KVNamespace) {}

  async getArticlesData(
    page = 1,
    pageSize = 10
  ): Promise<{ rows: Article[]; total: number; totalPages: number }> {
    const offset = (page - 1) * pageSize;

    // ambil data
    const stmt = this.db.prepare(
      "SELECT * FROM articles ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    const result = await stmt.bind(pageSize, offset).all<Article>();
    if (result.error) throw new Error(result.error);

    // hitung total rows
    const countStmt = this.db.prepare("SELECT COUNT(*) as count FROM articles");
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

  async createArticle(request: AddArticle): Promise<void> {
    console.log(request);
    const stmt = this.db.prepare(
      "INSERT INTO articles (title, markdown, slug, author_id, created_at) VALUES (?, ?, ?, ?, datetime('now'))"
    );
    await stmt
      .bind(request.title, request.markdown, request.slug, request.author_id)
      .run()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        throw new Error(err);
      });
  }
}
