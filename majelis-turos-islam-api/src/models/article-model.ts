export type Article = {
  article_id: number;
  slug: string;
  title: string;
  markdown: string;
  author_id: number;
  created_at: string;
};

export type AddArticle = {
  slug: string;
  title: string;
  markdown: string;
  author_id: number;
};
