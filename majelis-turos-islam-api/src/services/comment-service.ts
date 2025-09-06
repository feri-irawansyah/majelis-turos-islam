import { AddComment, Comments } from "../models/comment-model";

export class CommentService {
  constructor(private db: D1Database, private kv: KVNamespace) {}

  async getCommentsData(
    page = 1,
    pageSize = 10
  ): Promise<{ rows: Comments[]; total: number; totalPages: number }> {
    const offset = (page - 1) * pageSize;

    // ambil data
    const stmt = this.db.prepare(
      "SELECT * FROM comments ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    const result = await stmt.bind(pageSize, offset).all<Comments>();
    if (result.error) throw new Error(result.error);

    // hitung total rows
    const countStmt = this.db.prepare("SELECT COUNT(*) as count FROM comments");
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

  async createComment(request: AddComment): Promise<void> {
    console.log(request);
    const stmt = this.db.prepare(
      "INSERT INTO comments (title, stars, email, name, created_at) VALUES (?, ?, ?, ?, datetime('now'))"
    );
    await stmt
      .bind(request.title, request.stars, request.email, request.name)
      .run()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        throw new Error(err);
      });
  }
}
