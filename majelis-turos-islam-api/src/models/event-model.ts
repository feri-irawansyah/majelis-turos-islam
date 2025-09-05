export type Event = {
  event_id: number;
  slug: string;
  title: string;
  markdown: string;
  created_at: string;
};

export type AddEvent = {
  slug: string;
  title: string;
  markdown: string;
};

export type UpdateEvent = {
  slug: string;
  title: string;
  markdown: string;
};

export type AddKajian = {
  title: string;
  yt_link: string;
};
