import { Context } from "hono";
import { getConnection } from "../middleware/connection";
import { CommentService } from "../services/comment-service";
import { AddComment } from "../models/comment-model";

// Controller get all articles
export const getCommentsData = async (c: Context) => {
  const pool = getConnection(c);
  const page = parseInt(c.req.query("page") || "1");
  const pageSize = parseInt(c.req.query("pageSize") || "10");
  const service = new CommentService(pool, c.env.KV);
  try {
    const data = await service.getCommentsData(page, pageSize);
    return c.json(data);
  } catch (err) {
    console.error(err);
    return c.json({ error: "Internal Server Error" }, 500);
  }
};

// Controller create article
export const createComment = async (c: Context) => {
  try {
    const request = (await c.req.json()) as AddComment;
    console.log(request);
    const pool = getConnection(c);
    const service = new CommentService(pool, c.env.KV);
    await service.createComment(request);
    return c.json({ message: "Article created successfully" });
  } catch (err: any) {
    return c.json({ error: `Error in createComment: ${err.message}` }, 500);
  }
};
