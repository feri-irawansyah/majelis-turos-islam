import { Hono } from "hono";
import {
  createNews,
  createNewsArticle,
  getNewsArticle,
  getNewsData,
  getSingleNews,
  updateNews,
} from "./controllers/news-controller";
import { cors } from "hono/cors";
import {
  createArticle,
  getArticlesData,
} from "./controllers/article-controller";
import {
  createEvent,
  createKajian,
  getEventsData,
  getKajianData,
  updateEvent,
} from "./controllers/event-controller";

type Bindings = {
  DB: D1Database;
};

const app = new Hono<{ Bindings: Bindings }>();

app.use(
  "*",
  cors({
    origin: "*", // bisa dibatasi misalnya "https://example.com"
    allowMethods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
    allowHeaders: ["Content-Type", "Authorization"],
  })
);

const routes = (app: Hono<{ Bindings: Bindings }>) => {
  app.get("/", (c) => {
    return c.json({ message: "Assalamualaikum" });
  });
  app.get("/news/data", getNewsData);
  app.post("/news/create", createNews);
  app.post("/news/update/:id", updateNews);
  app.post("/news/delete/:id", updateNews);

  // single news
  app.get("/news/:id", getSingleNews);

  // news article
  app.post("/news/article/create/:slug", createNewsArticle);
  app.get("/news/article/get/:slug", getNewsArticle);

  // articles
  app.get("/article/data", getArticlesData);
  app.post("/article/create", createArticle);

  // events
  app.get("/event/data", getEventsData);
  app.post("/event/create", createEvent);
  app.post("/event/update/:id", updateEvent);

  // kajian
  app.get("/kajian/data", getKajianData);
  app.post("/kajian/create", createKajian);

  app.notFound((c) => c.json({ error: "Not Found" }, 404));
};

routes(app);

export default app;
