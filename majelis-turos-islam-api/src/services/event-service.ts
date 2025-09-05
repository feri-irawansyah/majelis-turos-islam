import { AddEvent, AddKajian, Event, UpdateEvent } from "../models/event-model";

export class EventService {
  constructor(private db: D1Database, private kv: KVNamespace) {}

  async getEventsData(
    page = 1,
    pageSize = 10
  ): Promise<{ rows: Event[]; total: number; totalPages: number }> {
    const offset = (page - 1) * pageSize;

    // ambil data
    const stmt = this.db.prepare(
      "SELECT * FROM events ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    const result = await stmt.bind(pageSize, offset).all<Event>();
    if (result.error) throw new Error(result.error);

    // hitung total rows
    const countStmt = this.db.prepare("SELECT COUNT(*) as count FROM events");
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

  async createEvent(request: AddEvent): Promise<void> {
    console.log(request);
    const stmt = this.db.prepare(
      "INSERT INTO events (title, markdown, slug, created_at) VALUES (?, ?, ?, datetime('now'))"
    );
    await stmt
      .bind(request.title, request.markdown, request.slug)
      .run()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        throw new Error(err);
      });
  }

  async updateEvent(request: UpdateEvent, id: number): Promise<void> {
    if (!id) {
      throw new Error("ID is required");
    }
    const stmt = this.db.prepare(
      "UPDATE events SET title = ?, markdown = ?, slug = ?, created_at = datetime('now') WHERE event_id = ?"
    );
    await stmt
      .bind(request.title, request.markdown, request.slug, id)
      .run()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        throw new Error(err);
      });
  }

  async getKajianData(
    page = 1,
    pageSize = 10
  ): Promise<{ rows: Event[]; total: number; totalPages: number }> {
    const offset = (page - 1) * pageSize;

    // ambil data
    const stmt = this.db.prepare(
      "SELECT * FROM kajian ORDER BY created_at DESC LIMIT ? OFFSET ?"
    );
    const result = await stmt.bind(pageSize, offset).all<Event>();
    if (result.error) throw new Error(result.error);

    // hitung total rows
    const countStmt = this.db.prepare("SELECT COUNT(*) as count FROM kajian");
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

  async createKajian(request: AddKajian): Promise<void> {
    console.log(request);
    const stmt = this.db.prepare(
      "INSERT INTO kajian (title, yt_link, created_at) VALUES (?, ?, datetime('now'))"
    );
    await stmt
      .bind(request.title, request.yt_link)
      .run()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        throw new Error(err);
      });
  }
}
