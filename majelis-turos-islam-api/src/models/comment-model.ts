export type Comments = {
  comment_id: number;
  title: string;
  stars: number;
  email: string;
  name: string;
  created_at: string;
};

export type AddComment = {
  title: string;
  stars: number;
  email: string;
  name: string;
};
